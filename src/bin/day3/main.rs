use aoc2023::helpers::grid::{Grid, GridIndex};
use aoc2023::{get_day_str, read_input};

type Schematic = Grid<char>;

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_numeric()
}

fn part_one(input: &str) -> Option<u32> {
    let schematic: Schematic = input.parse().unwrap();
    let mut part_numbers = Vec::<u32>::new();

    let mut index = GridIndex(0, 0);
    'main: loop {
        let mut is_part_number = false;

        // Search for number.
        loop {
            let res = index.increment(&schematic);

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

            let surrounds = schematic.surrounding_indices(index);
            if surrounds.map(|index| schematic[index]).any(is_symbol) {
                is_part_number = true;
            }

            if index.move_right(&schematic).is_err() {
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

fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = read_input(file!(), "input.txt");
    let day = get_day_str(file!());
    println!("{day}-1 solution: {:?}", part_one(&input));
    println!("{day}-2 solution: {:?}", part_two(&input));
}
