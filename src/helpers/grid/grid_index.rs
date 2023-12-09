use super::Grid;

#[derive(Clone, Copy, PartialEq)]
pub struct GridIndex(pub usize, pub usize);

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
    pub fn up<T>(self, _grid: &Grid<T>) -> Option<Self> {
        let mut index = self.clone();

        if index.0 > 0 {
            index.0 -= 1;
            Some(index)
        } else {
            None
        }
    }

    pub fn down<T>(self, grid: &Grid<T>) -> Option<Self> {
        let mut index = self.clone();

        if index.0 < grid.height() - 1 {
            index.0 += 1;
            Some(index)
        } else {
            None
        }
    }

    pub fn left<T>(self, _grid: &Grid<T>) -> Option<Self> {
        let mut index = self.clone();

        if index.1 > 0 {
            index.1 -= 1;
            Some(index)
        } else {
            None
        }
    }

    pub fn right<T>(self, grid: &Grid<T>) -> Option<Self> {
        let mut index = self.clone();

        if index.1 < grid.width() - 1 {
            index.1 += 1;
            Some(index)
        } else {
            None
        }
    }

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
    pub fn move_up<T>(&mut self, grid: &Grid<T>) -> Result<Self, ()> {
        if let Some(up) = self.up(&grid) {
            *self = up;
            Ok(*self)
        } else {
            Err(())
        }
    }

    pub fn move_down<T>(&mut self, grid: &Grid<T>) -> Result<Self, ()> {
        if let Some(down) = self.down(&grid) {
            *self = down;
            Ok(*self)
        } else {
            Err(())
        }
    }

    pub fn move_left<T>(&mut self, grid: &Grid<T>) -> Result<Self, ()> {
        if let Some(left) = self.left(&grid) {
            *self = left;
            Ok(*self)
        } else {
            Err(())
        }
    }

    pub fn move_right<T>(&mut self, grid: &Grid<T>) -> Result<Self, ()> {
        if let Some(right) = self.right(&grid) {
            *self = right;
            Ok(*self)
        } else {
            Err(())
        }
    }
}

/** Private */
impl GridIndex {}
