# DISCLAIMER
USED FOR GETTING A SENSE OF SCALE AND TO VALIDATE IMPROVED ATTEMPTS DO NOT PROVIDE INCORRECT RESULTS.

Additionally, see the [Benchmarking section of the Readme](readme.md#0---benchmarking-not-so-fun-yet) for more details.

- Benchmark is currently being run on an AMD Ryzen 1700 (non-X), with 32GB of RAM.
- Benchmark is running on `rustc` version 1.76.0
- Benchmarks are currently run on an input of 100,000. So all primes less than 100 thousand.
- Each update will be the full 'suite' update, rather than just patching in new numbers as I go. That way they are all on the same conditions.

# Benchmarks

- Function(N), where N = 100_000
- Expected Result Count = 9_592 primes
- Known "Junk" Tuples = 1_272 empty tuples. See [Initial Capacity of the Readme](readme.md#1---initial-capacity-allocation--prime-counting) for more details.

| Function           | Elapsed Time | Relative Performance | Note                                                        |
|--------------------|--------------|----------------------|-------------------------------------------------------------|
| dijkstra_naive     | 5.35s        | 1x                   |                                                             |
| dijkstra_attempt_1 | 79.86ms      | 66.9x                |                                                             |
| dijkstra_attempt_2 | 34.25ms      | 156.2x               | Increased tuple storage from (u64, u64) to (u64, u64, u64). |
| dijkstra_attempt_3 |              |                      |                                                             |

r 111111
x 111
b 11
c 1111