use aoc2023::helpers::grid::GridDirection;
use aoc2023::helpers::grid::GridDirection::*;

fn traverse_pipe(movement: GridDirection, symbol: char) -> Option<GridDirection> {
    match symbol {
        '|' => vertical_pipe(movement),
        '-' => horizontal_pipe(movement),
        'L' => north_east_bend(movement),
        'J' => north_west_bend(movement),
        '7' => south_west_bend(movement),
        'F' => south_east_bend(movement),
        _ => panic!("should not traverse a non-tile character"),
    }
}

// symbol |
fn vertical_pipe(dir: GridDirection) -> Option<GridDirection> {
    match dir {
        Up => Some(Up),
        Down => Some(Down),
        _ => None,
    }
}

// symbol -
fn horizontal_pipe(dir: GridDirection) -> Option<GridDirection> {
    match dir {
        Left => Some(Left),
        Right => Some(Right),
        _ => None,
    }
}

// symbol L
fn north_east_bend(dir: GridDirection) -> Option<GridDirection> {
    match dir {
        Down => Some(Right),
        Left => Some(Up),
        _ => None,
    }
}

// symbol J
fn north_west_bend(dir: GridDirection) -> Option<GridDirection> {
    match dir {
        Down => Some(Left),
        Left => Some(Up),
        _ => None,
    }
}

// symbol 7
fn south_west_bend(dir: GridDirection) -> Option<GridDirection> {
    match dir {
        Right => Some(Down),
        Up => Some(Left),
        _ => None,
    }
}

// symbol F
fn south_east_bend(dir: GridDirection) -> Option<GridDirection> {
    match dir {
        Left => Some(Down),
        Up => Some(Right),
        _ => None,
    }
}
