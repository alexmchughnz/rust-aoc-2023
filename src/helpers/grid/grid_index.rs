use super::{Grid, GridDirection};
use GridDirection::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct GridIndex(pub usize, pub usize);

/** Traits */
impl<T> From<(T, T)> for GridIndex
where
    T: Into<usize>,
{
    fn from(value: (T, T)) -> Self {
        GridIndex(value.0.into(), value.1.into())
    }
}

/** Public */
impl GridIndex {
    /// Returns the adjacent [`GridIndex`] in the specified [`GridDirection`].
    /// Return [`None`] if neighbour would be out-of-bounds of the [`Grid`].
    pub fn get_neighbour<T>(&self, dir: GridDirection, grid: &Grid<T>) -> Option<Self> {
        let mut index = *self;

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
                if index.1 < grid.width() - 1 {
                    index.1 += 1;
                    return Some(index);
                }
            }
        }

        None
    }

    /// Move [`GridIndex`] one step in a specified [`GridDirection`], if possible.
    /// Returns [`Err`] if step would be out-of-bounds of the [`Grid`].
    pub fn step<T>(
        &mut self,
        dir: GridDirection,
        grid: &Grid<T>,
    ) -> Result<&mut Self, &'static str> {
        if let Some(dest) = self.get_neighbour(dir, grid) {
            *self = dest;
            Ok(self)
        } else {
            Err("stepped out of bounds")
        }
    }

    /// Increment [`GridIndex`], wrapping to the next row if in the final column.
    /// Returns [`Err`] if [`GridIndex`] is already at end of [`Grid`].
    pub fn increment<T>(&mut self, grid: &Grid<T>) -> Result<&mut Self, &'static str> {
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
            Ok(self)
        } else {
            Err("stepped over end of Grid")
        }
    }
}

/** Private */
impl GridIndex {}
