use aoc2023::{get_day_str, read_input};
use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
    str::FromStr,
    time::Instant,
};

#[derive(Debug)]
struct Card {
    id: usize,
    card_nums: Vec<u32>,
    winning_nums: Vec<u32>,
}

type CardCountMap = HashMap<usize, u32>;
type CardPrizesMap = HashMap<usize, Option<Vec<usize>>>;

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

fn count_prizes(id: usize, card_prizes_map: &CardPrizesMap) -> CardCountMap {
    let mut prize_counts = CardCountMap::new();

    // Count this card.
    prize_counts
        .entry(id)
        .and_modify(|count| *count += 1)
        .or_insert(1);

    // Add each child's prizes to `prize_counts`.
    let maybe_prize_ids = card_prizes_map.get(&id).unwrap();
    if let Some(prize_ids) = maybe_prize_ids {
        for &id in prize_ids {
            let child_counts = count_prizes(id, card_prizes_map);
            for (k, v) in child_counts {
                prize_counts
                    .entry(k)
                    .and_modify(|count| *count += v)
                    .or_insert(v);
            }
        }
    }

    prize_counts
}

fn part_two(input: &str) -> Option<u32> {
    let mut card_totals_map = CardCountMap::new();
    let mut card_prizes_map = CardPrizesMap::new();

    let all_cards: Vec<Card> = input.lines().map(|line| line.parse().unwrap()).collect();

    // Find each card's prize IDs.
    for card in all_cards.iter() {
        // Determine the card ids won by this card.
        let card_nums: HashSet<_> = card.card_nums.iter().collect();
        let winning_nums: HashSet<_> = card.winning_nums.iter().collect();

        let num_winners = card_nums.intersection(&winning_nums).count();
        let prize_ids = {
            if num_winners == 0 {
                None
            } else {
                let ids = (card.id + 1)..=(card.id + num_winners);
                Some(ids.collect())
            }
        };

        // Add prize ids to map.
        assert!(card_prizes_map.get(&card.id).is_none());
        card_prizes_map.insert(card.id, prize_ids);
    }

    for card in all_cards {
        // Recursively count card, and all its prizes.
        let prize_counts = count_prizes(card.id, &card_prizes_map);

        // Update `card_totals_map` with this card's prizes.
        for (k, v) in prize_counts {
            card_totals_map
                .entry(k)
                .and_modify(|count| *count += v)
                .or_insert(v);
        }
    }

    Some(card_totals_map.values().into_iter().sum())
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
