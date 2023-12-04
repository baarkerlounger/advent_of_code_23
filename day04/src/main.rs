use std::env;
use std::fs;
use std::collections::HashMap;

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
    match part {
        Part::One => {
            input.lines().map(|line| {
                let nums: Vec<&str> = line.split(": ").collect::<Vec<_>>()[1].split(" | ").collect();
                let mut winning: Vec<u32> = nums[0].split_whitespace().map(|s| s.parse().unwrap()).collect();
                let actual: Vec<u32> = nums[1].split_whitespace().map(|s| s.parse().unwrap()).collect();
                winning.retain(|&n| actual.contains(&n));
                let count = winning.len();
                if count > 1 {
                    2_u32.pow((count - 1) as u32)
                } else {
                    count as u32
                }
            }).sum()
        }
        Part::Two => {
            let lines = input.lines();
            let mut map: HashMap<usize, u32> = HashMap::new();
            for (line_num, line) in lines.enumerate() {
                let nums: Vec<&str> = line.split(": ").collect::<Vec<_>>()[1].split(" | ").collect();
                let mut winning: Vec<u32> = nums[0].split_whitespace().map(|s| s.parse().unwrap()).collect();
                let actual: Vec<u32> = nums[1].split_whitespace().map(|s| s.parse().unwrap()).collect();
                winning.retain(|&n| actual.contains(&n));
                let original_score = winning.len();
                let card_num = line_num + 1;
                let self_copies = if map.contains_key(&card_num) {
                    *map.get(&card_num).unwrap()
                } else {
                    map.insert(card_num, 1);
                    1
                };
                for _ in 1..=self_copies {
                    if original_score > 0 {
                        for card in (card_num + 1)..=(card_num + original_score) {
                            let copies = map.get(&card).unwrap_or(&1) + 1;
                            map.insert(card, copies);
                        }
                    }
                }
            }
            map.values().sum()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::One), 13);
    }

    #[test]
    fn test_part_2() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::Two), 30);
    }
}
