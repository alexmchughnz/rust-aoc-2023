use std::{ops::Deref, str::FromStr};

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

pub struct Grid<T>(Vec<Vec<T>>);

impl<T> Deref for Grid<T> {
    type Target = Vec<Vec<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
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
    pub fn in_bounds(&self, coords: &(i64, i64)) -> bool {
        let i_valid = (0 <= coords.0) && (coords.0 < self.height() as i64);
        let j_valid = (0 <= coords.1) && (coords.1 < self.width() as i64);
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
    }
}
