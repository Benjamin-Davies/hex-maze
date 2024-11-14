use std::{
    io::{self, Read, Write},
    mem,
    os::fd::{AsRawFd, RawFd},
    sync::atomic::{AtomicBool, Ordering},
};

use self::{
    ioctl::{ioctl, WinSize, TIOCGWINSZ},
    signal::{sigaction, SigAction, SigActionFlags, SigSet, Signal, SIGTERM},
    termios::{cfmakeraw, tcgetattr, tcsetattr, Termios, TCSADRAIN},
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
                SIGTERM,
                Some(&SigAction {
                    handler: sigkill_handler,
                    mask: SigSet::default(),
                    flags: SigActionFlags::default(),
                }),
                None,
            );
        }

        let fd = term.fd();
        unsafe {
            tcgetattr(fd, &mut term.old_termios);
            let mut termios = term.old_termios;
            cfmakeraw(&mut termios);
            tcsetattr(fd, TCSADRAIN, &mut termios);
        }

        term.alt_screen(true).cursor_visible(false).flush();

        term
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.alt_screen(false).cursor_visible(true).flush();

        unsafe {
            tcsetattr(self.fd(), TCSADRAIN, &self.old_termios);
        }
    }
}

impl Terminal {
    pub fn should_exit(&self) -> bool {
        EXIT.load(Ordering::SeqCst)
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
            ioctl(self.fd(), TIOCGWINSZ, &mut size);
        }
        (size.col, size.row)
    }
}

/// ioctl.h
mod ioctl {
    use std::os::fd::RawFd;

    #[derive(Clone, Copy)]
    #[repr(transparent)]
    pub struct IOCtl(u64);

    pub const TIOCGWINSZ: IOCtl = IOCtl(0x5413);

    #[derive(Default)]
    #[repr(C)]
    pub struct WinSize {
        pub row: u16,
        pub col: u16,
        xpixel: u16,
        ypixel: u16,
    }

    extern "C" {
        pub fn ioctl(fd: RawFd, request: IOCtl, ...) -> i32;
    }
}

/// signal.h
mod signal {
    use std::mem;

    #[derive(Clone, Copy)]
    #[repr(transparent)]
    pub struct Signal(i32);

    pub const SIGTERM: Signal = Signal(15);

    #[repr(C)]
    pub struct SigAction {
        pub handler: SignalHandler,
        pub mask: SigSet,
        pub flags: SigActionFlags,
    }

    pub type SignalHandler = extern "C" fn(Signal);

    #[derive(Default)]
    #[repr(C)]
    pub struct SigSet([u64; 1024 / (8 * mem::size_of::<u64>())]);

    #[derive(Default)]
    #[repr(transparent)]
    pub struct SigActionFlags(i32);

    extern "C" {
        pub fn sigaction(
            signal: Signal,
            action: Option<&SigAction>,
            old_action: Option<&mut SigAction>,
        ) -> i32;
    }
}

/// termios.h
mod termios {
    use std::os::fd::RawFd;

    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct Termios {
        iflag: u32,
        oflag: u32,
        cflag: u32,
        lflag: u32,
        line: u8,
        control_chars: [u8; 32],
        ispeed: u32,
        ospeed: u32,
    }

    #[derive(Clone, Copy)]
    #[repr(transparent)]
    pub struct TermiosSetAttrActions(i32);

    pub const TCSADRAIN: TermiosSetAttrActions = TermiosSetAttrActions(1);

    extern "C" {
        pub fn cfmakeraw(termios: &mut Termios);
        pub fn tcgetattr(fd: RawFd, termios: &mut Termios) -> i32;
        pub fn tcsetattr(
            fd: RawFd,
            optional_actions: TermiosSetAttrActions,
            termios: &Termios,
        ) -> i32;
    }
}
