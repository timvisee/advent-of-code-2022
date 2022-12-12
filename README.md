# Advent of Code 2022 in Rust

My [Advent of Code 2022][aoc-2022] solutions in the Rust programming language.
This repository holds a separate Rust project for each day and part.

I attempt to develop a standalone, elegant, compact and fast solution for each
problem (two each day).

Previous year I did the same, solving everything in under a second:

- https://timvisee.com/blog/solving-aoc-2020-in-under-a-second/
- https://github.com/timvisee/advent-of-code-2021
- https://github.com/timvisee/advent-of-code-2020

## Timings

Here is how long each solution runs. All solutions are measured (non
scientifically) in [`bench.rs`](./runner/src/bin/bench.rs) on an
`AMD Ryzen 9 5900X (24) @ 3.7GHz` machine running Linux.

|                                                | part A                              | part B                              |
|:-----------------------------------------------|:------------------------------------|:------------------------------------|
| [day 1](https://adventofcode.com/2022/day/1)   | [` 0.027 ms`](./day01a/src/main.rs) | [` 0.031 ms`](./day01b/src/main.rs) |
| [day 2](https://adventofcode.com/2022/day/2)   | [` 0.006 ms`](./day02a/src/main.rs) | [` 0.006 ms`](./day02b/src/main.rs) |
| [day 3](https://adventofcode.com/2022/day/3)   | [` 0.015 ms`](./day03a/src/main.rs) | [` 0.016 ms`](./day03b/src/main.rs) |
| [day 4](https://adventofcode.com/2022/day/4)   | [` 0.040 ms`](./day04a/src/main.rs) | [` 0.039 ms`](./day04b/src/main.rs) |
| [day 5](https://adventofcode.com/2022/day/5)   | [` 0.020 ms`](./day05a/src/main.rs) | [` 0.019 ms`](./day05b/src/main.rs) |
| [day 6](https://adventofcode.com/2022/day/6)   | [` 0.001 ms`](./day06a/src/main.rs) | [`   720 ns`](./day06b/src/main.rs) |
| [day 7](https://adventofcode.com/2022/day/7)   | [` 0.008 ms`](./day07a/src/main.rs) | [` 0.011 ms`](./day07b/src/main.rs) |
| [day 8](https://adventofcode.com/2022/day/8)   | [` 0.045 ms`](./day08a/src/main.rs) | [` 0.177 ms`](./day08b/src/main.rs) |
| [day 9](https://adventofcode.com/2022/day/9)   | [` 0.120 ms`](./day09a/src/main.rs) | [` 0.290 ms`](./day09b/src/main.rs) |
| [day 10](https://adventofcode.com/2022/day/10)  | [` 0.001 ms`](./day10a/src/main.rs) | [` 0.003 ms`](./day10b/src/main.rs) |
| [day 11](https://adventofcode.com/2022/day/11)  | [` 0.019 ms`](./day11a/src/main.rs) | [` 5.72  ms`](./day11b/src/main.rs) |
| [day 12](https://adventofcode.com/2022/day/12)  | [` 0.294 ms`](./day12a/src/main.rs) | [`51.22  ms`](./day12b/src/main.rs) |

|              | one-by-one (1 CPU core)                  | parallel                                     |
|:-------------|:-----------------------------------------|:---------------------------------------------|
| _everything_ | [`58.55 ms`](./runner/src/bin/runner.rs) | [`51.86 ms`](./runner/src/bin/runner-par.rs) |

## Run solutions

Each Rust project contains a `input.txt` file, holding the puzzle input. Simply
run the project to see the solution appear.

```bash
# Switch to day 1a, and run it
cd day01a
cargo +nightly run --release

# or run everything in parallel
cd ../runner
cargo +nightly run --release --bin runner-par

# or benchmark every day
cd ../runner
cargo +nightly run --release --bin bench
```

Some solutions require Rust Nightly, that's why `+nightly` is included.

## Other years

- [2022](https://github.com/timvisee/advent-of-code-2022) _(current)_
- [2021](https://github.com/timvisee/advent-of-code-2021)
- [2020](https://github.com/timvisee/advent-of-code-2020)
- [2019](https://github.com/timvisee/advent-of-code-2019)
- [2018](https://github.com/timvisee/advent-of-code-2018)
- [2017](https://github.com/timvisee/advent-of-code-2017)

## License

This project is released under the GNU GPL-3.0 license.
Check out the [LICENSE](LICENSE) file for more information.

[aoc-2022]: https://adventofcode.com/2022
