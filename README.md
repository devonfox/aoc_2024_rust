# Advent of Code 2024

Using Rust again this year.  Used it back in 2021 I believe!  

Using the following template: [aoc-template-rs](https://github.com/wjwh/aoc-template-rs)

Each day has a `solve()` function that returns a pair of `Solution`. The type `Solution` is an enum that can contain any integer or a string.

You can create a `Solution` by specifying its type, for example `Solution::U32(value)`, or by using the From trait which is implemented for all supported types, for example, `Solution::from(value)`.

To run: `cargo run --release [days...]`
