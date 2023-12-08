use std::collections::HashMap;
use std::env;
use std::fs;
use num_integer::lcm;

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

fn result(input: &str, part: Part) -> u64 {
    let mut lines = input.lines();
    let steps = lines.nth(0).unwrap().chars();
    let map_lines = lines.skip(1);
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut count = 0;

    for line in map_lines {
        let split: Vec<&str> = line.split(" = ").collect();
        let bind = split[1].replace(['(', ')'], "");
        let tup_vals: Vec<&str> = bind.split(", ").collect();
        map.insert(
            split[0].to_string(),
            (tup_vals[0].to_string(), tup_vals[1].to_string()),
        );
    }

    match part {
        Part::One => {
            let mut loc = String::from("AAA");
            let dest = String::from("ZZZ");
            for step in steps.cycle() {
                let tup = map.get(&loc).unwrap();
                loc = match step {
                    'L' => tup.0.clone(),
                    'R' => tup.1.clone(),
                    _ => panic!()
                };
                count += 1;
                if loc == dest {
                    break;
                }
            }
            count
        },
        Part::Two => {
            let locs: Vec<&String> = map.keys().filter(|k| k.ends_with('A')).collect();
            let mut counts: Vec<u64> = Vec::new();
            for loc in locs {
                let mut count = 0;
                let mut current_loc = loc;

                for step in steps.clone().cycle() {
                    let tup = map.get(current_loc).unwrap();
                    current_loc = match step {
                        'L' => &tup.0,
                        'R' => &tup.1,
                        _ => panic!()
                    };
                    count += 1;
                    if current_loc.ends_with('Z') {
                        counts.push(count);
                        break;
                    }
                }
            }
            counts.into_iter().reduce(lcm).unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::One), 6);
    }

    #[test]
    fn test_part_2() {
        let file_contents = fs::read_to_string("data/demo_input_2.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::Two), 6);
    }
}
