use aoc2023::{get_day_str, read_input};
use core::num;
use std::time::Instant;

fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let times: Vec<u32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
        .collect(); // [milliseconds]
    let records: Vec<u32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
        .collect(); // [millimeters]

    let mut num_winning_combos = Vec::new();
    for race in 0..times.len() {
        let mut num_wins = 0;
        let total_time = times[race];

        for hold_time in 1..total_time {
            let travel_time = total_time - hold_time;
            let speed = hold_time;

            let distance = speed * travel_time;
            if distance > records[race] {
                num_wins += 1;
            }
        }

        num_winning_combos.push(num_wins);
    }

    Some(num_winning_combos.into_iter().product())
}

fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let time: u64 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>()
        .unwrap();
    let record: u64 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    let mut num_wins = 0;

    for hold_time in 1..time {
        let travel_time = time - hold_time;
        let speed = hold_time;

        let distance = speed * travel_time;
        if distance > record {
            num_wins += 1;
        }
    }

    Some(num_wins)
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
