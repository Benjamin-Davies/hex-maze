use std::time::Duration;

use hex_maze::{
    generator::Generator,
    terminal::{Terminal, CTRL_C, ESC},
};

fn main() {
    let mut term = Terminal::new();

    let mut generator = Generator::new(&term);
    'main_loop: while !term.should_exit() {
        generator.step();

        generator.maze.draw(&mut term);
        term.flush();

        let mut timeout = Duration::from_millis(100);
        while term.poll(timeout) > 0 {
            match term.read() {
                CTRL_C | ESC | b'q' => break 'main_loop,
                b'r' => {
                    term.clear();
                    generator = Generator::new(&term);
                }
                _ => {}
            }

            timeout = Duration::ZERO;
        }
    }
}
