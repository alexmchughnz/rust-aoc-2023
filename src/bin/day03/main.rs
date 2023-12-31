use std::time::Instant;

use aoc2023::{get_day_str, read_input};

use aoc2023::helpers::grid::{Grid, GridDirection};
use GridDirection::*;

type Schematic = Grid<char>;

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_numeric()
}

fn part_one(input: &str) -> Option<u32> {
    let schematic: Schematic = input.parse().unwrap();
    let mut part_numbers = Vec::<u32>::new();

    let mut index = schematic.make_index(0, 0);
    'main: loop {
        let mut is_part_number = false;

        // Search for number.
        loop {
            let res = index.increment();

            if res.is_err() {
                // Reached end of grid.
                break 'main;
            }

            if schematic[index].is_numeric() {
                break;
            }
        }

        // Scan number.
        let mut number = String::new();
        while schematic[index].is_numeric() {
            number.push(schematic[index]);

            let surrounds = index.surrounding();
            if surrounds.map(|i| schematic[i]).any(is_symbol) {
                is_part_number = true;
            }

            let res = index.step(Right);
            if res.is_err() {
                break;
            }
        }

        // If a symbol was adjacent, collect this part number.
        if is_part_number {
            part_numbers.push(number.parse::<u32>().unwrap());
        }
    }

    Some(part_numbers.into_iter().sum())
}

fn part_two(input: &str) -> Option<u32> {
    let schematic: Schematic = input.parse().unwrap();
    let mut gear_ratios = Vec::<u32>::new();

    let mut index = schematic.make_index(0, 0);
    'main: loop {
        // Search for gear symbol ('*').
        loop {
            let res = index.increment();

            if res.is_err() {
                // Reached end of grid.
                break 'main;
            }

            if schematic[index] == '*' {
                break;
            }
        }

        // Search surrounds for numbers.
        let surrounds = index.surrounding();
        let indices_with_digits: Vec<_> =
            surrounds.filter(|i| schematic[*i].is_numeric()).collect();
        let indices_with_numbers: Vec<_> = indices_with_digits
            .iter()
            .filter(|i| {
                // If an index's left neighbour is ALSO a gear-adjacent digit, they're part of the same number.
                let left = i.neighbour(Left);
                left.is_none() || !indices_with_digits.contains(&left.unwrap())
            })
            .collect();

        // If valid gear, count and parse surrounding numbers.
        const NUMBERS_PER_GEAR: usize = 2;
        if indices_with_numbers.len() != NUMBERS_PER_GEAR {
            continue 'main;
        }

        let mut gear_ratio = 1;
        for index in indices_with_numbers {
            // Find start of number.
            let mut leftmost = *index;
            loop {
                let left = leftmost.neighbour(Left);
                if left.is_some_and(|i| schematic[i].is_numeric()) {
                    leftmost = left.unwrap();
                } else {
                    break;
                }
            }

            // Scan number.
            let mut number = String::new();
            while schematic[leftmost].is_numeric() {
                number.push(schematic[leftmost]);
                let res = leftmost.step(Right);

                if res.is_err() {
                    break;
                }
            }

            // Factor number into gear ratio.
            gear_ratio *= number.parse::<u32>().unwrap();
        }

        gear_ratios.push(gear_ratio)
    }

    Some(gear_ratios.into_iter().sum())
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
