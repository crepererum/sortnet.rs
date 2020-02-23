# Sortnet

[Sorting Networks](https://en.wikipedia.org/wiki/Sorting_network) for Rust.

[![Build Status](https://github.com/crepererum/sortnet.rs/workflows/CI/badge.svg)](https://github.com/crepererum/sortnet.rs/actions?query=workflow%3ACI)
[![Crates.io](https://img.shields.io/crates/v/sortnet.svg)](https://crates.io/crates/sortnet)
[![Documentation](https://docs.rs/sortnet/badge.svg)](https://docs.rs/sortnet/)
[![License](https://img.shields.io/crates/l/sortnet.svg)](#license)
[![Dependency Status](https://deps.rs/repo/github/crepererum/sortnet.rs/status.svg)](https://deps.rs/repo/github/crepererum/sortnet.rs)

## Current Implementation

| Input Size | Number of Comparisons |
| ---------: | --------------------: |
|          0 |                     0 |
|          1 |                     0 |
|          2 |                     1 |
|          3 |                     3 |
|          4 |                     5 |
|          5 |                     9 |
|          6 |                    12 |
|          7 |                    16 |
|          8 |                    19 |
|          9 |                    25 |
|         10 |                    29 |
|         11 |                    35 |
|         12 |                    39 |
|         13 |                    45 |
|         14 |                    51 |
|         15 |                    56 |
|         16 |                    60 |

## Prior Art
- [`sorting_networks` (Rust)](https://github.com/regexident/sorting_networks): Uses
  [Odd-even Mergesort](https://www.inf.hs-flensburg.de/lang/algorithmen/sortieren/networks/oemen.htm) but requires
  slightly more comparisons in some cases (e.g. 63 for 16 inputs). Also, `sortnet` uses compile-time unrolling of the
  whole algorithm which hopefully helps compiler and CPU
- [Sorting Networks](http://pages.ripco.net/~jgamble/nw.html): This is where the networks in `sortnet` are taken from.

## License

Licensed under either of these:

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

### Contributing

Unless you explicitly state otherwise, any contribution you intentionally submit for inclusion in the work, as defined
in the Apache-2.0 license, shall be dual-licensed as above, without any additional terms or conditions.
