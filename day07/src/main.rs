use std::env;
use std::fs;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::convert::From;
use std::str::Chars;
use phf::phf_map;

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

static PICTURE_CARDS: phf::Map<char, u32> = phf_map! {
    'A' => 14,
    'K' => 13,
    'Q' => 12,
    'J' => 11,
    'T' => 10,
};

impl From<Chars<'_>> for Hand {
    fn from(chars: Chars) -> Self {
        let mut cards: HashMap<u32, u32> = HashMap::new();
        let mut high_cards: Vec<u32> = Vec::new();
        let hand_type: HandType;
        chars.for_each(|card| {
            let card_val = if card.is_ascii_digit() {
                card.to_digit(10).unwrap()
            } else {
                *PICTURE_CARDS.get(&card).unwrap()
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
        // let high_cards: Vec<u32> = v.iter().map(|tup| *tup.1 ).collect();
        Hand { cards, hand_type, high_cards }
    }
}

impl Ord for Round {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand.hand_type.cmp(&other.hand.hand_type) {
            Ordering::Equal => {
                let mut ord: Option<Ordering> = None;
                for (idx, card) in self.hand.high_cards.iter().enumerate() {
                    if card < &other.hand.high_cards[idx] {
                        ord = Some(Ordering::Less);
                        break;
                    } else if card > &other.hand.high_cards[idx] {
                        ord = Some(Ordering::Greater);
                        break;
                    }
                };
                ord.unwrap()
            },
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
    let mut rounds: Vec<Round> = lines.map(|l| {
        let split: Vec<&str> = l.split_whitespace().collect();
        let bid: u32 = split[1].parse().unwrap();
        let hand = Hand::from(split[0].chars());
        Round { hand, bid }
    }).collect();

    rounds.sort();

    // println!("{:#?}", rounds);

    let sum: u32 = rounds.iter().enumerate().map(|(idx, round)| {
        (idx + 1) as u32 * round.bid
    }).sum();

    match part {
        Part::One => {
            sum
        }
        Part::Two => {
            unimplemented!()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::One), 6440);
    }

    // #[test]
    // fn test_part_2() {
    //     let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
    //     assert_eq!(result(&file_contents, Part::Two), 2);
    // }
}
