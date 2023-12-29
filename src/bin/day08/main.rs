use aoc2023::{get_day_str, read_input};
use num::Integer;
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
#[derive(Clone, Copy, Default)]
struct Cycle {
    first_hit: usize,
    prev_hit: usize,
    period: usize,
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
    let mut num_steps = 0;

    let starts = network.keys().filter(|s| s.ends_with('A'));
    let mut currents = starts.collect::<Vec<_>>();
    let mut cycles = vec![Cycle::default(); currents.len()];
    while cycles.iter().any(|cyc| cyc.period == 0) {
        num_steps += 1;
        let instruction = instructions.next();

        for (i, current) in currents.iter_mut().enumerate() {
            let paths = network.get(*current).unwrap();
            *current = match instruction {
                Some('L') => &paths.0,
                Some('R') => &paths.1,
                _ => unreachable!(),
            };

            if current.ends_with('Z') {
                let cycle = &mut cycles[i];
                if cycle.first_hit == 0 {
                    cycle.first_hit = num_steps;
                } else {
                    cycle.period = num_steps - cycle.prev_hit;
                }
                cycle.prev_hit = num_steps;

                println!(
                    "Start {} hit 'Z'. First hit = {}. Delta = {}",
                    i, cycle.first_hit, cycle.period
                );
            }
        }
    }

    let steps_to_finish = cycles
        .into_iter()
        .map(|cyc| cyc.period)
        .fold(1, |multiple, ref x| multiple.lcm(x));
    Some(steps_to_finish as u64)
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
