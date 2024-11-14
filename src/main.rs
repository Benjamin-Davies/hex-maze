use std::time::Duration;

use self::{
    maze::Maze,
    terminal::{Terminal, CTRL_C, ESC},
};

mod maze;
mod sys;
mod terminal;

fn main() {
    let mut term = Terminal::new();

    let mut maze;
    let mut last_maze = Maze::empty();
    'main_loop: while !term.should_exit() {
        maze = Maze::new(&term);

        if maze != last_maze {
            term.clear();
            Maze::new(&term).draw(&mut term);
            term.flush();

            last_maze.copy_from(&maze);
        }

        let mut timeout = Duration::from_millis(100);
        while term.poll(timeout) > 0 {
            match term.read() {
                CTRL_C | ESC | b'q' => break 'main_loop,
                _ => {}
            }

            timeout = Duration::ZERO;
        }
    }
}
