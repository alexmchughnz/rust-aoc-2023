use std::ops::Index;

use aoc2023::{get_day_str, read_input};

struct Schematic {
    chars: Vec<char>,
    width: usize,
    height: usize,
}

impl Index<usize> for Schematic {
    type Output = char;
    fn index(&self, index: usize) -> &Self::Output {
        self.chars.index(index)
    }
}

impl Schematic {
    fn is_valid_index(&self, index: usize) -> bool {
        index < self.chars.len()
    }

    fn index_to_coords(&self, index: usize) -> (i32, i32) {
        let i = index / self.width;
        let j = index % self.width;
        (i as i32, j as i32)
    }

    fn coords_to_index(&self, coords: (i32, i32)) -> usize {
        let index = coords.0 * self.width as i32 + coords.1;
        index as usize
    }

    fn surrounding_chars(&self, index: usize) -> impl Iterator<Item = char> + '_ {
        let (i, j) = self.index_to_coords(index);
        let i = i as i32;
        let j = j as i32;

        let all = [
            (i - 1, j - 1),
            (i - 1, j),
            (i - 1, j + 1),
            (i, j - 1),
            (i, j + 1),
            (i + 1, j - 1),
            (i + 1, j),
            (i + 1, j + 1),
        ];

        let valid_indices = all
            .into_iter()
            .filter(|tup| {
                (0 <= tup.0 && tup.0 < self.height as i32)
                    && (0 <= tup.1 && tup.1 < self.width as i32)
            })
            .map(|tup| self.coords_to_index(tup));

        valid_indices.map(|i| self[i])
    }
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_numeric()
}

fn part_one(input: &str) -> Option<u32> {
    let width = input.find('\n').unwrap();
    let height = input.matches('\n').count();
    let schematic = Schematic {
        chars: input.chars().filter(|c| *c != '\n').collect(),
        width,
        height,
    };

    let mut nums = Vec::<u32>::new();

    let mut i = 0;
    loop {
        let mut is_part_number = false;

        // Search for number.
        while schematic.is_valid_index(i) && !schematic[i].is_numeric() {
            i += 1;
        }

        // Scan number.
        let i_start = i;
        while schematic.is_valid_index(i) && schematic[i].is_numeric() {
            if schematic.surrounding_chars(i).any(is_symbol) {
                is_part_number = true;
            }

            i += 1;
            if i % schematic.width == 0 {
                // Don't parse numbers over a newline!
                break;
            }
        }

        // Collect valid 'part numbers'.
        if is_part_number {
            let num: String = schematic
                .chars
                .iter()
                .skip(i_start)
                .take(i - i_start)
                .collect();
            nums.push(num.parse::<u32>().unwrap());
        }

        // Break once `chars` is exhausted.
        if i >= schematic.chars.len() {
            break;
        }
    }

    Some(nums.into_iter().sum())
}

fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = read_input(file!(), "input.txt");
    let day = get_day_str(file!());
    println!("{day}-1 solution: {:?}", part_one(&input));
    println!("{day}-2 solution: {:?}", part_two(&input));
}
