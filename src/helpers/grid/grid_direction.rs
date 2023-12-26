use std::ops::Neg;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GridDirection {
    Up,
    Down,
    Left,
    Right,
}
use GridDirection::*;

pub const GRID_DIRECTIONS: [GridDirection; 4] = [Up, Down, Left, Right];

/** Traits */
impl Neg for GridDirection {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}
