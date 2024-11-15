use std::ops::{Index, IndexMut};

use crate::hex::Position;

/// A hexagonal grid that is indexed by column-staggered coordinates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HexGrid<T> {
    cols: u16,
    rows: u16,
    cells: Vec<T>,
}

impl<T> HexGrid<T> {
    pub fn empty() -> Self {
        Self {
            cols: 0,
            rows: 0,
            cells: Vec::new(),
        }
    }

    pub fn new_with(cols: u16, rows: u16, mut default: impl FnMut(Position) -> T) -> Self {
        let mut cells = Vec::with_capacity(cols as usize * rows as usize);
        for row in 0..rows as i16 {
            for col in 0..cols as i16 {
                cells.push(default(Position { col, row }));
            }
        }

        Self { cols, rows, cells }
    }

    pub fn copy_from(&mut self, other: &HexGrid<T>)
    where
        T: Copy,
    {
        if self.cols == other.cols && self.rows == other.rows {
            self.cells.copy_from_slice(&other.cells);
        } else {
            *self = other.clone();
        }
    }

    pub fn is_empty(&self) -> bool {
        self.cols == 0 || self.rows == 0
    }

    pub fn cols(&self) -> u16 {
        self.cols
    }

    pub fn rows(&self) -> u16 {
        self.rows
    }

    pub fn contains(&self, coords: impl Into<Position>) -> bool {
        let coords: Position = coords.into();
        coords.col >= 0
            && coords.col < self.cols as i16
            && coords.row >= 0
            && coords.row < self.rows as i16
    }

    fn index(&self, coords: impl Into<Position>) -> Option<usize> {
        let coords = coords.into();
        if self.contains(coords) {
            Some(coords.row as usize * self.cols as usize + coords.col as usize)
        } else {
            None
        }
    }

    pub fn get(&self, coords: impl Into<Position>) -> Option<&T> {
        self.index(coords).map(|index| &self.cells[index])
    }

    pub fn get_mut(&mut self, coords: impl Into<Position>) -> Option<&mut T> {
        self.index(coords).map(move |index| &mut self.cells[index])
    }

    pub fn indices(&self) -> impl Iterator<Item = Position> {
        let cols = self.cols as i16;
        let rows = self.rows as i16;
        (0..cols).flat_map(move |col| (0..rows).map(move |row| Position { col, row }))
    }
}

impl<I, T> Index<I> for HexGrid<T>
where
    I: Into<Position>,
{
    type Output = T;

    fn index(&self, index: I) -> &T {
        self.get(index).expect("index out of bounds")
    }
}

impl<I, T> IndexMut<I> for HexGrid<T>
where
    I: Into<Position>,
{
    fn index_mut(&mut self, index: I) -> &mut T {
        self.get_mut(index).expect("index out of bounds")
    }
}
