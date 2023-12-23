use aoc2023::{get_day_str, read_input};
use std::time::Instant;

use aoc2023::helpers::grid::Grid;

mod symbols;
fn part_one(input: &str) -> Option<u32> {
    let grid: Grid<char> = input.parse().unwrap();

    // Find the loop start and a valid direction to move in.
    let start_index = grid.find('S').unwrap();
    let all_movements = grid
        .adjacent_indices(start_index)
        .into_iter()
        .collect::<Vec<_>>();

    let valid_movements = all_movements
        .into_iter()
        .filter(|(dir, index)| {
            if index.is_none() {
                return false;
            }
            let symbol = grid[index.unwrap()];
            symbols::traverse_pipe(*dir, symbol).is_some()
        })
        .collect::<Vec<_>>();
    let mut pipe_dir = valid_movements[0].0;

    // Follow the pipes until the start index is reached again.
    let mut current = start_index.clone().step(pipe_dir, &grid).unwrap();
    let mut num_steps: u32 = 1;
    while current != start_index {
        let symbol = grid[current];
        pipe_dir = symbols::traverse_pipe(pipe_dir, symbol).unwrap();
        current = current.step(pipe_dir, &grid).unwrap();
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
