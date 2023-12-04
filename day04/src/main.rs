use std::env;
use std::fs;
use std::collections::HashSet;

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
                let winning: HashSet<u32> = nums[0].split_whitespace().map(|s| s.parse().unwrap()).collect();
                let actual: HashSet<u32> = nums[1].split_whitespace().map(|s| s.parse().unwrap()).collect();
                let count = winning.intersection(&actual).count() as u32;
                if count > 1 {
                    2_u32.pow(count - 1)
                } else {
                    count
                }
            }).sum()
        }
        Part::Two => {
            let lines = input.lines();
            let mut copies_list: Vec<u32> = vec![1; lines.clone().count()];
            for (line_num, line) in lines.enumerate() {
                let nums: Vec<&str> = line.split(": ").collect::<Vec<_>>()[1].split(" | ").collect();
                let winning: HashSet<u32> = nums[0].split_whitespace().map(|s| s.parse().unwrap()).collect();
                let actual: HashSet<u32> = nums[1].split_whitespace().map(|s| s.parse().unwrap()).collect();
                let original_score = winning.intersection(&actual).count();
                let self_copies = copies_list[line_num];
                if original_score > 0 {
                    for card in (line_num + 1)..=(line_num + original_score) {
                        copies_list[card] = copies_list[card] + self_copies;
                    }
                }
            }
            copies_list.iter().sum()
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
