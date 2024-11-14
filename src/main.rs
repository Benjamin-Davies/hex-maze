use std::{io, thread::sleep, time::Duration};

use terminal::{Terminal, CTRL_C, ESC};

mod terminal;

fn main() -> io::Result<()> {
    let mut term = Terminal::new();

    term.clear().goto(1, 1).write("Hello").flush();

    while !term.should_exit() {
        match term.read() {
            CTRL_C | ESC | b'q' => break,
            _ => {}
        }
        sleep(Duration::from_millis(16));
    }

    Ok(())
}
