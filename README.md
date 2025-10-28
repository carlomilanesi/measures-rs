# `measures-rs`

[![Crates.io](https://img.shields.io/crates/v/measures-rs.svg)](https://crates.io/crates/measures-rs)
[![Docs.rs](https://docs.rs/measures-rs/badge.svg)](https://docs.rs/measures-rs)
[![CI](https://github.com/carlomilanesi/measures-rs/workflows/Continuous%20Integration/badge.svg)](https://github.com/carlomilanesi/measures-rs/actions)
[![Coverage Status](https://coveralls.io/repos/github/carlomilanesi/measures-rs/badge.svg?branch=master)](https://coveralls.io/github/carlomilanesi/measures-rs?branch=master)

## Description

This repository contains the source code and the documentation of the library contained in the Rust-language crate: `measures`.

Its purpose is to improve the readability and correctness of Rust applications that use numeric values having units of measurement.
Such units can be the ones used in physics or in geometry, but also the ones commonly used in industry.

This purpose is achieved by encapsulating such numbers into objects whose type represents their unit of measurement, and providing for such types only the operations which make sense.

The documentation is in these files:
* [**Motivation**](docs/Motivation.md): It describes the advantages of using this library instead of other libraries or naked numbers.
* [**Tutorial**](docs/Tutorial.md): It is a step-by-step course on the use of this library.
* [**Architecture**](docs/Architecture.md): It explains the design choices of the library.

There are several examples.
At the beginning of any source files, there are instructions about how to run them:
* [**`base`**](examples/base.rs), [**`mks`**](examples/mks.rs), [**`si`**](examples/si.rs), [**`user_defined_unit`**](examples/user_defined_unit.rs): Translation for use with the library Measures of the examples included in the crate `uom` version 0.35.0.
* [**`bench`**](examples/bench.rs): Simple check of speed, comparing the performance using primitive numbers, exact measures, and approximate measures.
* [**`full`**](examples/full.rs): It prints all the provided examples of units of measurement, and the results of any supported operation.
* [**`matrix-mul`**](examples/matrix-mul.rs): It computes the performance of matrix multiplication, using several libraries and algorithms. The library used are `nalgebra`, `faer`, `ndarray`, and `measures`.
* [**`nbody-naked`**](examples/nbody-naked.rs), [**`nbody-measures`**](examples/nbody-measures.rs), [**`nbody-approx-measures`**](examples/nbody-approx-measures.rs): Benchmark simulating the movements of four planets around the sun, taken from *The Computer Language Benchmarks Game*. The first program is the original benchmark, not using any external libraries, nor units of measurement. The second one is a translation of that code to a version using the library Measures, with exact values. The third one is a version using the library Measures, with approximate values and propagation of uncertainties.
* [**`air-mass-naked`**](examples/air-mass-naked.rs) and [**`air-mass-measures`**](examples/air-mass-measures.rs): Program taken from the Rosetta Code website. In particular, the first one is the original version, without units of measurement, taken from https://rosettacode.org/wiki/Air_mass#Rust. The second one is a version in which units of measurement are specified by using the library Measures.
* [**`haversine-naked`**](examples/haversine-naked.rs), [**`haversine-measures`**](examples/haversine-measures.rs): Program taken from the Rosetta Code website. In particular, the first one is the original version, without units of measurement, taken from https://rosettacode.org/wiki/Haversine_formula#Rust. The second one is a version in which units of measurement are specified by using the library Measures.
* [**`resistor-mesh-naked.cpp`**](examples/resistor-mesh-naked.cpp), [**`resistor-mesh-naked`**](examples/resistor-mesh-naked.rs), [**`resistor-mesh-measures`**](examples/resistor-mesh-measures.rs): Programs taken from the Rosetta Code website. In particular, the first one is the C++ version, taken from https://rosettacode.org/wiki/Resistor_mesh#C++. The second one is the Rust version, without units of measurement, taken from https://rosettacode.org/wiki/Resistor_mesh#Rust, with some fixes. The third one is a version in which units of measurement are specified by using the library Measures.

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
