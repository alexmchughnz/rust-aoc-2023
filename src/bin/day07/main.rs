use aoc2023::{get_day_str, read_input};

use std::{cmp::Ordering, collections::HashSet, time::Instant};

const BASE_TEN: u32 = 10;
const CARDS_PER_HAND: usize = 5;

type Cards = [u32; CARDS_PER_HAND];

fn card_to_value(card: char, jokers_on: bool) -> u32 {
    if card.is_numeric() {
        card.to_digit(BASE_TEN).unwrap()
    } else {
        match card {
            'T' => 10,
            'J' => match jokers_on {
                false => 11,
                true => 0,
            },
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Cards,
    bid: u32,
    jokers_on: bool, // Lazy implementation to switch settings between Parts 1 & 2.
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_ordering = self.determine_type().cmp(&other.determine_type());
        if type_ordering != Ordering::Equal {
            return type_ordering;
        }

        for i in 0..CARDS_PER_HAND {
            let card_ordering = self.cards[i].cmp(&other.cards[i]);
            if card_ordering != Ordering::Equal {
                return card_ordering;
            }
        }

        Ordering::Equal
    }
}

impl Hand {
    fn determine_type(&self) -> HandType {
        // If enabled, jokers will match the highest frequency card for the purposes of card matching.
        let num_jokers = if self.jokers_on {
            self.cards
                .iter()
                .filter(|&&v| v == card_to_value('J', true))
                .count()
        } else {
            0
        };

        let unique_cards = self
            .cards
            .into_iter()
            .filter(|&c| c != 0) // excluding jokers
            .collect::<HashSet<_>>();
        let num_unique = unique_cards.len();

        let highest_freq = unique_cards // excluding jokers
            .into_iter()
            .map(|u| self.cards.iter().filter(|&&c| c == u).count())
            .max()
            .unwrap_or(0)
            + num_jokers;

        match num_unique {
            0 | 1 => HandType::FiveOfAKind, // 0 signifies all jokers
            2 => match highest_freq {
                4 => HandType::FourOfAKind,
                3 => HandType::FullHouse,
                _ => unreachable!(),
            },
            3 => match highest_freq {
                3 => HandType::ThreeOfAKind,
                2 => HandType::TwoPair,
                _ => unreachable!(),
            },
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    // Collect all hands from input.
    let mut hands = Vec::<Hand>::new();
    for line in lines {
        let mut parts = line.split_whitespace();
        let hand_str = parts.next().unwrap();
        let bid = parts.next().unwrap();

        let hand = Hand {
            cards: std::array::from_fn(|n| card_to_value(hand_str.chars().nth(n).unwrap(), false)),
            bid: bid.parse::<u32>().unwrap(),
            jokers_on: false,
        };
        hands.push(hand);
    }

    // Sort all hands by comparing (with `Ord`).
    hands.sort();
    let winnings = hands.into_iter().enumerate().map(|(i, hand)| {
        let rank = (i + 1) as u32;
        rank * hand.bid
    });

    Some(winnings.sum())
}

fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();

    // Collect all hands from input.
    let mut hands = Vec::<Hand>::new();
    for line in lines {
        let mut parts = line.split_whitespace();
        let hand_str = parts.next().unwrap();
        let bid = parts.next().unwrap();

        let hand = Hand {
            cards: std::array::from_fn(|n| card_to_value(hand_str.chars().nth(n).unwrap(), true)),
            bid: bid.parse::<u32>().unwrap(),
            jokers_on: true,
        };
        hands.push(hand);
    }

    // Sort all hands by comparing (with `Ord`).
    hands.sort();
    let winnings = hands.into_iter().enumerate().map(|(i, hand)| {
        let rank = (i + 1) as u32;
        rank * hand.bid
    });

    Some(winnings.sum())
}

fn main() {
    let input = read_input(file!(), "input.txt");
    let day = get_day_str(file!());

    let time = Instant::now();
    println!(
        "{day}-1 solution: {:?} | {:.2?}",
        part_one(&input),
        time.elapsed()
    );

    let time = Instant::now();
    println!(
        "{day}-2 solution: {:?} | {:.2?}",
        part_two(&input),
        time.elapsed()
    );
}
