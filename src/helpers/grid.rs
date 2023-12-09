use std::{
    ops::{Deref, Index},
    str::FromStr,
};

#[derive(Clone, Copy)]
pub struct GridIndex(pub usize, pub usize);

impl<T> TryFrom<(T, T)> for GridIndex
where
    T: TryInto<usize>,
{
    type Error = T::Error;

    fn try_from(value: (T, T)) -> Result<Self, Self::Error> {
        let i: usize = value.0.try_into()?;
        let j: usize = value.1.try_into()?;
        Ok(GridIndex(i, j))
    }
}

impl GridIndex {
    /// Increment [`GridIndex`], wrapping to new lines.
    /// Returns [`Err`] if [`GridIndex`] is already at end of [`Grid`].
    pub fn increment<T>(&mut self, grid: &Grid<T>) -> Result<Self, ()> {
        let mut i = self.0;
        let mut j = self.1;
        j += 1;
        if j >= grid.width() {
            j = 0;
            i += 1;
        }

        let index = GridIndex(i, j);
        if grid.in_bounds(&index) {
            *self = index;
            Ok(*self)
        } else {
            Err(())
        }
    }

    pub fn move_up<T>(&mut self, _grid: &Grid<T>) -> Result<Self, ()> {
        let i = &mut self.0;

        if *i > 0 {
            *i -= 1;
            Ok(*self)
        } else {
            Err(())
        }
    }

    pub fn move_down<T>(&mut self, grid: &Grid<T>) -> Result<Self, ()> {
        let i = &mut self.0;

        if *i < grid.height() - 1 {
            *i += 1;
            Ok(*self)
        } else {
            Err(())
        }
    }

    pub fn move_left<T>(&mut self, _grid: &Grid<T>) -> Result<Self, ()> {
        let j = &mut self.1;

        if *j > 0 {
            *j -= 1;
            Ok(*self)
        } else {
            Err(())
        }
    }

    pub fn move_right<T>(&mut self, grid: &Grid<T>) -> Result<Self, ()> {
        let j = &mut self.1;

        if *j < grid.width() - 1 {
            *j += 1;
            Ok(*self)
        } else {
            Err(())
        }
    }
}

pub struct Grid<T>(Vec<Vec<T>>);

impl<T> Deref for Grid<T> {
    // Workaround for using a 1-tuple of primitive type as a new type.
    // Using `Grid` in a deref-able context will automatically unpack the tuple.
    type Target = Vec<Vec<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Index<GridIndex> for Grid<T> {
    type Output = T;

    fn index(&self, index: GridIndex) -> &Self::Output {
        let row = &self.deref()[index.0];
        &row[index.1]
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

impl<T> Grid<T> {
    fn width(&self) -> usize {
        self.first().unwrap().len()
    }

    fn height(&self) -> usize {
        self.len()
    }
}

impl<T> Grid<T> {
    pub fn in_bounds(&self, index: &GridIndex) -> bool {
        let i_valid = index.0 < self.height();
        let j_valid = index.1 < self.width();
        i_valid && j_valid
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
