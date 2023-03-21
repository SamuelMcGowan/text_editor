use std::ops::{Index, IndexMut, Range};

struct Grid<T> {
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

    pub fn view(&mut self) -> GridView<T> {
        GridView {
            data: &mut self.data,
            row_width: self.width,
            row_span: 0..self.width,
        }
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data
    }
}

pub struct GridView<'a, T> {
    data: &'a mut [T],

    row_width: usize,
    row_span: Range<usize>,
}

impl<'a, T> GridView<'a, T> {
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        let index = self.index(x, y)?;
        self.data.get(index)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        let index = self.index(x, y)?;
        self.data.get_mut(index)
    }

    pub fn view(&mut self, row_span: Range<usize>, col_span: Range<usize>) -> Option<GridView<T>> {
        if row_span.end > self.width()
            || col_span.end > self.height()
            || row_span.is_empty()
            || col_span.is_empty()
        {
            return None;
        }

        Some(GridView {
            data: &mut self.data
                [(col_span.start * self.row_width)..(col_span.end * self.row_width)],
            row_width: self.row_width,

            row_span,
        })
    }

    /// The width of this slice.
    pub fn width(&self) -> usize {
        self.row_span.len()
    }

    /// The height of this slice.
    pub fn height(&self) -> usize {
        self.data.len() / self.row_width
    }

    /// The number of elements in this slice.
    pub fn len(&self) -> usize {
        self.width() * self.height()
    }

    fn index(&self, x: usize, y: usize) -> Option<usize> {
        let x = self.row_span.start.checked_add(x)?;

        if x > self.row_span.end || y > self.height() {
            return None;
        }

        let index = y * self.row_width + x;

        Some(index)
    }
}

impl<T> Index<[usize; 2]> for GridView<'_, T> {
    type Output = T;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        self.get(index[0], index[1]).expect("out of bounds")
    }
}

impl<T> IndexMut<[usize; 2]> for GridView<'_, T> {
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
        let mut view = arr.view();
        assert_eq!(view.len(), 10 * 10);

        view[[0, 0]] = 12;
        view[[9, 9]] = 14;

        assert_eq!(view[[0, 0]], 12);
        assert_eq!(view[[9, 9]], 14);
        assert!(view.get(10, 10).is_none());
    }

    #[test]
    fn view_mut() {
        let mut arr = Grid::filled(10, 10, 0);
        let mut view = arr.view();

        let mut view2 = view.view(1..9, 1..9).unwrap();
        assert_eq!(view2.len(), 8 * 8);

        view2[[0, 0]] = 12;
        view2[[7, 7]] = 14;
        assert!(view2.get(8, 8).is_none());

        assert_eq!(view[[1, 1]], 12);
        assert_eq!(view[[8, 8]], 14);
    }
}
