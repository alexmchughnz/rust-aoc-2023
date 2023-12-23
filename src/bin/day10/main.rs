use aoc2023::{get_day_str, read_input};
use std::time::Instant;

use std::ops::Not;

use aoc2023::helpers::grid::Grid;
use aoc2023::helpers::grid::GridDirection;
use aoc2023::helpers::grid::GridIndex;
use GridDirection::*;

mod symbols;

#[derive(PartialEq, Eq, Debug)]
enum Location {
    Inside,
    Outside,
}
use Location::*;

impl Not for Location {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Location::Inside => Location::Outside,
            Location::Outside => Location::Inside,
        }
    }
}

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

fn part_two(input: &str) -> Option<u32> {
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
    let mut visted_pipes = Vec::<GridIndex>::new();
    visted_pipes.push(start_index);

    let mut current = start_index.clone().step(dir, &grid).unwrap();
    while current != start_index {
        visted_pipes.push(current);
        let symbol = grid[current];
        dir = symbols::traverse_pipe(dir, symbol).unwrap();
        current = current.step(dir, &grid).unwrap();
    }

    // Count all indices "inside" the pipe structure.
    let mut inside_indices = Vec::<GridIndex>::new();
    for i in 0..grid.height() {
        let mut location = Outside;

        for j in 0..grid.width() {
            let index = GridIndex(i, j);
            let is_pipe = visted_pipes.contains(&index);
            let connects_right = symbols::traverse_pipe(Left, grid[index]).is_some();

            println!("\nAt {index:?} = {:?}", grid[index]);
            match location {
                Outside => {
                    if is_pipe {
                        location = Inside;
                        println!("Moving {location:?}");
                    }
                }
                Inside => {
                    if is_pipe && !connects_right {
                        location = Outside;
                        println!("Moving {location:?}");
                    }
                    if !is_pipe {
                        inside_indices.push(index);
                        println!("Adding index!");
                    }
                }
            }
        }
    }
    // dbg!(&inside_indices);
    Some(inside_indices.len() as u32)
}

fn main() {
    let input = read_input(file!(), "example3.txt");
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
