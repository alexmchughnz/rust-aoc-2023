use super::{GridDirection, GridIndex};

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
            let neighbour = index.clone().step(dir, self).ok();
            adjacent.insert(dir, neighbour);
        }

        adjacent
    }

    pub fn surrounding_indices(&self, index: GridIndex) -> impl Iterator<Item = GridIndex> + '_ {
        let i = index.0 as i64;
        let j = index.1 as i64;
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

        all.into_iter()
            .filter_map(|coords| GridIndex::try_from(coords).ok())
            .filter(|index| self.in_bounds(index))
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
