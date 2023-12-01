# 🎄 Advent of Code 2023

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

Generate project: `cargo make day {##}`

<!--- advent_readme_stars table --->
## 2023 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2023/day/1) | ⭐ | ⭐ |
<!--- advent_readme_stars table --->

<!--- benchmarking table --->
## Benchmarks

| Day | Part 1 | Part 2 | Notes |
| :---: | :---: | :---:  | :---: |
| [Day 1](./day01/src/main.rs) | `1.5ms` | `1.4ms` | Aho-corasick |

**Total: 2.9ms**
<!--- benchmarking table --->

Benchmarks run using:
 ```bash
 hyperfine --runs 100  -N  "target/release/day{##} 1"
 hyperfine --runs 100  -N  "target/release/day{##} 2"
 ```
