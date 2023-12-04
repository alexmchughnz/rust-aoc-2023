use aoc2023::{get_day_str, read_input};
use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn parse(s: &str) -> Self {
        match s {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => panic!("invalid color string"),
        }
    }
}

#[derive(Debug, Default)]
struct Game {
    id: u32,
    most_cubes: HashMap<Color, u32>,
}

impl Game {
    fn is_subset_of(&self, parent: &Self) -> bool {
        parent
            .most_cubes
            .keys()
            .all(|c| self.most_cubes[c] <= parent.most_cubes[c])
    }
}

fn part_one(input: &str) -> Option<u32> {
    let test_game = Game {
        most_cubes: HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]),
        ..Game::default()
    };

    let mut games = Vec::<Game>::new();

    for mut game_desc in input.lines().map(|s| s.split(":")) {
        // Init `game` struct with id.
        let id = game_desc
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let mut game = Game {
            id,
            most_cubes: HashMap::default(),
        };

        // Find maximum value for each color over all `play`s.
        for set in game_desc.next().unwrap().split(';') {
            for mut play in set.split(',').map(|s| s.split_whitespace()) {
                let n = play.next().unwrap().parse::<u32>().unwrap();
                let color = play.next().unwrap();

                let current_max = game.most_cubes.entry(Color::parse(color)).or_default();
                if n > *current_max {
                    *current_max = n;
                }
            }
        }
        games.push(game);
    }

    let valid_ids = games
        .iter_mut()
        .filter(|g| g.is_subset_of(&test_game))
        .map(|g| g.id);

    Some(valid_ids.sum())
}

fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = read_input(file!(), "input");
    let day = get_day_str(file!());
    println!("{day}-1 solution: {:?}", part_one(&input));
    println!("{day}-2 solution: {:?}", part_two(&input));
}
