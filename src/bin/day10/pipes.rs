use aoc2023::helpers::grid::GridDirection;
use aoc2023::helpers::grid::GridDirection::*;

pub fn dirs_to_pipe(dirs: impl Iterator<Item = GridDirection>) -> Option<char> {
    let dirs: Vec<_> = dirs.collect();
    if dirs.len() != 2 {
        return None;
    }

    if dirs.contains(&Up) && dirs.contains(&Down) {
        return Some('|');
    }
    if dirs.contains(&Left) && dirs.contains(&Right) {
        return Some('-');
    }
    if dirs.contains(&Up) && dirs.contains(&Right) {
        return Some('L');
    }
    if dirs.contains(&Up) && dirs.contains(&Left) {
        return Some('J');
    }
    if dirs.contains(&Left) && dirs.contains(&Down) {
        return Some('7');
    }
    if dirs.contains(&Right) && dirs.contains(&Down) {
        return Some('F');
    }
    None
}

pub fn traverse_pipe(dir: GridDirection, symbol: char) -> Option<GridDirection> {
    match symbol {
        '|' => traverse_vertical_pipe(dir),
        '-' => traverse_horizontal_pipe(dir),
        'L' => traverse_north_east_bend(dir),
        'J' => traverse_north_west_bend(dir),
        '7' => traverse_south_west_bend(dir),
        'F' => traverse_south_east_bend(dir),
        _ => None,
    }
}

pub fn map_inward_dir(dir: GridDirection, symbol: char) -> Option<GridDirection> {
    match symbol {
        '|' => map_inward_vertical_pipe(dir),
        '-' => map_inward_horizontal_pipe(dir),
        'L' => map_inward_north_east_bend(dir),
        'J' => map_inward_north_west_bend(dir),
        '7' => map_inward_south_west_bend(dir),
        'F' => map_inward_south_east_bend(dir),
        _ => None,
    }
}

// symbol |
fn traverse_vertical_pipe(dir: GridDirection) -> Option<GridDirection> {
    match dir {
        Up => Some(Up),
        Down => Some(Down),
        _ => None,
    }
}

fn map_inward_vertical_pipe(dir: GridDirection) -> Option<GridDirection> {
    match dir {
        Left => Some(Left),
        Right => Some(Right),
        _ => None,
    }
}

// symbol -
fn traverse_horizontal_pipe(dir: GridDirection) -> Option<GridDirection> {
    match dir {
        Left => Some(Left),
        Right => Some(Right),
        _ => None,
    }
}

fn map_inward_horizontal_pipe(dir: GridDirection) -> Option<GridDirection> {
    match dir {
        Up => Some(Up),
        Down => Some(Down),
        _ => None,
    }
}

// symbol L
fn traverse_north_east_bend(dir: GridDirection) -> Option<GridDirection> {
    match dir {
        Down => Some(Right),
        Left => Some(Up),
        _ => None,
    }
}

fn map_inward_north_east_bend(dir: GridDirection) -> Option<GridDirection> {
    match dir {
        Up => Some(Right),
        Down => Some(Left),
        Left => Some(Down),
        Right => Some(Up),
    }
}

// symbol J
fn traverse_north_west_bend(dir: GridDirection) -> Option<GridDirection> {
    match dir {
        Down => Some(Left),
        Right => Some(Up),
        _ => None,
    }
}

fn map_inward_north_west_bend(dir: GridDirection) -> Option<GridDirection> {
    match dir {
        Up => Some(Left),
        Down => Some(Right),
        Left => Some(Up),
        Right => Some(Down),
    }
}

// symbol 7
fn traverse_south_west_bend(dir: GridDirection) -> Option<GridDirection> {
    match dir {
        Right => Some(Down),
        Up => Some(Left),
        _ => None,
    }
}

fn map_inward_south_west_bend(dir: GridDirection) -> Option<GridDirection> {
    match dir {
        Up => Some(Right),
        Down => Some(Left),
        Left => Some(Down),
        Right => Some(Up),
    }
}

// symbol F
fn traverse_south_east_bend(dir: GridDirection) -> Option<GridDirection> {
    match dir {
        Left => Some(Down),
        Up => Some(Right),
        _ => None,
    }
}

fn map_inward_south_east_bend(dir: GridDirection) -> Option<GridDirection> {
    match dir {
        Up => Some(Left),
        Down => Some(Right),
        Left => Some(Up),
        Right => Some(Down),
    }
}
