use std::ops::{Add, Neg, Sub};

/// A vector in orthogonal coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector {
    pub col: i16,
    pub half_row: i16,
}

/// A position in column-staggered coordinates.
///
/// ```
/// # use hex_maze::hex::{Position, Vector};
///
/// assert_eq!(Position { col: 0, row: 0 }, Vector { col: 0, half_row: 0 }.into());
/// assert_eq!(Position { col: 1, row: 0 }, Vector { col: 1, half_row: 1 }.into());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub col: i16,
    pub row: i16,
}

impl Vector {
    pub const ZERO: Self = Self {
        col: 0,
        half_row: 0,
    };

    pub const NORTH: Self = Self {
        col: 0,
        half_row: -2,
    };
    pub const NORTH_EAST: Self = Self {
        col: 1,
        half_row: -1,
    };
    pub const TWO_EAST: Self = Self {
        col: 2,
        half_row: 0,
    };
    pub const SOUTH_EAST: Self = Self {
        col: 1,
        half_row: 1,
    };
    pub const SOUTH: Self = Self {
        col: 0,
        half_row: 2,
    };
    pub const SOUTH_WEST: Self = Self {
        col: -1,
        half_row: 1,
    };
    pub const TWO_WEST: Self = Self {
        col: -2,
        half_row: 0,
    };
    pub const NORTH_WEST: Self = Self {
        col: -1,
        half_row: -1,
    };

    pub const DIRECTIONS: [Self; 6] = [
        Self::NORTH,
        Self::NORTH_EAST,
        Self::SOUTH_EAST,
        Self::SOUTH,
        Self::SOUTH_WEST,
        Self::NORTH_WEST,
    ];

    pub const fn on_grid(&self) -> bool {
        (self.col + self.half_row) % 2 == 0
    }

    pub const fn nearest_north(&self) -> Self {
        Self {
            col: self.col,
            half_row: if self.on_grid() {
                self.half_row
            } else {
                self.half_row - 1
            },
        }
    }
}

impl Add<Vector> for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            col: self.col + other.col,
            half_row: self.half_row + other.half_row,
        }
    }
}

impl Sub<Vector> for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            col: self.col - other.col,
            half_row: self.half_row - other.half_row,
        }
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            col: -self.col,
            half_row: -self.half_row,
        }
    }
}

impl From<Vector> for Position {
    fn from(screen: Vector) -> Self {
        Self {
            col: screen.col,
            row: if screen.col % 2 == 0 {
                screen.half_row / 2
            } else {
                (screen.half_row - 1) / 2
            },
        }
    }
}

impl From<Position> for Vector {
    fn from(grid: Position) -> Self {
        Self {
            col: grid.col,
            half_row: if grid.col % 2 == 0 {
                2 * grid.row
            } else {
                2 * grid.row + 1
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hex::{Position, Vector};

    #[test]
    fn test_position_from_vector() {
        assert_eq!(
            Position::from(Vector {
                col: 0,
                half_row: 0,
            }),
            Position { col: 0, row: 0 }
        );
        assert_eq!(
            Position::from(Vector {
                col: 1,
                half_row: 1,
            }),
            Position { col: 1, row: 0 }
        );
        assert_eq!(
            Position::from(Vector {
                col: 2,
                half_row: 0,
            }),
            Position { col: 2, row: 0 }
        );
        assert_eq!(
            Position::from(Vector {
                col: 3,
                half_row: 1,
            }),
            Position { col: 3, row: 0 }
        );
        assert_eq!(
            Position::from(Vector {
                col: 0,
                half_row: 2,
            }),
            Position { col: 0, row: 1 }
        );
        assert_eq!(
            Position::from(Vector {
                col: 1,
                half_row: 3,
            }),
            Position { col: 1, row: 1 }
        );
        assert_eq!(
            Position::from(Vector {
                col: 2,
                half_row: 2,
            }),
            Position { col: 2, row: 1 }
        );
        assert_eq!(
            Position::from(Vector {
                col: 3,
                half_row: 3,
            }),
            Position { col: 3, row: 1 }
        );
    }

    #[test]
    fn test_vector_from_position() {
        assert_eq!(
            Vector::from(Position { col: 0, row: 0 }),
            Vector {
                col: 0,
                half_row: 0,
            },
        );
        assert_eq!(
            Vector::from(Position { col: 1, row: 0 }),
            Vector {
                col: 1,
                half_row: 1,
            },
        );
        assert_eq!(
            Vector::from(Position { col: 2, row: 0 }),
            Vector {
                col: 2,
                half_row: 0,
            },
        );
        assert_eq!(
            Vector::from(Position { col: 3, row: 0 }),
            Vector {
                col: 3,
                half_row: 1,
            },
        );
    }
}
