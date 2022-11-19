# Advent of Code 2017

[![Rust](https://github.com/Aidiakapi/advent_of_code_2017/actions/workflows/rust.yml/badge.svg)](https://github.com/Aidiakapi/advent_of_code_2017/actions/workflows/rust.yml)

My solutions for Advent of Code 2017. Written in Rust 🦀.

- Clone the repository.
- Make sure you have a nightly version of Rust (written around December 2022).
- `cargo run --release` for all days, `cargo run --release -- N` for a specific
  day.
- Want your own inputs?
    - **Auto-download:** Delete the `inputs` directory, then create a
      `session_key.txt` file containing your AoC website's session cookie value.
    - **Manually:** Replace the contents of a `inputs/XX.txt` file with your
      desired input.
- Benchmarks? 🚤
    - `cargo bench --features "criterion"`
    - optionally add `-- dayN` at the end, to run a specific day!
