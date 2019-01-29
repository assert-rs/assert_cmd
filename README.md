# assert_cmd

> **Assert `process::Command`** - Easy command initialization and assertions.

[![Travis Status](https://travis-ci.org/assert-rs/assert_cmd.svg?branch=master)](https://travis-ci.org/assert-rs/assert_cmd)
[![Appveyor Status](https://ci.appveyor.com/api/projects/status/i1e8vpebw3hu0afg/branch/master?svg=true)](https://ci.appveyor.com/project/epage/assert-cmd/branch/master)
[![Documentation](https://img.shields.io/badge/docs-master-blue.svg)][Documentation]
![License](https://img.shields.io/crates/l/assert_cmd.svg)
[![Crates Status](https://img.shields.io/crates/v/assert_cmd.svg)](https://crates.io/crates/assert_cmd)

## Install

Add to your `Cargo.toml`:

```toml
[dependencies]
assert_cmd = "0.11"
```

## Example

Here's a trivial example:

```rust,no_run
extern crate assert_cmd;

use std::process::Command;
use assert_cmd::prelude::*;

Command::cargo_bin("bin_fixture")
    .unwrap()
    .assert()
    .success();
```

## Relevant crates

Other crates that might be useful in testing command line programs.
* [duct][duct] for orchestrating multiple processes.
* [rexpect][rexpect] for testing interactive programs.
* [`assert_fs`][assert_fs] for filesystem fixtures and assertions.
* [dir-diff][dir-diff] for testing file side-effects.
* [tempfile][tempfile] for scratchpad directories.

[rexpect]: https://crates.io/crates/rexpect
[dir-diff]: https://crates.io/crates/dir-diff
[tempfile]: https://crates.io/crates/tempfile
[duct]: https://crates.io/crates/duct
[assert_fs]: https://crates.io/crates/assert_fs

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[Crates.io]: https://crates.io/crates/assert_cmd
[Documentation]: https://docs.rs/assert_cmd
