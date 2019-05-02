use std::{
    collections::BTreeMap,
    error::Error,
    fmt, io,
    path::{Path, PathBuf},
};
use tar;

/// Provides access to files in a tarball stored behind a Read impl.
pub struct Tardex<R>
where
    R: io::Read + io::Seek + Clone,
{
    dex: BTreeMap<PathBuf, Entry<R>>,
}

impl<R> Tardex<R>
where
    R: io::Read + io::Seek + Clone,
{
    pub fn new(reader: R) -> Result<Self> {
        let mut tar = tar::Archive::new(reader.clone());
        let mut dex = BTreeMap::new();
        for tar_entry in tar.entries()? {
            let tar_entry = tar_entry?;
            let header = tar_entry.header();
            let path = tar_entry.path()?.into_owned();
            let offset = tar_entry.raw_file_position();
            match header.entry_type() {
                tar::EntryType::Regular => (),
                _ => continue,
            }
            let len = tar_entry.header().entry_size()?;
            let entry = Entry::in_tarball(reader.clone(), offset, len)?;
            dex.insert(path, entry);
        }
        Ok(Tardex { dex })
    }

    /// Returns the tarball's paths in lexical order
    pub fn paths(&self) -> impl Iterator<Item = &Path> {
        self.dex.keys().map(|x| x.as_path())
    }

    /// Access the entry at a path.
    pub fn entry<'a, P>(&'a self, k: P) -> Option<Entry<R>>
    where
        P: AsRef<Path>,
    {
        self.dex.get(k.as_ref()).cloned()
    }
}

impl<R> fmt::Debug for Tardex<R>
where
    R: io::Read + io::Seek + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tardex")
    }
}

#[derive(Debug)]
pub enum TardexError {
    IoError(io::Error),
}
pub type Result<T> = std::result::Result<T, TardexError>;

impl fmt::Display for TardexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TardexError::IoError(err) => write!(f, "i/o error {}", err),
        }
    }
}
impl std::convert::From<io::Error> for TardexError {
    fn from(err: io::Error) -> TardexError {
        TardexError::IoError(err)
    }
}

impl Error for TardexError {}

/// An entry corresponds to a file in the tarball.
pub struct Entry<R> {
    read: std::io::Take<R>,
}

impl<R> Clone for Entry<R>
where
    R: io::Read + Clone,
{
    fn clone(&self) -> Self {
        let limit = self.read.limit();
        let inner = self.read.get_ref().clone();
        Entry {
            read: inner.take(limit),
        }
    }
}

impl<R> io::Read for Entry<R>
where
    R: io::Read + Clone,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.read.read(buf)
    }
}

impl<R> Entry<R>
where
    R: io::Read + io::Seek + Clone,
{
    fn in_tarball(tarball_reader: R, file_pos: u64, file_len: u64) -> Result<Entry<R>> {
        let mut entry_reader = tarball_reader.clone();
        entry_reader.seek(io::SeekFrom::Start(file_pos))?;
        Ok(Entry {
            read: entry_reader.take(file_len),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Metadata {
    mtime: u64,
    len: u64,
}

impl Metadata {
    pub fn mtime(&self) -> u64 {
        self.mtime
    }

    pub fn len(&self) -> u64 {
        self.len
    }
}

#[cfg(test)]
mod tests {
    use super::Tardex;
    use std::{
        io::{Cursor, Read},
        path::Path,
    };

    static TAR_FIXTURE: &'static [u8] =
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/fixture/fixture.tar"));

    #[test]
    fn test_paths() {
        let tardex = Tardex::new(Cursor::new(TAR_FIXTURE)).unwrap();
        let mut paths = tardex.paths();
        assert_eq!(Path::new("a.txt"), paths.next().unwrap());
        assert_eq!(Path::new("kida/a.txt"), paths.next().unwrap());
        assert_eq!(Path::new("kida/b.txt"), paths.next().unwrap());
        assert!(paths.next().is_none());
    }

    #[test]
    fn test_content() {
        let tardex = Tardex::new(Cursor::new(TAR_FIXTURE)).unwrap();
        let mut entry = tardex.entry("a.txt").unwrap();
        let mut a_contents = String::new();
        entry
            .read_to_string(&mut a_contents)
            .expect("read_to_string failed");
        assert_eq!(a_contents, "A is for Apple\n");

        entry = tardex.entry("kida/b.txt").unwrap();
        let mut kida_b_contents = String::new();
        entry
            .read_to_string(&mut kida_b_contents)
            .expect("read_to_string failed");
        assert_eq!(
            kida_b_contents,
            "Kid A In Alphabet Land Bashes Another Belligerent Beastie - The Bellicose Blot!\n"
        );
    }
}
