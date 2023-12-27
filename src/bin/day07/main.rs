use aoc2023::{get_day_str, read_input};
use core::num;
use std::{collections::HashSet, time::Instant};

const BASE_TEN: usize = 10;

type Cards = [u32; 5];

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

struct Hand {
    cards: [u32; 5],
    bid: u32,
}
impl Hand {
    fn get_type(&self) -> HandType {
        let unique_cards = self.cards.into_iter().collect::<HashSet<_>>();
        let num_unique = unique_cards.len();
        let highest_freq = unique_cards
            .into_iter()
            .map(|u| self.cards.iter().filter(|&&c| c == u).count())
            .max()
            .unwrap();

        match num_unique {
            1 => HandType::FiveOfAKind,
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

fn card_to_value(card: char) -> u32 {
    if card.is_numeric() {
        return card.to_digit(10).unwrap();
    } else {
        match card {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!(),
        }
    }
}

fn part_one(input: &str) -> Option<u32> {
    None
}

fn part_two(_input: &str) -> Option<u32> {
    None
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
