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
