use std::{thread::sleep, time::Duration};

use self::{
    maze::Maze,
    terminal::{Terminal, CTRL_C, ESC},
};

mod maze;
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

        if term.poll(Duration::from_millis(100)) > 0 {
            loop {
                match term.read() {
                    CTRL_C | ESC | b'q' => break 'main_loop,
                    _ => {}
                }

                if term.poll(Duration::ZERO) == 0 {
                    break;
                }
            }
        }
    }
}
