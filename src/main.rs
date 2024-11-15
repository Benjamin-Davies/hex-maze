use std::{time::Duration, vec::IntoIter};

use hex_maze::{
    generator::Generator,
    hex::Vector,
    maze::Maze,
    solver::Solver,
    terminal::{Terminal, CLEAR_COLOR, CTRL_C, ESC, GREEN},
};

fn main() {
    let mut term = Terminal::new();

    let mut state = State::new(&term);
    'main_loop: while !term.should_exit() {
        state.step();

        state.maze().draw(&mut term);
        term.flush();

        let mut timeout = Duration::from_millis(16);
        while term.poll(timeout) > 0 {
            match term.read() {
                CTRL_C | ESC | b'q' => break 'main_loop,
                b'r' => {
                    term.clear();
                    state = State::new(&term);
                }
                _ => {}
            }

            timeout = Duration::ZERO;
        }
    }
}

enum State {
    Generating(Generator),
    Solving(Solver),
    Done(Maze, IntoIter<Vector>),
}

impl State {
    pub fn new(term: &Terminal) -> Self {
        Self::Generating(Generator::new(term))
    }

    pub fn step(&mut self) {
        match self {
            Self::Generating(generator) => {
                generator.step();

                if generator.is_done {
                    let maze = generator.maze.clone();
                    *self = Self::Solving(Solver::new(maze));
                }
            }
            Self::Solving(solver) => {
                solver.step();

                if solver.is_done {
                    let mut maze = solver.maze.clone();
                    for pos in maze.cells.indices() {
                        maze.cells[pos].background = CLEAR_COLOR;
                    }
                    let path = solver.path.clone();
                    *self = Self::Done(maze, path.into_iter());
                }
            }
            Self::Done(maze, path) => {
                if let Some(pos) = path.next() {
                    maze.cells[pos].background = GREEN;
                }
            }
        }
    }

    pub fn maze(&self) -> &Maze {
        match self {
            Self::Generating(generator) => &generator.maze,
            Self::Solving(solver) => &solver.maze,
            Self::Done(maze, _) => maze,
        }
    }
}
