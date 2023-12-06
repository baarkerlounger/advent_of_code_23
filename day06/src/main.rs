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

enum Part {
    One,
    Two,
}

fn result(input: &str, part: Part) -> u64 {
    let lines = match part {
        Part::One => input
            .lines()
            .map(|l| {
                l.split(':')
                    .map(|s| s.trim())
                    .nth(1)
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
            })
            .collect::<Vec<Vec<u64>>>(),
        Part::Two => input
            .lines()
            .map(|l| {
                Vec::from([l
                    .split(':')
                    .map(|s| s.trim())
                    .nth(1)
                    .unwrap()
                    .replace(' ', "")
                    .parse::<u64>()
                    .unwrap()])
            })
            .collect::<Vec<Vec<u64>>>(),
    };

    let times = &lines[0];
    let dists = &lines[1];

    let total_win_strats: u64 = times
        .iter()
        .enumerate()
        .map(|(idx, time)| {
            let dist = dists[idx];
            (0_u64..=*time)
                .map(|hold| ((hold * (time - hold)) > dist) as u64)
                .sum::<u64>()
        })
        .product();

    total_win_strats
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::One), 288);
    }

    #[test]
    fn test_part_2() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::Two), 71503);
    }
}
