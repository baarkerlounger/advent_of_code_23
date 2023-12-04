# 🎄 Advent of Code 2023

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

Generate project: `cargo make day {##}`

<!--- advent_readme_stars table --->
## 2023 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2023/day/1) | ⭐ | ⭐ |
| [Day 2](https://adventofcode.com/2023/day/2) | ⭐ | ⭐ |
| [Day 3](https://adventofcode.com/2023/day/3) | ⭐ | ⭐ |
| [Day 4](https://adventofcode.com/2023/day/4) | ⭐ | ⭐ |
<!--- advent_readme_stars table --->

<!--- benchmarking table --->
## Benchmarks

| Day | Part 1 | Part 2 | Notes |
| :---: | :---: | :---:  | :---: |
| [Day 1](./day01/src/main.rs) | `1.1ms` | `1.8ms` | Aho-corasick |
| [Day 2](./day02/src/main.rs) | `1.4ms` | `1.0ms` ||
| [Day 3](./day03/src/main.rs) | `2.5ms` | `2.0ms` ||
| [Day 4](./day04/src/main.rs) | `1.7ms` | `2.0ms` ||

**Total: 13.5ms**
<!--- benchmarking table --->

Benchmarks run on `Intel i7-10510U` using:
 ```bash
 hyperfine --warmup 50 --runs 100 -N  "target/release/day{##} 1"
 hyperfine --warmup 50 --runs 100 -N  "target/release/day{##} 2"
 ```
Warm up to reduce impact of disk I/O since that's just loading the input file - which takes around 0.8ms even with warm caches.
