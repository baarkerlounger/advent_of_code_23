use itertools::Itertools;
use std::env;
use std::fs;

fn main() {
    let file_contents = fs::read_to_string("data/input.txt").expect("Valid file");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let part: u64 = args[1].parse().unwrap();

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

#[derive(PartialEq)]
enum Part {
    One,
    Two,
}

struct Map {
    pub src_rng: Rng,
    offset: i64,
}

struct Rng {
    pub start: u64,
    pub end: u64,
}

fn result(input: &str, part: Part) -> u64 {
    let mut input_groups = input.split("\n\n");
    let seed_line = input_groups.next().unwrap().split(": ").collect::<Vec<_>>()[1];
    let seed_nums = seed_line
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap());

    let map_groups: Vec<Vec<Map>> = input_groups
        .map(|group| {
            let split_1: Vec<&str> = group.split(":").collect();
            let num_lines = split_1[1].lines().filter(|l| !l.is_empty());
            let map_group: Vec<Map> = num_lines
                .map(|l| {
                    let nums: Vec<u64> = l.split_whitespace().map(|e| e.parse().unwrap()).collect();
                    let src_rng = Rng {
                        start: nums[1],
                        end: nums[1] + nums[2],
                    };
                    let offset = nums[0] as i64 - nums[1] as i64;
                    Map { src_rng, offset }
                })
                .collect();
            map_group
        })
        .collect();

    let mut location_min = u64::MAX;
    match part {
        Part::One => {
            let seeds: Vec<u64> = seed_nums.collect();
            for seed in seeds {
                location_min = min_location(seed, &map_groups, &mut location_min);
            }
            location_min
        }
        Part::Two => {
            for (start, size) in seed_nums.tuples() {
                let end = start + size;
                for seed in start..end {
                    location_min = min_location(seed, &map_groups, &mut location_min);
                }
            }
            location_min
        }
    }
}

fn min_location(seed: u64, map_groups: &Vec<Vec<Map>>, min: &mut u64) -> u64 {
    let mut loc = seed;
    for map_group in map_groups {
        for map in map_group {
            if loc >= map.src_rng.start && loc < map.src_rng.end {
                loc = (loc as i64 + map.offset) as u64;
                break;
            }
        }
    }
    if loc < *min {
        *min = loc;
    }
    *min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::One), 35);
    }

    #[test]
    fn test_part_2() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::Two), 46);
    }
}
