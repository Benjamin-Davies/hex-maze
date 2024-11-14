use std::{
    io::{self, Read, Write},
    os::fd::AsRawFd,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use termios::{cfmakeraw, tcsetattr, Termios};

pub const fn ctrl(c: u8) -> u8 {
    c & 0x1f
}

pub const CTRL_C: u8 = ctrl(b'c');
pub const ESC: u8 = ctrl(b'[');

pub struct Terminal {
    exit: Arc<AtomicBool>,
    stdin: io::StdinLock<'static>,
    stdout: io::StdoutLock<'static>,
    old_termios: Termios,
}

impl Terminal {
    pub fn new() -> Self {
        let exit = Arc::new(AtomicBool::new(false));
        let exit_clone = exit.clone();
        ctrlc::try_set_handler(move || {
            exit_clone.store(true, Ordering::SeqCst);
        })
        .unwrap();

        let stdin = io::stdin().lock();
        let stdout = io::stdout().lock();
        let fd = stdin.as_raw_fd();

        let old_termios = Termios::from_fd(fd).unwrap();
        let mut termios = old_termios;
        cfmakeraw(&mut termios);
        tcsetattr(fd, termios::TCSADRAIN, &mut termios).unwrap();

        let mut term = Self {
            exit,
            stdin,
            stdout,
            old_termios,
        };

        term.alt_screen(true).cursor_visible(false).flush();

        term
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.alt_screen(false).cursor_visible(true).flush();

        let fd = self.stdin.as_raw_fd();
        tcsetattr(fd, termios::TCSADRAIN, &self.old_termios).unwrap();
    }
}

impl Terminal {
    pub fn should_exit(&self) -> bool {
        self.exit.load(Ordering::SeqCst)
    }

    pub fn read(&mut self) -> u8 {
        let mut buf = [0];
        match self.stdin.read(&mut buf) {
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => return 0,
            res => res,
        }
        .unwrap();
        buf[0]
    }

    pub fn write(&mut self, s: impl AsRef<[u8]>) -> &mut Self {
        self.stdout.write_all(s.as_ref()).unwrap();
        self
    }

    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    fn csi(&mut self) -> &mut Self {
        self.write([ESC, b'['])
    }

    pub fn clear(&mut self) -> &mut Self {
        self.csi().write("2J")
    }

    pub fn goto(&mut self, row: u32, col: u32) -> &mut Self {
        self.csi();
        write!(self.stdout, "{row};{col}H").unwrap();
        self
    }

    fn cursor_visible(&mut self, visible: bool) -> &mut Self {
        if visible {
            self.csi().write("?25h")
        } else {
            self.csi().write("?25l")
        }
    }

    /// Alternate screen buffer
    fn alt_screen(&mut self, enable: bool) -> &mut Self {
        if enable {
            self.csi().write("?1049h")
        } else {
            self.csi().write("?1049l")
        }
    }
}
