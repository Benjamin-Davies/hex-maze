use std::time::Duration;

use hex_maze::{
    maze::Maze,
    terminal::{Terminal, CTRL_C, ESC},
};

fn main() {
    let mut term = Terminal::new();

    let mut maze = Maze::new(&term);
    let mut last_maze = Maze::empty();
    'main_loop: while !term.should_exit() {
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
                b'r' => maze = Maze::new(&term),
                _ => {}
            }

            timeout = Duration::ZERO;
        }
    }
}
