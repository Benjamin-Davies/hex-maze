use std::{
    io::{self, Read, Write},
    mem,
    os::fd::AsRawFd,
    sync::atomic::{AtomicBool, Ordering},
};

use self::{
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

        let stdin = io::stdin().lock();
        let stdout = io::stdout().lock();
        let fd = stdin.as_raw_fd();

        let mut old_termios;
        unsafe {
            old_termios = mem::zeroed();
            tcgetattr(fd, &mut old_termios);
            let mut termios = old_termios;
            cfmakeraw(&mut termios);
            tcsetattr(fd, TCSADRAIN, &mut termios);
        }

        let mut term = Self {
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
        unsafe {
            tcsetattr(fd, TCSADRAIN, &self.old_termios);
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

    pub fn goto(&mut self, row: u16, col: u16) -> &mut Self {
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

/// A subset of signal.h
mod signal {
    use std::{ffi::c_int, mem};

    #[derive(Clone, Copy)]
    #[repr(transparent)]
    pub struct Signal(c_int);

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
    pub struct SigActionFlags(c_int);

    extern "C" {
        pub fn sigaction(
            signal: Signal,
            action: Option<&SigAction>,
            old_action: Option<&mut SigAction>,
        ) -> c_int;
    }
}

/// A subset of termios.h
mod termios {
    use std::{
        ffi::{c_int, c_uint},
        os::fd::RawFd,
    };

    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct Termios {
        iflag: c_uint,
        oflag: c_uint,
        cflag: c_uint,
        lflag: c_uint,
        line: u8,
        control_chars: [u8; 32],
        ispeed: c_uint,
        ospeed: c_uint,
    }

    #[derive(Clone, Copy)]
    #[repr(transparent)]
    pub struct TermiosSetAttrActions(c_int);

    pub const TCSADRAIN: TermiosSetAttrActions = TermiosSetAttrActions(1);

    extern "C" {
        pub fn tcgetattr(fd: RawFd, termios: &mut Termios) -> c_int;
        pub fn tcsetattr(
            fd: RawFd,
            optional_actions: TermiosSetAttrActions,
            termios: &Termios,
        ) -> c_int;
        pub fn cfmakeraw(termios: &mut Termios);
    }
}
