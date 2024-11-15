use crate::{
    grid::HexGrid,
    hex::Vector,
    terminal::{Terminal, CLEAR_COLOR},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Maze {
    pub cells: HexGrid<Cell>,
}

/// Each cell keeps track of its north-east, south, and north-west walls.
/// The other three walls belong to neighboring cells.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub north_east: bool,
    pub south: bool,
    pub north_west: bool,
    pub background: u8,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            north_east: true,
            south: true,
            north_west: true,
            background: CLEAR_COLOR,
        }
    }
}

impl Maze {
    pub fn new(term: &Terminal) -> Self {
        let (term_width, term_height) = term.size();
        if term_width < 11 || term_height < 7 {
            return Self::empty();
        }

        let cols = (term_width - 1) / 4;
        let rows = (term_height - 2) / 2;
        Self {
            cells: HexGrid::new_with(cols, rows, |_| Cell::default()),
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

    pub fn wall_between(&self, a: Vector, b: Vector) -> bool {
        let a_inside = self.cells.contains(a);
        let b_inside = self.cells.contains(b);
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

    pub fn set_wall_between(&mut self, a: Vector, b: Vector, wall: bool) {
        match b - a {
            Vector::NORTH => self.cells[b].south = wall,
            Vector::NORTH_EAST => self.cells[a].north_east = wall,
            Vector::SOUTH_EAST => self.cells[b].north_west = wall,
            Vector::SOUTH => self.cells[a].south = wall,
            Vector::SOUTH_WEST => self.cells[b].north_east = wall,
            Vector::NORTH_WEST => self.cells[a].north_west = wall,
            _ => {}
        }
    }

    fn horizontal_wall_at(&self, coords: Vector) -> bool {
        let coords = coords.nearest_north();
        let above = coords;
        let below = coords + Vector::SOUTH;
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

    pub fn draw(&self, term: &mut Terminal) {
        term.sgr().reset();
        if self.cells.is_empty() {
            return;
        }

        let height = self.cells.rows() * 2 + 2;
        for y in 0..height {
            let half_row = y as i16 - 1;
            term.goto(0, y);

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
                    term.sgr().bg(cell.background);
                }
                if !coords.on_grid() && self.horizontal_wall_at(coords) {
                    term.write("___");
                } else {
                    term.write("   ");
                }
                term.sgr().reset();
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
