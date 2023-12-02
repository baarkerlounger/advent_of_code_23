use std::env;
use std::fs;
use std::collections::{HashMap, HashSet};

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
    let lines = input.trim().split("\n");

    match part {
        Part::One => {
            let bag = HashMap::from([
                ("red", 12),
                ("green", 13),
                ("blue", 14),
            ]);
            let mut possible = HashSet::new();

            for line in lines {
                let split = line.split(":").collect::<Vec<&str>>();
                let id = split[0]
                    .split_whitespace()
                    .collect::<Vec<&str>>()[1]
                    .parse::<u32>()
                    .unwrap();
                possible.insert(id);

                let rounds: Vec<&str> = split[1].split(";").collect();
                for round in rounds {
                    let die: Vec<&str> = round.split(", ").collect();
                    for dice in die {
                        let dice_split: Vec<&str> = dice.split_whitespace().collect();
                        let actual = dice_split[0].parse::<u32>().unwrap();
                        let max = bag.get(dice_split[1]).unwrap();
                        if  actual > *max {
                            possible.remove(&id);
                        }
                    }
                }
            }
            possible.iter().sum()
        },
        Part::Two => {
            let mut powers: Vec<u32> = Vec::new();

            for line in lines {
                let mut bag = HashMap::from([
                    ("red", 0),
                    ("green", 0),
                    ("blue", 0),
                ]);

                let split = line.split(":").collect::<Vec<&str>>();

                let rounds: Vec<&str> = split[1].split(";").collect();
                for round in rounds {
                    let die: Vec<&str> = round.split(", ").collect();
                    for dice in die {
                        let dice_split: Vec<&str> = dice.split_whitespace().collect();
                        let actual = dice_split[0].parse::<u32>().unwrap();
                        let max = bag.get_mut(dice_split[1]).unwrap();
                        if  actual > *max {
                            *max = actual;
                        }
                    }
                }
                powers.push(bag.values().product());
            }
            powers.iter().sum()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::One), 8);
    }

    #[test]
    fn test_part_2() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::Two), 2286);
    }
}
