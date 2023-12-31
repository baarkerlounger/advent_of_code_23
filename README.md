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
| [Day 5](https://adventofcode.com/2023/day/5) | ⭐ | ⭐ |
| [Day 6](https://adventofcode.com/2023/day/6) | ⭐ | ⭐ |
| [Day 7](https://adventofcode.com/2023/day/7) | ⭐ | ⭐ |
| [Day 8](https://adventofcode.com/2023/day/8) | ⭐ | ⭐ |
<!--- advent_readme_stars table --->

<!--- benchmarking table --->
## Benchmarks

| Day | Part 1 | Part 2 | Notes |
| :---: | :---: | :---:  | :---: |
| [Day 1](./day01/src/main.rs) | `0.9ms` | `1.3ms` | Aho-corasick |
| [Day 2](./day02/src/main.rs) | `1.2ms` | `1.0ms` ||
| [Day 3](./day03/src/main.rs) | `2.4ms` | `1.5ms` ||
| [Day 4](./day04/src/main.rs) | `1.2ms` | `1.2ms` ||
| [Day 5](./day05/src/main.rs) | `0.8ms` | `57259ms` |Rayon to brute force|
| [Day 6](./day06/src/main.rs) | `0.7ms` | `28.5ms` ||
| [Day 7](./day07/src/main.rs) | `1.7ms` | `1.7ms` ||
| [Day 8](./day08/src/main.rs) | `2.1ms` | `6.6ms` |LCM|

<!--- benchmarking table --->

Benchmarks run on `Intel i7-10510U` using:
 ```bash
 hyperfine --warmup 50 --runs 100 -N  "target/release/day{##} 1"
 hyperfine --warmup 50 --runs 100 -N  "target/release/day{##} 2"
 ```
Warm up to reduce impact of disk I/O since that's just loading the input file - which takes around 0.8ms even with warm caches.
