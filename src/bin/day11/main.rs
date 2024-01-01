use aoc2023::{get_day_str, read_input};
use std::time::Instant;

use aoc2023::helpers::grid::Grid;

fn part_one(input: &str) -> Option<u32> {
    let image: Grid<char> = input.parse().unwrap();
    dbg!(&image);

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
            let mut row_indices = [a.0, b.0];
            row_indices.sort();

            let mut column_indices = [a.1, b.1];
            column_indices.sort();

            let num_doubled_rows = doubled_rows
                .iter()
                .filter(|&&i| row_indices[0] < i && i < row_indices[1])
                .count();
            let num_doubled_columns = doubled_columns
                .iter()
                .filter(|&&j| column_indices[0] < j && j < column_indices[1])
                .count();

            let i_distance = row_indices[1] - row_indices[0] + num_doubled_rows;
            let j_distance = column_indices[1] - column_indices[0] + num_doubled_columns;
            distances.push(i_distance + j_distance);
        }
    }

    Some(distances.into_iter().sum::<usize>() as u32)
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
