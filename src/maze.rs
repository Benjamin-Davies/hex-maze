use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::{grid::HexGrid, hex::Vector, terminal::Terminal};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Maze {
    cells: HexGrid<Cell>,
}

/// Each cell keeps track of its north-east, south, and north-west walls.
/// The other three walls belong to neighboring cells.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    north_east: bool,
    south: bool,
    north_west: bool,
    background: u8,
}

impl Distribution<Cell> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cell {
        Cell {
            north_east: rng.gen(),
            south: rng.gen(),
            north_west: rng.gen(),
            background: rng.gen::<u8>() % 16,
        }
    }
}

impl Maze {
    pub fn new(term: &Terminal) -> Self {
        let (term_width, term_height) = term.size();
        if term_width < 11 || term_height < 7 {
            return Self::empty();
        }

        let cols = (term_width - 3) / 8;
        let rows = (term_height - 3) / 4;
        Self {
            cells: HexGrid::new_with(cols, rows, |_| rand::random()),
        }
    }

    pub fn empty() -> Maze {
        Self {
            cells: HexGrid::empty(),
        }
    }

    pub fn copy_from(&mut self, other: &Maze) {
        self.cells.copy_from(&other.cells);
    }

    pub fn contains(&self, coords: Vector) -> bool {
        self.cells.contains(coords)
    }

    pub fn wall_between(&self, a: Vector, b: Vector) -> bool {
        let a_inside = self.contains(a);
        let b_inside = self.contains(b);
        if !(a_inside || b_inside) {
            return false;
        }
        if (a_inside && !b_inside) || (!a_inside && b_inside) {
            return true;
        }

        match b - a {
            Vector::NORTH => self.cells[b].south,
            Vector::NORTH_EAST => self.cells[a].north_east,
            Vector::SOUTH_EAST => self.cells[b].north_west,
            Vector::SOUTH => self.cells[a].south,
            Vector::SOUTH_WEST => self.cells[b].north_east,
            Vector::NORTH_WEST => self.cells[a].north_west,
            _ => false,
        }
    }

    fn horizontal_wall_at(&self, coords: Vector) -> bool {
        let above = coords + Vector::NORTH;
        let below = coords;
        self.wall_between(above, below)
    }

    fn vertical_wall_at(&self, coords: Vector) -> bool {
        let (left, right) = if coords.on_grid() {
            (coords + Vector::NORTH_WEST, coords)
        } else {
            let coords = coords.nearest_north();
            (coords + Vector::SOUTH_WEST, coords)
        };
        self.wall_between(left, right)
    }

    fn vertex_wall_at(&self, coords: Vector) -> bool {
        let (a, b, c) = if coords.on_grid() {
            (
                coords + Vector::ZERO,
                coords + Vector::NORTH_WEST,
                coords + Vector::NORTH,
            )
        } else {
            let coords = coords.nearest_north();
            (
                coords + Vector::ZERO,
                coords + Vector::SOUTH_WEST,
                coords + Vector::NORTH_WEST,
            )
        };
        self.wall_between(a, b) || self.wall_between(b, c) || self.wall_between(c, a)
    }

    pub fn draw(&self, term: &mut Terminal) {
        term.sgr().reset();
        if self.cells.is_empty() {
            return;
        }

        let height = self.cells.rows() * 4 + 3;
        for y in 0..height {
            let half_row = y as i16 / 2;
            if y % 2 == 0 {
                let indent = 2 - y % 4;
                term.goto(indent, y);

                for col in 0..self.cells.cols() as i16 {
                    let coords = Vector { col, half_row };
                    if self.vertex_wall_at(coords) {
                        term.write("*");
                    } else {
                        term.write(" ");
                    }
                    if coords.on_grid() {
                        if self.horizontal_wall_at(coords) {
                            term.write(" --- ");
                        } else {
                            term.write("     ");
                        }
                    } else {
                        if let Some(cell) = self.cells.get(coords.nearest_north()) {
                            term.write(" ");
                            term.sgr().bg(cell.background);
                            term.write("       ");
                            term.sgr().reset();
                            term.write(" ");
                        } else {
                            term.write("         ");
                        }
                    }
                }

                let col = self.cells.cols() as i16;
                let coords = Vector { col, half_row };
                term.sgr().reset();
                if self.vertex_wall_at(coords) {
                    term.write("*");
                } else {
                    term.write(" ");
                }
            } else {
                term.goto(1, y);

                for col in 0..self.cells.cols() as i16 {
                    let coords = Vector { col, half_row };
                    term.sgr().reset();
                    if self.vertical_wall_at(coords) {
                        if coords.on_grid() {
                            term.write("/");
                        } else {
                            term.write("\\");
                        }
                    } else {
                        term.write(" ");
                    }

                    if let Some(cell) = self.cells.get(coords.nearest_north()) {
                        term.write(" ");
                        term.sgr().bg(cell.background);
                        term.write("     ");
                        term.sgr().reset();
                        term.write(" ");
                    } else {
                        term.write("       ");
                    }
                }

                let col = self.cells.cols() as i16;
                let coords = Vector { col, half_row };
                term.sgr().reset();
                if self.vertical_wall_at(coords) {
                    if coords.on_grid() {
                        term.write("/");
                    } else {
                        term.write("\\");
                    }
                } else {
                    term.write(" ");
                }
            }
        }
    }
}
