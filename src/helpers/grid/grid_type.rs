use super::{GridDirection, GridIndex};
use GridDirection::*;

use super::GRID_DIRECTIONS;

use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
    str::FromStr,
};

pub struct Grid<T>(Vec<Vec<T>>);

/** Traits */

impl<T> Index<GridIndex> for Grid<T> {
    type Output = T;

    fn index(&self, index: GridIndex) -> &Self::Output {
        let Grid(rows) = self;
        let row = &rows[index.0];
        &row[index.1]
    }
}

impl<T> IndexMut<GridIndex> for Grid<T> {
    fn index_mut(&mut self, index: GridIndex) -> &mut Self::Output {
        let Grid(rows) = self;
        let row = &mut rows[index.0];
        &mut row[index.1]
    }
}

impl FromStr for Grid<char> {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars_iter = s.lines().map(|line| line.chars().collect());
        let lines_iter = chars_iter.collect();
        Ok(Grid(lines_iter))
    }
}

/** Public */
impl<T> Grid<T> {
    pub fn width(&self) -> usize {
        let Grid(rows) = self;
        rows.first().unwrap().len()
    }

    pub fn height(&self) -> usize {
        let Grid(rows) = self;
        rows.len()
    }

    pub fn in_bounds(&self, index: &GridIndex) -> bool {
        let i_valid = index.0 < self.height();
        let j_valid = index.1 < self.width();
        i_valid && j_valid
    }

    pub fn adjacent_indices(&self, index: GridIndex) -> HashMap<GridDirection, Option<GridIndex>> {
        let mut adjacent = HashMap::<GridDirection, Option<GridIndex>>::new();
        for dir in GRID_DIRECTIONS {
            let neighbour = index.get_neighbour(dir, self);
            adjacent.insert(dir, neighbour);
        }

        adjacent
    }

    pub fn surrounding_indices(&self, index: GridIndex) -> impl Iterator<Item = GridIndex> + '_ {
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

        let indices = all_steps.into_iter().filter_map(move |steps| {
            let mut neighbour = index;
            for dir in steps.into_iter().flatten() {
                neighbour = neighbour.get_neighbour(dir, self)?;
            }
            Some(neighbour)
        });

        indices
    }
}

impl<T: PartialEq> Grid<T> {
    /// Returns the [`GridIndex`] of the first instance of 'target' in the [`Grid`].
    pub fn find(&self, target: T) -> Option<GridIndex> {
        let mut index = GridIndex(0, 0);

        while self[index] != target {
            index.increment(self).ok()?;
        }

        Some(index)
    }
}

/** Private */
impl<T> Grid<T> {}
