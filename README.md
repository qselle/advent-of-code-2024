[![Rust](https://github.com/qselle/advent-of-code-2024/actions/workflows/rust.yml/badge.svg)](https://github.com/qselle/advent-of-code-2024/actions/workflows/rust.yml) [![wakatime](https://wakatime.com/badge/github/qselle/advent-of-code-2024.svg)](https://wakatime.com/badge/github/qselle/advent-of-code-2024)
# Advent of code 2024 🎄🎅

This time in Rust to learn the language! Using https://github.com/gobanos/cargo-aoc 🦀.

Enter the competition: https://adventofcode.com/2024

## Build with `cargo-aoc`

Install [`cargo-aoc`](https://crates.io/crates/cargo-aoc) crate:

```console
advent-of-code-2024 $> cargo install cargo-aoc
```

Run:

```console
advent-of-code-2024 $> cargo aoc                                                                                 2 ↵
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
   Compiling aoc-autobuild v0.3.0 (/Users/quentin/Work/advent-of-code-2024/target/aoc/aoc-autobuild)
    Finished `release` profile [optimized] target(s) in 0.12s
     Running `target/release/aoc-autobuild`
AOC 2024
Day 1 - Part 1 : 1651298
	generator: 135.833µs,
	runner: 542ns

Day 1 - Part 2 : 21306195
	generator: 104.125µs,
	runner: 157.5µs
```

## Test

```console
advent-of-code-2024 $> cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running unittests src/lib.rs (target/debug/deps/advent_of_code_2024-027d060ec3f37a3f)

running 2 tests
test day1::tests::test_part2 ... ok
test day1::tests::test_part1 ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests advent_of_code_2024

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
