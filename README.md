# `measures-rs`

[![Crates.io](https://img.shields.io/crates/v/measures-rs.svg)](https://crates.io/crates/measures-rs)
[![Docs.rs](https://docs.rs/measures-rs/badge.svg)](https://docs.rs/measures-rs)
[![CI](https://github.com/carlomilanesi/measures-rs/workflows/Continuous%20Integration/badge.svg)](https://github.com/carlomilanesi/measures-rs/actions)
[![Coverage Status](https://coveralls.io/repos/github/carlomilanesi/measures-rs/badge.svg?branch=master)](https://coveralls.io/github/carlomilanesi/measures-rs?branch=master)

## Description

This repository contains the source code and the documentation of the library contained in the Rust-language crate: `measures`.

Its purpose is to improve the readability and correctness of applications using numeric values having units of measurement.
These can be the ones used in physics or in geometry, but also the ones commonly used in industry.

This purpose is achieved by encapsulating such numbers into objects whose type represents their unit of measurement, and providing for such types only the operations which make sense.

The documentation is in these files:
* [**Motivation**](docs/Motivation.md): It describes the advantages of using this library instead of other libraries or naked numbers.
* [**Tutorial**](docs/Tutorial.md): It is a step-by-step course on the use of this library.
* [**Architecture**](docs/Architecture.md): It explains the design choices of the library.

There are several examples:
* [**`full`**](examples/full.rs): Print of all the provided examples of units of measurement, and of the results of any supported operation.
* [**`bench`**](examples/examples/bench.rs), [**`bench2`**](measures-rs/examples/bench2.rs): Simple check of speed, compared with naked numbers or with the crate `nalgebra`.
* [**`base`**](examples/examples/base.rs), [**`mks`**](examples/examples/mks.rs), [**`si`**](examples/examples/si.rs), [**`unit`**](examples/examples/unit.rs): Translation for Measures of the examples included in the crate `uom` version 0.35.0.
* [**`nbody-measures`**](examples/examples/nbody-measures.rs): It is put beside example [**`nbody-naked`**](examples/examples/nbody-naked.rs). The latter is a benchmark program not using any external libraries, nor units of measurement, taken from *The Computer Language Benchmarks Game*. The former is a translation to a version using this library. At the beginning of both files, it is explained how to use them. In a specific setting, it appears that `nbody-measures` takes 11% more time than `nbody-naked`.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
