use aoc2023::{get_day_str, read_input};
use std::{collections::HashSet, str::FromStr, time::Instant};

#[derive(Debug)]
struct Card {
    id: usize,
    card_nums: Vec<u32>,
    winning_nums: Vec<u32>,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut parts = line.split(':');
        let id = parts
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let parse_nums = |nums: &str| {
            nums.split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect()
        };
        let mut number_lists = parts.next().unwrap().split('|');
        let card_nums = parse_nums(number_lists.next().unwrap());
        let winning_nums = parse_nums(number_lists.next().unwrap());

        Ok(Card {
            id,
            card_nums,
            winning_nums,
        })
    }
}

fn part_one(input: &str) -> Option<u32> {
    let mut scores = Vec::<u32>::new();

    for line in input.lines() {
        let card: Card = line.parse().unwrap();
        let card_nums: HashSet<_> = card.card_nums.into_iter().collect();
        let winning_nums: HashSet<_> = card.winning_nums.into_iter().collect();

        let num_winners = card_nums.intersection(&winning_nums).count() as u32;
        let score = {
            if num_winners > 0 {
                2u32.pow(num_winners - 1)
            } else {
                0
            }
        };
        scores.push(score);
    }

    Some(scores.into_iter().sum())
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
