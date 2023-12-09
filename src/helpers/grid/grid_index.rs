use super::Grid;

#[derive(Clone, Copy, PartialEq)]
pub struct GridIndex(pub usize, pub usize);

pub enum GridDirection {
    Up,
    Down,
    Left,
    Right,
}
use GridDirection::*;

/** Traits */
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

/** Public */
impl GridIndex {
    /// Returns the adjacent [`GridIndex`] in the specified [`GridDirection`].
    /// Return [`None`] if neighbour would be out-of-bounds of the [`Grid`].
    pub fn get_neighbour<T>(&self, dir: GridDirection, grid: &Grid<T>) -> Option<Self> {
        let mut index = self.clone();

        match dir {
            Up => {
                if index.0 > 0 {
                    index.0 -= 1;
                    return Some(index);
                }
            }
            Down => {
                if index.0 < grid.height() - 1 {
                    index.0 += 1;
                    return Some(index);
                }
            }
            Left => {
                if index.1 > 0 {
                    index.1 -= 1;
                    return Some(index);
                }
            }
            Right => {
                if index.1 < grid.height() - 1 {
                    index.1 += 1;
                    return Some(index);
                }
            }
        }

        None
    }

    /// Move [`GridIndex`] one step in a specified [`GridDirection`], if possible.
    /// Returns [`Err`] if step would be out-of-bounds of the [`Grid`].
    pub fn step<T>(&mut self, dir: GridDirection, grid: &Grid<T>) -> Result<Self, ()> {
        if let Some(dest) = self.get_neighbour(dir, grid) {
            *self = dest;
            Ok(*self)
        } else {
            Err(())
        }
    }

    /// Increment [`GridIndex`], wrapping to the next row if in the final column.
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
}

/** Private */
impl GridIndex {}
