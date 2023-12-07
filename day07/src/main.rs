use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::str::Chars;

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

#[derive(PartialEq)]
enum Part {
    One,
    Two,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug, Eq, PartialEq)]
struct Round {
    hand: Hand,
    bid: u32,
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: HashMap<u32, u32>,
    hand_type: HandType,
    high_cards: Vec<u32>,
}

impl Hand {
    pub fn from_chars(chars: Chars, part: &Part, picture_cards: &HashMap<char, u32>) -> Self {
        let mut cards: HashMap<u32, u32> = HashMap::new();
        let mut high_cards: Vec<u32> = Vec::new();
        let mut hand_type: HandType;
        chars.for_each(|card| {
            let card_val = if card.is_ascii_digit() {
                card.to_digit(10).unwrap()
            } else {
                *picture_cards.get(&card).unwrap()
            };
            high_cards.push(card_val);

            if let Some(count) = cards.get_mut(&card_val) {
                *count += 1;
            } else {
                cards.insert(card_val, 1);
            }
        });

        let mut v: Vec<(&u32, &u32)> = cards.iter().map(|(k, v)| (v, k)).collect();
        v.sort_by(|a, b| b.cmp(a));
        if v[0].0 == &5 {
            hand_type = HandType::FiveOfAKind;
        } else if v[0].0 == &4 {
            hand_type = HandType::FourOfAKind;
        } else if v[0].0 == &3 && v[1].0 == &2 {
            hand_type = HandType::FullHouse;
        } else if v[0].0 == &3 {
            hand_type = HandType::ThreeOfAKind;
        } else if v[0].0 == &2 && v[1].0 == &2 {
            hand_type = HandType::TwoPair;
        } else if v[0].0 == &2 {
            hand_type = HandType::OnePair;
        } else {
            hand_type = HandType::HighCard;
        }

        if part == &Part::Two {
            hand_type = Self::apply_jokers(hand_type, &cards);
        }
        Hand {
            cards,
            hand_type,
            high_cards,
        }
    }

    fn apply_jokers(hand_type: HandType, cards: &HashMap<u32, u32>) -> HandType {
        let joker_count = cards.get(&1).unwrap_or(&0);
        if joker_count > &3 {
            HandType::FiveOfAKind
        } else if joker_count == &3 {
            match hand_type {
                HandType::FullHouse => HandType::FiveOfAKind,
                HandType::ThreeOfAKind => HandType::FourOfAKind,
                _ => panic!(),
            }
        } else if joker_count == &2 {
            match hand_type {
                HandType::FullHouse => HandType::FiveOfAKind,
                HandType::TwoPair => HandType::FourOfAKind,
                HandType::OnePair => HandType::ThreeOfAKind,
                _ => panic!(),
            }
        } else if joker_count == &1 {
            match hand_type {
                HandType::FourOfAKind => HandType::FiveOfAKind,
                HandType::ThreeOfAKind => HandType::FourOfAKind,
                HandType::TwoPair => HandType::FullHouse,
                HandType::OnePair => HandType::ThreeOfAKind,
                HandType::HighCard => HandType::OnePair,
                _ => panic!(),
            }
        } else {
            hand_type
        }
    }
}

impl Ord for Round {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand.hand_type.cmp(&other.hand.hand_type) {
            Ordering::Equal => {
                let mut ord: Option<Ordering> = None;
                for (idx, card) in self.hand.high_cards.iter().enumerate() {
                    match card.cmp(&other.hand.high_cards[idx]) {
                        Ordering::Equal => {}
                        Ordering::Less => {
                            ord = Some(Ordering::Less);
                            break;
                        }
                        Ordering::Greater => {
                            ord = Some(Ordering::Greater);
                            break;
                        }
                    }
                }
                ord.unwrap()
            }
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
        }
    }
}

impl PartialOrd for Round {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn result(input: &str, part: Part) -> u32 {
    let lines = input.lines();
    let j_val = match part {
        Part::One => 11,
        Part::Two => 1,
    };
    let picture_cards = HashMap::from([('A', 14), ('K', 13), ('Q', 12), ('J', j_val), ('T', 10)]);
    let mut rounds: Vec<Round> = lines
        .map(|l| {
            let split: Vec<&str> = l.split_whitespace().collect();
            let bid: u32 = split[1].parse().unwrap();
            let hand = Hand::from_chars(split[0].chars(), &part, &picture_cards);
            Round { hand, bid }
        })
        .collect();
    rounds.sort();
    rounds
        .iter()
        .enumerate()
        .map(|(idx, round)| (idx + 1) as u32 * round.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::One), 6440);

        let file_contents = fs::read_to_string("data/input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::One), 249390788);
    }

    #[test]
    fn test_part_2() {
        let file_contents = fs::read_to_string("data/demo_input_2.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::Two), 5905);

        let file_contents = fs::read_to_string("data/input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::Two), 248750248);
    }
}
