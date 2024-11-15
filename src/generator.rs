use rand::Rng;

use crate::{
    grid::HexGrid,
    hex::Vector,
    maze::Maze,
    terminal::{Terminal, BLUE, CLEAR_COLOR, LIGHT_GREY},
};

pub struct Generator {
    pub maze: Maze,
    head: Vector,
    tail: Vec<Vector>,
    visited: HexGrid<bool>,
}

impl Generator {
    pub fn new(term: &Terminal) -> Self {
        let maze = Maze::new(term);
        let head = Vector {
            col: 0,
            half_row: 0,
        };
        let tail = Vec::new();
        let visited = HexGrid::new_with(maze.cells.cols(), maze.cells.rows(), |_| false);
        Self {
            maze,
            head,
            tail,
            visited,
        }
    }

    pub fn step(&mut self) {
        self.visited[self.head] = true;
        if let Some(next) = self.pick_next_cell() {
            self.maze.set_wall_between(self.head, next, false);
            self.tail.push(self.head);
            self.head = next;
        } else if let Some(prev) = self.tail.pop() {
            self.head = prev;
        }

        for pos in self.maze.cells.indices() {
            self.maze.cells[pos].background = CLEAR_COLOR;
        }
        for &pos in &self.tail {
            self.maze.cells[pos].background = LIGHT_GREY;
        }
        self.maze.cells[self.head].background = BLUE;
    }

    pub fn pick_next_cell(&self) -> Option<Vector> {
        let candidates = Vector::DIRECTIONS
            .into_iter()
            .map(|dir| dir + self.head)
            .filter(|&neighbor| self.maze.cells.contains(neighbor) && !self.visited[neighbor])
            .collect::<Vec<_>>();
        if candidates.is_empty() {
            None
        } else {
            candidates
                .get(rand::thread_rng().gen_range(0..candidates.len()))
                .copied()
        }
    }
}
