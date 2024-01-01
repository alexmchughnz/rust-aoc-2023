use super::GridIndex;

use std::{
    fmt::Debug,
    iter::once,
    ops::{Index, IndexMut},
    str::FromStr,
};

#[derive(Clone)]
pub struct Grid<T>(Vec<Vec<T>>);

/** Traits */
impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, indices: (usize, usize)) -> &Self::Output {
        let Grid(rows) = self;
        let row = &rows[indices.0];
        &row[indices.1]
    }
}

impl<T> Index<GridIndex<'_, T>> for Grid<T> {
    type Output = T;

    fn index(&self, grid_index: GridIndex<T>) -> &Self::Output {
        self.index(grid_index.indices)
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, indices: (usize, usize)) -> &mut Self::Output {
        let Grid(rows) = self;
        let row = &mut rows[indices.0];
        &mut row[indices.1]
    }
}

impl<T> IndexMut<GridIndex<'_, T>> for Grid<T> {
    fn index_mut(&mut self, grid_index: GridIndex<T>) -> &mut Self::Output {
        self.index_mut(grid_index.indices)
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

impl Debug for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Grid(rows) = self;
        let grid_str = rows
            .into_iter()
            .flat_map(|row| row.into_iter().chain(once(&'\n')))
            .collect::<String>();

        f.write_str("\n")?;
        f.write_str(&grid_str)
    }
}

/** Public */
impl<T> Grid<T> {
    pub fn iter(&self) -> impl Iterator<Item = (GridIndex<T>, &T)> + '_ {
        let Grid(rows) = self;
        rows.iter().flatten().enumerate().map(|(i, item)| {
            let mut index = GridIndex {
                indices: (0, 0),
                grid: self,
            };
            for _ in 0..i {
                index.increment().ok();
            }
            (index, item)
        })
    }

    pub fn make_index(&self, i: usize, j: usize) -> GridIndex<T> {
        GridIndex {
            grid: self,
            indices: (i, j),
        }
    }
}

impl<T: PartialEq> Grid<T> {
    /// Returns the [`GridIndex`] of the first instance of 'target' in the [`Grid`].
    pub fn find(&self, target: T) -> Option<GridIndex<T>> {
        let mut index = GridIndex {
            indices: (0, 0),
            grid: self,
        };

        while self[index] != target {
            index.increment().ok()?;
        }

        Some(index)
    }
}

/** Private */
impl<T> Grid<T> {
    pub fn width(&self) -> usize {
        let Grid(rows) = self;
        rows.first().unwrap().len()
    }

    pub fn height(&self) -> usize {
        let Grid(rows) = self;
        rows.len()
    }
}
