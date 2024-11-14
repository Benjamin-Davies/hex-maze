use std::{io, thread::sleep, time::Duration};

use terminal::{Terminal, CTRL_C, ESC};

mod terminal;

fn main() -> io::Result<()> {
    let mut term = Terminal::new();

    let (w, h) = term.size();
    term.clear().goto(0, 0);
    for y in 0..h {
        for x in 0..w {
            term.write(if (x + y) % 2 == 0 { "#" } else { " " });
        }
    }
    term.flush();

    while !term.should_exit() {
        match term.read() {
            CTRL_C | ESC | b'q' => break,
            _ => {}
        }
        sleep(Duration::from_millis(16));
    }

    Ok(())
}
