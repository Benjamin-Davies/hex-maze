use std::{
    io::{self, Read, Write},
    mem,
    os::fd::{AsRawFd, RawFd},
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use crate::sys::{
    ioctl::{ioctl, IOCtl, WinSize},
    poll::{poll, PollEvents, PollFd},
    signal::{sigaction, SigAction, SigActionFlags, SigSet, Signal},
    termios::{cfmakeraw, tcgetattr, tcsetattr, SetAttrActions, Termios},
};

pub const fn ctrl(c: u8) -> u8 {
    c & 0x1f
}

pub const CTRL_C: u8 = ctrl(b'c');
pub const ESC: u8 = ctrl(b'[');

pub static EXIT: AtomicBool = AtomicBool::new(false);

pub struct Terminal {
    stdin: io::StdinLock<'static>,
    stdout: io::StdoutLock<'static>,
    old_termios: Termios,
}

impl Terminal {
    pub fn new() -> Self {
        let mut term = Self {
            stdin: io::stdin().lock(),
            stdout: io::stdout().lock(),
            old_termios: unsafe { mem::zeroed() },
        };

        extern "C" fn sigkill_handler(_: Signal) {
            EXIT.store(true, Ordering::SeqCst);
        }
        unsafe {
            sigaction(
                Signal::Term,
                Some(&SigAction {
                    handler: sigkill_handler,
                    mask: SigSet::default(),
                    flags: SigActionFlags::none(),
                }),
                None,
            );
        }

        let fd = term.fd();
        unsafe {
            tcgetattr(fd, &mut term.old_termios);
            let mut termios = term.old_termios;
            cfmakeraw(&mut termios);
            tcsetattr(fd, SetAttrActions::Drain, &mut termios);
        }

        term.alt_screen(true).cursor_visible(false).flush();

        term
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.alt_screen(false).cursor_visible(true).flush();

        unsafe {
            tcsetattr(self.fd(), SetAttrActions::Drain, &self.old_termios);
        }
    }
}

impl Terminal {
    pub fn should_exit(&self) -> bool {
        EXIT.load(Ordering::SeqCst)
    }

    pub fn poll(&mut self, timeout: Duration) -> u32 {
        let mut poll_fd = PollFd {
            fd: self.fd(),
            events: PollEvents::In,
            revents: PollEvents::none(),
        };
        let res = unsafe { poll(&mut poll_fd, 1, timeout.as_millis() as i32) };
        if res < 0 {
            panic!("poll failed");
        } else {
            res as u32
        }
    }

    pub fn read(&mut self) -> u8 {
        let mut buf = [0];
        self.stdin.read(&mut buf).unwrap();
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

    pub fn goto(&mut self, x: u16, y: u16) -> &mut Self {
        let row = y + 1;
        let col = x + 1;
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

    fn fd(&self) -> RawFd {
        self.stdin.as_raw_fd()
    }

    pub fn size(&self) -> (u16, u16) {
        let mut size = WinSize::default();
        unsafe {
            ioctl(self.fd(), IOCtl::WinSize, &mut size);
        }
        (size.col, size.row)
    }
}
