use aoc2023::{get_day_str, read_input};
use std::time::Instant;

use aoc2023::helpers::grid::Grid;

mod symbols;
fn part_one(input: &str) -> Option<u32> {
    let grid: Grid<char> = input.parse().unwrap();

    // Find the loop start and a valid direction to move in.
    let start_index = grid.find('S').unwrap();
    let mut valid_movements =
        grid.adjacent_indices(start_index)
            .into_iter()
            .filter_map(|(dir, index)| {
                let symbol = grid[index?];
                if symbols::traverse_pipe(dir, symbol).is_some() {
                    Some(dir)
                } else {
                    None
                }
            });
    let mut dir = valid_movements.next().unwrap(); // Pick arbitrary valid direction.

    // Follow the pipes until the start index is reached again.
    let mut current = start_index.clone().step(dir, &grid).unwrap();
    let mut num_steps: u32 = 1;
    while current != start_index {
        let symbol = grid[current];
        dir = symbols::traverse_pipe(dir, symbol).unwrap();
        current = current.step(dir, &grid).unwrap();
        num_steps += 1;
    }

    let farthest_distance = num_steps / 2;
    Some(farthest_distance)
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
