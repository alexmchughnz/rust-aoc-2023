use aoc2023::{get_day_str, read_input};
use std::time::Instant;

fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut predictions = Vec::<_>::new();
    for line in lines {
        let nums = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        // Generate sequences of diffs.
        let mut all_diffs = Vec::<Vec<_>>::new();
        all_diffs.push(nums);
        while !all_diffs.last().unwrap().iter().all(|&d| d == 0) {
            let next_diffs = all_diffs.last().unwrap().windows(2).map(|w| w[1] - w[0]);
            all_diffs.push(next_diffs.collect());
        }
        assert!(all_diffs.last().unwrap().iter().all(|&d| d == 0));

        // Interpolate diffs to predict next value.
        let mut delta = 0;
        for diffs in all_diffs.iter_mut().rev() {
            let end = diffs.last().unwrap();
            let new = end + delta;
            diffs.push(new);

            delta = new;
        }

        predictions.push(delta);
    }

    Some(predictions.into_iter().sum::<i32>() as u32)
}

fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut predictions = Vec::<_>::new();
    for line in lines {
        let nums = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        // Generate sequences of diffs.
        let mut all_diffs = Vec::<Vec<_>>::new();
        all_diffs.push(nums.into());
        while !all_diffs.last().unwrap().iter().all(|&d| d == 0) {
            let next_diffs = all_diffs.last().unwrap().windows(2).map(|w| w[1] - w[0]);
            all_diffs.push(next_diffs.collect());
        }
        assert!(all_diffs.last().unwrap().iter().all(|&d| d == 0));

        // Interpolate diffs to predict *previous* value.
        let mut delta = 0;
        for diffs in all_diffs.iter_mut().rev() {
            let start = diffs.first().unwrap();
            let new = start - delta;
            diffs.insert(0, new);

            delta = new;
        }

        predictions.push(delta);
    }

    Some(predictions.into_iter().sum::<i32>() as u32)
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
