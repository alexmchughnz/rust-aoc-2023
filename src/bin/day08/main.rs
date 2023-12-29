use aoc2023::{get_day_str, read_input};
use std::{collections::HashMap, time::Instant};

fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let mut instructions = lines.next().unwrap().chars().cycle();
    lines.next(); // blank line

    // Populate the network.
    let mut network = HashMap::<String, (String, String)>::new();
    while let Some(line) = lines.next() {
        let [node, left, right] = line
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect::<String>()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .try_into()
            .unwrap();

        network.insert(node, (left, right));
    }

    // Traverse network until `finish` is reached.
    let mut num_steps = 0;

    let finish: &str = "ZZZ";
    let mut current = "AAA";
    while current != finish {
        let paths = network.get(current).unwrap();
        current = match instructions.next() {
            Some('L') => &paths.0,
            Some('R') => &paths.1,
            _ => unreachable!(),
        };
        num_steps += 1;
    }

    Some(num_steps)
}

fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let mut instructions = lines.next().unwrap().chars().cycle();
    lines.next(); // blank line

    // Populate the network.
    let mut network = HashMap::<String, (String, String)>::new();
    while let Some(line) = lines.next() {
        let [node, left, right] = line
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect::<String>()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .try_into()
            .unwrap();

        network.insert(node, (left, right));
    }

    // Traverse network until all starts reach a node ending in 'Z'.
    let time = Instant::now();
    let mut num_steps: u64 = 0;

    let starts = network.keys().filter(|s| s.ends_with('A'));
    let mut currents = starts.collect::<Vec<_>>();
    while !currents.iter().all(|s| s.ends_with('Z')) {
        num_steps += 1;
        if num_steps % 1_000_000 == 0 {
            println!("Step {num_steps} @ {:?}", time.elapsed());
        }

        let instruction = instructions.next();
        for current in currents.iter_mut() {
            let paths = network.get(*current).unwrap();
            *current = match instruction {
                Some('L') => &paths.0,
                Some('R') => &paths.1,
                _ => unreachable!(),
            };
        }
    }

    Some(num_steps)
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
