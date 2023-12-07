use itertools::Itertools;
use std::{cmp::Ordering, str::FromStr};

use aoc2023::filter_input_lines;

use super::Problem;

const CARD_ORDERING_PART1: &str = "23456789TJQKA";
const CARD_ORDERING_PART2: &str = "J23456789TQKA";

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: u32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

fn get_hand_type(cards: &str) -> HandType {
    let unique_cards = cards.chars().counts();
    match unique_cards.len() {
        1 => HandType::FiveOfKind,
        2 => {
            let count = unique_cards.iter().next().unwrap().1;
            match count {
                1 | 4 => HandType::FourOfKind,
                2 | 3 => HandType::FullHouse,
                _ => panic!(),
            }
        }
        3 => {
            let max_count = unique_cards.iter().map(|c| c.1).max().unwrap();
            match max_count {
                2 => HandType::TwoPair,
                3 => HandType::ThreeOfKind,
                _ => panic!(),
            }
        }
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => panic!(),
    }
}

impl Hand {
    fn hand_type_part1(&self) -> HandType {
        get_hand_type(&self.cards)
    }

    fn hand_type_part2(&self) -> HandType {
        let mut num_js = 0;
        let hand_without_js: String = self
            .cards
            .chars()
            .map(|c| {
                if c == 'J' {
                    num_js += 1;
                    match num_js {
                        1 => 'X',
                        2 => 'Y',
                        3 => 'Z',
                        4 => 'U',
                        5 => 'W',
                        _ => panic!(),
                    }
                } else {
                    c
                }
            })
            .collect();
        let hand_type_without_js = get_hand_type(&hand_without_js);

        if num_js == 0 {
            return hand_type_without_js;
        }

        match hand_type_without_js {
            HandType::HighCard => match num_js {
                1 => HandType::OnePair,
                2 => HandType::ThreeOfKind,
                3 => HandType::FourOfKind,
                4 => HandType::FiveOfKind,
                5 => HandType::FiveOfKind,
                _ => panic!(),
            },
            HandType::OnePair => match num_js {
                1 => HandType::ThreeOfKind,
                2 => HandType::FourOfKind,
                3 => HandType::FiveOfKind,
                _ => panic!(),
            },
            HandType::TwoPair => HandType::FullHouse,
            HandType::ThreeOfKind => match num_js {
                1 => HandType::FourOfKind,
                2 => HandType::FiveOfKind,
                _ => panic!(),
            },
            HandType::FourOfKind => HandType::FiveOfKind,
            _ => panic!(),
        }
    }
}

impl FromStr for Hand {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').unwrap();
        Ok(Hand {
            cards: cards.to_owned(),
            bid: bid.parse().unwrap(),
        })
    }
}

pub struct Day;

impl Problem for Day {
    fn part_one(&self, input: &str) -> String {
        let input = filter_input_lines(input);
        let mut hands: Vec<Hand> = input.iter().map(|s| s.parse().unwrap()).collect();

        hands.sort_by(|a, b| match a.hand_type_part1().cmp(&b.hand_type_part1()) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                for (self_card, other_card) in a.cards.chars().zip(b.cards.chars()) {
                    if self_card == other_card {
                        continue;
                    }
                    let self_card_rank = CARD_ORDERING_PART1.find(self_card).unwrap();
                    let other_card_rank = CARD_ORDERING_PART1.find(other_card).unwrap();
                    return self_card_rank.cmp(&other_card_rank);
                }
                return Ordering::Equal;
            }
        });
        let winnings: u32 = hands.iter().enumerate().map(|(ind, hand)| (ind as u32 + 1) * hand.bid).sum();

        println!("{winnings}");
        format!("{winnings}")
    }

    fn part_two(&self, input: &str) -> String {
        let input = filter_input_lines(input);
        let mut hands: Vec<Hand> = input.iter().map(|s| s.parse().unwrap()).collect();

        hands.sort_by(|a, b| match a.hand_type_part2().cmp(&b.hand_type_part2()) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                for (self_card, other_card) in a.cards.chars().zip(b.cards.chars()) {
                    if self_card == other_card {
                        continue;
                    }
                    let self_card_rank = CARD_ORDERING_PART2.find(self_card).unwrap();
                    let other_card_rank = CARD_ORDERING_PART2.find(other_card).unwrap();
                    return self_card_rank.cmp(&other_card_rank);
                }
                return Ordering::Equal;
            }
        });
        let winnings: u32 = hands.iter().enumerate().map(|(ind, hand)| (ind as u32 + 1) * hand.bid).sum();

        println!("{winnings}");
        format!("{winnings}")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = "
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483";
        let value = Day.part_one(input);
        assert_eq!(value, "6440");
    }

    #[test]
    fn test_part_two() {
        let input = "
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483";
        let value = Day.part_two(input);
        assert_eq!(value, "5905");
    }
}
