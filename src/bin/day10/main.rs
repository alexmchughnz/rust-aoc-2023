use aoc2023::{get_day_str, read_input};
use std::time::Instant;

use aoc2023::helpers::grid::{Grid, GridDirection, GridIndex};
use GridDirection::*;

mod pipes;

fn part_one(input: &str) -> Option<u32> {
    let grid: Grid<char> = input.parse().unwrap();

    // Find the loop start and a valid direction to move in.
    let start_index = grid.find('S').unwrap();
    let mut valid_movements =
        grid.adjacent_indices(start_index)
            .into_iter()
            .filter_map(|(dir, index)| {
                let symbol = grid[index?];
                if pipes::traverse_pipe(dir, symbol).is_some() {
                    Some(dir)
                } else {
                    None
                }
            });
    let mut dir = valid_movements.next().unwrap(); // Pick arbitrary valid direction.

    // Follow the pipes until the start index is reached again.
    let mut current = *start_index.clone().step(dir, &grid).unwrap();
    let mut num_steps: u32 = 1;
    while current != start_index {
        let symbol = grid[current];
        dir = pipes::traverse_pipe(dir, symbol).unwrap();
        current = *current.step(dir, &grid).unwrap();
        num_steps += 1;
    }

    let farthest_distance = num_steps / 2;
    Some(farthest_distance)
}

fn part_two(input: &str) -> Option<u32> {
    let mut grid: Grid<char> = input.parse().unwrap();

    // Find the loop start and a valid direction to move in.
    let start_index = grid.find('S').unwrap();
    let valid_movements: Vec<_> = grid
        .adjacent_indices(start_index)
        .into_iter()
        .filter(|(dir, index)| index.is_some_and(|i| pipes::traverse_pipe(*dir, grid[i]).is_some()))
        .map(|(dir, _)| dir)
        .collect();

    // Replace the start symbol with correct pipe char (for later traversal).
    let start_symbol = pipes::dirs_to_pipe(valid_movements.iter().cloned()).unwrap();
    grid[start_index] = start_symbol;

    // Follow the pipes until the start index is reached again.
    let mut visted_pipes = Vec::<GridIndex>::new();
    visted_pipes.push(start_index);

    let mut dir = valid_movements.into_iter().next().unwrap(); // Pick arbitrary valid direction to start.
    let mut pipe_index = *start_index.clone().step(dir, &grid).unwrap();
    while pipe_index != start_index {
        visted_pipes.push(pipe_index);
        let symbol = grid[pipe_index];
        dir = pipes::traverse_pipe(dir, symbol).unwrap();
        pipe_index.step(dir, &grid).unwrap();
    }

    // Starting from the top left and moving Right, travel row-by-row until the pipe is hit.
    let mut pipe_index = GridIndex(0, 0);
    let mut pipe_dir = Right;
    while !visted_pipes.contains(&pipe_index) {
        pipe_index
            .increment(&grid)
            .expect("should find pipe before end of array");
    }
    assert!(grid[pipe_index] == 'F');
    let entry_index = pipe_index;

    // Traverse the pipe system, recording each adjacent interior tile.
    let mut interior_indices = Vec::<GridIndex>::new();
    let mut inward_dir = Right;
    loop {
        // Check interior tile (both directions if rounding a corner).
        let prev_inward_dir = inward_dir;
        inward_dir = pipes::map_inward_dir(inward_dir, grid[pipe_index]).unwrap();
        for dir in [prev_inward_dir, inward_dir] {
            let inward_index = pipe_index.get_neighbour(dir, &grid).unwrap();
            if !visted_pipes.contains(&inward_index) && !interior_indices.contains(&inward_index) {
                interior_indices.push(inward_index);
            }
        }

        // Get next index.
        pipe_index.step(pipe_dir, &grid).unwrap();
        if pipe_index == entry_index {
            break;
        }
        pipe_dir = pipes::traverse_pipe(pipe_dir, grid[pipe_index]).unwrap();
    }

    // Recursively locate any interior tiles adjacent to those already found.
    let mut i: usize = 0;
    loop {
        let index = interior_indices[i];
        for adjacent in grid.adjacent_indices(index).values().filter_map(|d| *d) {
            if !visted_pipes.contains(&adjacent) && !interior_indices.contains(&adjacent) {
                interior_indices.push(adjacent);
            }
        }

        i += 1;
        if i >= interior_indices.len() {
            break;
        }
    }

    Some(interior_indices.len() as u32)
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
