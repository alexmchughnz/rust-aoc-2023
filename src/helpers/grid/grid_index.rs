use std::collections::HashMap;

use super::{Grid, GridDirection};
use GridDirection::*;

pub struct GridIndex<'a, T> {
    pub(super) grid: &'a Grid<T>,
    pub(super) indices: (usize, usize),
}

/** Traits */
impl<T> Copy for GridIndex<'_, T> {}

impl<T> Clone for GridIndex<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> PartialEq for GridIndex<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.indices.eq(&other.indices)
    }
}

/** Public */
impl<T> GridIndex<'_, T> {
    /// Returns the adjacent [`GridIndex`] in the specified [`GridDirection`].
    /// Return [`None`] if neighbour would be out-of-bounds of the [`Grid`].
    pub fn neighbour(&self, dir: GridDirection) -> Option<Self> {
        let mut dest = *self;

        match dir {
            Up => {
                if dest.indices.0 > 0 {
                    dest.indices.0 -= 1;
                    return Some(dest);
                }
            }
            Down => {
                if dest.indices.0 < self.grid.height() - 1 {
                    dest.indices.0 += 1;
                    return Some(dest);
                }
            }
            Left => {
                if dest.indices.1 > 0 {
                    dest.indices.1 -= 1;
                    return Some(dest);
                }
            }
            Right => {
                if dest.indices.1 < self.grid.width() - 1 {
                    dest.indices.1 += 1;
                    return Some(dest);
                }
            }
        }

        None
    }

    pub fn all_neighbours(&self) -> HashMap<GridDirection, Option<GridIndex<T>>> {
        let mut adjacent = HashMap::new();
        for dir in super::GRID_DIRECTIONS {
            let neighbour = self.neighbour(dir);
            adjacent.insert(dir, neighbour);
        }

        adjacent
    }

    pub fn surrounding(&self) -> impl Iterator<Item = GridIndex<T>> {
        let all_steps = [
            [Some(Up), Some(Left)],
            [Some(Up), None],
            [Some(Up), Some(Right)],
            [Some(Left), None],
            [Some(Right), None],
            [Some(Down), Some(Left)],
            [Some(Down), None],
            [Some(Down), Some(Right)],
        ];

        all_steps.into_iter().filter_map(|steps| {
            let mut neighbour = *self;
            for dir in steps.into_iter().flatten() {
                neighbour = neighbour.neighbour(dir)?;
            }
            Some(neighbour)
        })
    }
    /// Move [`GridIndex`] one step in a specified [`GridDirection`], if possible.
    /// Returns [`Err`] if step would be out-of-bounds of the [`Grid`].
    pub fn step(&mut self, dir: GridDirection) -> Result<&mut Self, &'static str> {
        if let Some(dest) = self.neighbour(dir) {
            *self = dest;
            Ok(self)
        } else {
            Err("stepped out of bounds")
        }
    }

    /// Increment [`GridIndex`], wrapping to the next row if in the final column.
    /// Returns [`Err`] if [`GridIndex`] is already at end of [`Grid`].
    pub fn increment(&mut self) -> Result<&mut Self, &'static str> {
        match self.step(Right) {
            Ok(_) => (),
            Err(_) => {
                self.indices.1 = 0; // Return to start of row.
                self.step(Down)?;
            }
        }

        Ok(self)
    }
}

/** Private */
impl<T> GridIndex<'_, T> {}
