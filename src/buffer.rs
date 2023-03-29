use std::ops::{Index, IndexMut};

use crate::style::Style;

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub c: char,
    pub style: Style,
}

impl Cell {
    fn char(c: char) -> Self {
        Self {
            c,
            style: Style::default(),
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            c: ' ',
            style: Style::default(),
        }
    }
}

pub struct Buffer {
    data: Box<[Cell]>,

    width: usize,
    height: usize,

    cursor: Option<(usize, usize)>,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self::new_inner(width, height, vec![], None)
    }

    pub fn resize_and_clear(&mut self, width: usize, height: usize) {
        if width != self.width || height != self.height {
            let data = std::mem::take(&mut self.data).into_vec();
            *self = Self::new_inner(width, height, data, self.cursor);
        } else {
            self.data.fill(Cell::default());
        }
        self.cursor = None;
    }

    fn new_inner(
        width: usize,
        height: usize,

        mut data: Vec<Cell>,

        cursor: Option<(usize, usize)>,
    ) -> Self {
        data.clear();

        let len = width
            .checked_mul(height)
            .expect("width * height overflowed");
        data.extend(std::iter::repeat(Cell::default()).take(len));

        Self {
            data: data.into_boxed_slice(),
            width,
            height,
            cursor,
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

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn as_slice(&self) -> &[Cell] {
        &self.data
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        let index = self.index(x, y)?;
        self.data.get(index)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        let index = self.index(x, y)?;
        self.data.get_mut(index)
    }

    pub fn blit(&mut self, x: usize, y: usize, buf: &Buffer, set_cursor: bool) {
        for (x, buf_x) in (x..self.width).zip(0..buf.height) {
            for (y, buf_y) in (y..self.height).zip(0..buf.height) {
                self[[x, y]] = buf[[buf_x, buf_y]];
            }
        }

        if set_cursor {
            self.cursor = buf.cursor;
        }
    }

    pub fn set_cursor(&mut self, cursor: Option<(usize, usize)>) {
        self.cursor = cursor;
    }

    pub fn cursor(&self) -> Option<(usize, usize)> {
        self.cursor
    }

    fn index(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y > self.height {
            return None;
        }

        let index = y * self.width + x;

        Some(index)
    }
}

impl Index<[usize; 2]> for Buffer {
    type Output = Cell;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        self.get(index[0], index[1]).expect("out of bounds")
    }
}

impl IndexMut<[usize; 2]> for Buffer {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        self.get_mut(index[0], index[1]).expect("out of bounds")
    }
}

#[cfg(test)]
mod tests {
    use super::{Buffer, Cell};

    #[test]
    fn simple() {
        let b = Cell::char('b');
        let c = Cell::char('c');

        let mut arr = Buffer::new(10, 10);
        assert_eq!(arr.len(), 10 * 10);

        arr[[0, 0]] = b;
        arr[[9, 9]] = c;

        assert_eq!(arr[[0, 0]].c, 'b');
        assert_eq!(arr[[9, 9]].c, 'c');
        assert!(arr.get(10, 10).is_none());
    }
}
