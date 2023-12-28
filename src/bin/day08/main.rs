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
            .filter(|c| c.is_alphabetic() || c.is_whitespace())
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
