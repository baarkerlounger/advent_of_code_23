# 🎄 Advent of Code 2023

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

Generate project: `cargo make day {##}`

<!--- advent_readme_stars table --->
## 2023 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2023/day/1) | ⭐ | ⭐ |
| [Day 2](https://adventofcode.com/2023/day/2) | ⭐ | ⭐ |
| [Day 2](https://adventofcode.com/2023/day/3) | ⭐ | ⭐ |
<!--- advent_readme_stars table --->

<!--- benchmarking table --->
## Benchmarks

| Day | Part 1 | Part 2 | Notes |
| :---: | :---: | :---:  | :---: |
| [Day 1](./day01/src/main.rs) | `1.5ms` | `1.4ms` | Aho-corasick |
| [Day 2](./day02/src/main.rs) | `1.5ms` | `1.4ms` ||
| [Day 3](./day03/src/main.rs) | `3.0ms` | `2.0ms` ||

**Total: 10.8ms**
<!--- benchmarking table --->

Benchmarks run using:
 ```bash
 hyperfine --runs 100  -N  "target/release/day{##} 1"
 hyperfine --runs 100  -N  "target/release/day{##} 2"
 ```
