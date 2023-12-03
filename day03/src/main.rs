use regex::{Match, Regex};
use std::cmp;
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let file_contents = fs::read_to_string("data/input.txt").expect("Valid file");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let part: u32 = args[1].parse().unwrap();

        match part {
            1 => println!("Result for part 1 is {}", result(&file_contents, Part::One)),
            2 => println!("Result for part 2 is {}", result(&file_contents, Part::Two)),
            _ => println!("Specify 1 or 2"),
        }
    } else {
        println!("Result for part 1 is {}", result(&file_contents, Part::One));
        println!("Result for part 2 is {}", result(&file_contents, Part::Two));
    }
}

enum Part {
    One,
    Two,
}

fn result(input: &str, part: Part) -> u32 {
    let grid: Vec<&str> = input.lines().collect();
    let num_regx = Regex::new(r"\d+").unwrap();

    match part {
        Part::One => {
            let mut part_nums: Vec<u32> = Vec::new();

            for (row_num, row) in grid.iter().enumerate() {
                for mat in num_regx.find_iter(row) {
                    let start = cmp::max(mat.start(), 1) - 1;
                    let end = cmp::min(mat.end(), grid[0].len() - 1) + 1;

                    if row_num > 0 && substring_contains_symbols(grid[row_num - 1], start, end) {
                        part_nums.push(mat.as_str().parse().unwrap());
                    }
                    if row_num < (grid.len() - 1)
                        && substring_contains_symbols(grid[row_num + 1], start, end)
                    {
                        part_nums.push(mat.as_str().parse().unwrap());
                    }
                    if substring_contains_symbols(grid[row_num], start, end) {
                        part_nums.push(mat.as_str().parse().unwrap());
                    }
                }
            }
            part_nums.iter().sum()
        }
        Part::Two => {
            let mut gear_ratios: Vec<u32> = Vec::new();
            let mut nums_map: HashMap<usize, Vec<Match>> = HashMap::new();
            for (row_num, row) in grid.iter().enumerate() {
                nums_map.insert(row_num, num_regx.find_iter(row).collect::<Vec<Match>>());
            }

            for (row_num, row) in grid.iter().enumerate() {
                for mat in row.match_indices("*") {
                    let mut gear_ratio_parts: Vec<u32> = Vec::new();
                    let match_idx = mat.0;

                    if row_num > 0 {
                        let mut adjacent_above = adjacent_nums(row_num - 1, &nums_map, match_idx);
                        gear_ratio_parts.append(&mut adjacent_above);
                    }
                    if row_num < (grid.len() - 1) {
                        let mut adjacent_below = adjacent_nums(row_num + 1, &nums_map, match_idx);
                        gear_ratio_parts.append(&mut adjacent_below);
                    }
                    gear_ratio_parts.append(&mut adjacent_nums(row_num, &nums_map, match_idx));

                    if gear_ratio_parts.len() == 2 {
                        gear_ratios.push(gear_ratio_parts.iter().product());
                    }
                }
            }
            gear_ratios.iter().sum()
        }
    }
}

fn substring_contains_symbols(row: &str, start: usize, end: usize) -> bool {
    let symbols = Vec::from(['*', '#', '$', '+', '-', '/', '=', '@', '%', '&']);
    let adjacent_chars = &row.chars().collect::<Vec<_>>()[start..end];
    adjacent_chars.iter().any(|c| symbols.contains(&c))
}

fn adjacent_nums(
    row_num: usize,
    nums_map: &HashMap<usize, Vec<Match>>,
    match_idx: usize,
) -> Vec<u32> {
    let nums = nums_map.get(&(row_num)).unwrap();
    nums.iter()
        .filter(|n| n.start() <= match_idx + 1 && n.end() >= match_idx)
        .map(|n| n.as_str().parse::<u32>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::One), 4361);
    }

    #[test]
    fn test_part_2() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::Two), 467835);
    }
}
