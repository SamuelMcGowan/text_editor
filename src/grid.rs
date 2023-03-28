use std::ops::{Index, IndexMut};

pub struct Grid<T> {
    data: Box<[T]>,

    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    pub fn filled(width: usize, height: usize, elem: T) -> Self
    where
        T: Clone,
    {
        let size = width
            .checked_mul(height)
            .expect("width * height overflowed");

        let data = vec![elem; size].into_boxed_slice();

        Self {
            data,

            width,
            height,
        }
    }

    pub fn from_iter(width: usize, height: usize, iter: impl IntoIterator<Item = T>) -> Self {
        let len = width
            .checked_mul(height)
            .expect("width * height overflowed");

        let mut data_vec = Vec::with_capacity(len);
        data_vec.extend(iter.into_iter().take(len));

        assert_eq!(data_vec.len(), len, "iterator is too small to fill array");

        Self {
            data: data_vec.into_boxed_slice(),

            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        let index = self.index(x, y)?;
        self.data.get(index)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        let index = self.index(x, y)?;
        self.data.get_mut(index)
    }

    pub fn blit(&mut self, x: usize, y: usize, grid: &Grid<T>)
    where
        T: Clone,
    {
        for (x, grid_x) in (x..self.width).zip(0..grid.height) {
            for (y, grid_y) in (y..self.height).zip(0..grid.height) {
                self[[x, y]] = grid[[grid_x, grid_y]].clone();
            }
        }
    }

    fn index(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y > self.height {
            return None;
        }

        let index = y * self.width + x;

        Some(index)
    }
}

impl<T> Index<[usize; 2]> for Grid<T> {
    type Output = T;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        self.get(index[0], index[1]).expect("out of bounds")
    }
}

impl<T> IndexMut<[usize; 2]> for Grid<T> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        self.get_mut(index[0], index[1]).expect("out of bounds")
    }
}

#[cfg(test)]
mod tests {
    use super::Grid;

    #[test]
    fn simple() {
        let mut arr = Grid::filled(10, 10, 0);
        assert_eq!(arr.len(), 10 * 10);

        arr[[0, 0]] = 12;
        arr[[9, 9]] = 14;

        assert_eq!(arr[[0, 0]], 12);
        assert_eq!(arr[[9, 9]], 14);
        assert!(arr.get(10, 10).is_none());
    }
}
