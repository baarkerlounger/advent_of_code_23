use aho_corasick::AhoCorasick;
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
    let lines = input.lines();

    match part {
        Part::One => {
            let sum = lines
                .map(|l| {
                    let digits = l
                        .chars()
                        .filter(|c| c.is_ascii_digit())
                        .collect::<Vec<char>>();
                    let first = digits[0];
                    let last = digits[digits.len() - 1];
                    format!("{}{}", first, last).parse::<u32>().unwrap()
                })
                .sum();
            sum
        }
        Part::Two => {
            let patterns = &[
                "1", "one", "2", "two", "3", "three", "4", "four", "5", "five", "6", "six", "7",
                "seven", "8", "eight", "9", "nine",
            ];

            let ac = AhoCorasick::new(patterns).unwrap();
            let sum = lines
                .map(|l| {
                    let digits = ac
                        .find_overlapping_iter(l)
                        .map(|m| match_to_digit(patterns[m.pattern()]))
                        .collect::<Vec<_>>();
                    let first = digits[0];
                    let last = digits[digits.len() - 1];
                    format!("{}{}", first, last).parse::<u32>().unwrap()
                })
                .sum();
            sum
        }
    }
}

fn match_to_digit(str: &str) -> u32 {
    if str.len() > 1 {
        match str {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => panic!(),
        }
    } else {
        str.chars().next().unwrap().to_digit(10).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::One), 142);
    }

    #[test]
    fn test_part_2() {
        let file_contents = fs::read_to_string("data/demo_input_2.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::Two), 281);
    }
}
