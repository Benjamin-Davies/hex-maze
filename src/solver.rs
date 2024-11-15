use std::collections::BinaryHeap;

use crate::{
    grid::HexGrid,
    hex::{Position, Vector},
    maze::Maze,
    terminal::{CLEAR_COLOR, GREEN, RED},
};

/// Solves a maze using A*.
pub struct Solver {
    pub maze: Maze,
    pub is_done: bool,
    pub path: Vec<Vector>,
    goal: Vector,
    unvisited: BinaryHeap<Unvisited>,
    distances: HexGrid<i32>,
}

#[derive(Debug, PartialEq, Eq)]
struct Unvisited {
    score: i32,
    distance: i32,
    position: Vector,
}

impl PartialOrd for Unvisited {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Unvisited {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.score, self.distance, self.position)
            .cmp(&(other.score, other.distance, other.position))
            // Reverse so that the smallest distance is at the top of the max-heap
            .reverse()
    }
}

impl Solver {
    pub fn new(maze: Maze) -> Self {
        let position = Vector::ZERO;
        let goal = Vector::from(Position {
            col: maze.cells.cols() as i16 - 1,
            row: maze.cells.rows() as i16 - 1,
        });

        let mut unvisited = BinaryHeap::new();
        unvisited.push(Unvisited {
            score: 0,
            distance: 0,
            position,
        });

        let mut distances = HexGrid::new_with(maze.cells.cols(), maze.cells.rows(), |_| i32::MAX);
        distances[position] = 0;

        Self {
            maze,
            is_done: false,
            path: Vec::new(),
            goal,
            unvisited,
            distances,
        }
    }

    pub fn step(&mut self) {
        if let Some(Unvisited {
            score: _,
            distance,
            position,
        }) = self.unvisited.pop()
        {
            if position == self.goal {
                self.fill_path();
                self.is_done = true;
                return;
            }

            for dir in Vector::DIRECTIONS {
                let neighbor = position + dir;
                if self.maze.wall_between(position, neighbor) {
                    continue;
                }

                let new_distance = distance + dir.length();
                if new_distance < self.distances[neighbor] {
                    self.distances[neighbor] = new_distance;
                    self.unvisited.push(Unvisited {
                        score: new_distance + (neighbor - self.goal).length(),
                        distance: new_distance,
                        position: neighbor,
                    });
                }
            }
        }

        for pos in self.maze.cells.indices() {
            self.maze.cells[pos].background = CLEAR_COLOR;
        }
        self.maze.cells[self.goal].background = RED;
        for unvisited in &self.unvisited {
            self.maze.cells[unvisited.position].background = GREEN;
        }
    }

    fn fill_path(&mut self) {
        let mut position = self.goal;
        self.path.push(position);

        while self.distances[position] > 0 {
            let next_position = Vector::DIRECTIONS
                .into_iter()
                .map(|dir| position + dir)
                .filter(|&neighbor| !self.maze.wall_between(position, neighbor))
                .min_by_key(|&neighbor| self.distances[neighbor])
                .unwrap();
            self.path.push(next_position);
            position = next_position;
        }
    }
}
