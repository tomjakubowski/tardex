# tardex

A Rust crate to access a file's contents and metadata in a tarball using its
path.  It's a "tarball index".

## Why?

The [`tar`](https://lib.rs/crates/tar) crate provides an extensive API but
randomly accessing tarball entries by path is slightly awkward, because it:

  * requires mutable access to the `Archive`.
  * requires accessing the entries in order each time, not randomly.

Tardex is just a less featureful alternative interface tuned for a particular
use case, built atop it.

## Anticipated features

* [ ] Support hard and symbolic link tarball entries
* [ ] Support directory tarball entries

## License

Tardex is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Tardex by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
