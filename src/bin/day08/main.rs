use aoc2023::{get_day_str, read_input};
use std::time::Instant;

fn part_one(_input: &str) -> Option<u32> {
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
