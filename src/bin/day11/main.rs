use aoc2023::{get_day_str, read_input};
use std::{
    cmp::{max, min},
    time::Instant,
};

use aoc2023::helpers::grid::Grid;

fn part_one(input: &str) -> Option<u32> {
    let image: Grid<char> = input.parse().unwrap();
    // Find all galaxies.
    let mut galaxies = Vec::new();
    for (index, c) in image.iter() {
        if *c == '#' {
            galaxies.push(index);
        }
    }

    // Locate indices of double-width rows/columns.
    let doubled_rows = (0..image.height())
        .into_iter()
        .filter(|&i| {
            galaxies
                .iter()
                .all(|galaxy_index| galaxy_index.indices.0 != i)
        })
        .collect::<Vec<_>>();

    let doubled_columns = (0..image.width())
        .into_iter()
        .filter(|&j| {
            galaxies
                .iter()
                .all(|galaxy_index| galaxy_index.indices.1 != j)
        })
        .collect::<Vec<_>>();

    // Determine distance between each pair of galaxies, counting doubled rows/columns twice.
    let mut distances = Vec::new();
    for n in 1..galaxies.len() {
        let (start, rest) = galaxies.split_at(n);
        let a = start.last().unwrap().indices;
        for b in rest.into_iter().map(|index| index.indices) {
            let rows_between = min(a.0, b.0)..max(a.0, b.0);
            let columns_between = min(a.1, b.1)..max(a.1, b.1);

            let num_doubled_rows = doubled_rows
                .iter()
                .filter(|&i| rows_between.contains(i))
                .count();
            let num_expanded_columns = doubled_columns
                .iter()
                .filter(|&j| columns_between.contains(j))
                .count();

            let i_distance = rows_between.len() + num_doubled_rows;
            let j_distance = columns_between.len() + num_expanded_columns;
            distances.push(i_distance + j_distance);
        }
    }

    Some(distances.into_iter().sum::<usize>() as u32)
}

const EXPANSION_SIZE: usize = 1_000_000;
fn part_two(input: &str) -> Option<u64> {
    let image: Grid<char> = input.parse().unwrap();

    // Find all galaxies.
    let mut galaxies = Vec::new();
    for (index, c) in image.iter() {
        if *c == '#' {
            galaxies.push(index);
        }
    }

    // Locate indices of double-width rows/columns.
    let expanded_rows = (0..image.height())
        .into_iter()
        .filter(|&i| {
            galaxies
                .iter()
                .all(|galaxy_index| galaxy_index.indices.0 != i)
        })
        .collect::<Vec<_>>();

    let expanded_columns = (0..image.width())
        .into_iter()
        .filter(|&j| {
            galaxies
                .iter()
                .all(|galaxy_index| galaxy_index.indices.1 != j)
        })
        .collect::<Vec<_>>();

    // Determine distance between each pair of galaxies, counting doubled rows/columns twice.
    let mut distances = Vec::new();
    for n in 1..galaxies.len() {
        let (start, rest) = galaxies.split_at(n);
        let a = start.last().unwrap().indices;
        for b in rest.into_iter().map(|index| index.indices) {
            let rows_between = min(a.0, b.0)..max(a.0, b.0);
            let columns_between = min(a.1, b.1)..max(a.1, b.1);

            let num_expanded_rows = expanded_rows
                .iter()
                .filter(|&i| rows_between.contains(i))
                .count();
            let num_expanded_columns = expanded_columns
                .iter()
                .filter(|&j| columns_between.contains(j))
                .count();

            let i_distance = rows_between.len() + num_expanded_rows * (EXPANSION_SIZE - 1);
            let j_distance = columns_between.len() + num_expanded_columns * (EXPANSION_SIZE - 1);
            distances.push(i_distance + j_distance);
        }
    }

    Some(distances.into_iter().sum::<usize>() as u64)
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
