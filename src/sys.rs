/// ioctl.h
pub mod ioctl {
    use std::os::fd::RawFd;

    use bitmask_enum::bitmask;

    #[bitmask(u64)]
    pub enum IOCtl {
        WinSize = 0x5413,
    }

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

/// poll.h
pub mod poll {
    use std::os::fd::RawFd;

    use bitmask_enum::bitmask;

    #[repr(C)]
    pub struct PollFd {
        pub fd: RawFd,
        pub events: PollEvents,
        pub revents: PollEvents,
    }

    #[bitmask(i32)]
    pub enum PollEvents {
        In = 0x001,
    }

    extern "C" {
        pub fn poll(fds: *mut PollFd, nfds: usize, timeout: i32) -> i32;
    }
}

/// signal.h
pub mod signal {
    use std::mem;

    use bitmask_enum::bitmask;

    #[derive(Clone, Copy)]
    #[repr(i32)]
    pub enum Signal {
        Term = 15,
    }

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

    #[bitmask(i32)]
    pub enum SigActionFlags {
        _NONE = 0,
    }

    extern "C" {
        pub fn sigaction(
            signal: Signal,
            action: Option<&SigAction>,
            old_action: Option<&mut SigAction>,
        ) -> i32;
    }
}

/// termios.h
pub mod termios {
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
    #[repr(i32)]
    pub enum SetAttrActions {
        Drain = 1,
    }

    extern "C" {
        pub fn cfmakeraw(termios: &mut Termios);
        pub fn tcgetattr(fd: RawFd, termios: &mut Termios) -> i32;
        pub fn tcsetattr(fd: RawFd, optional_actions: SetAttrActions, termios: &Termios) -> i32;
    }
}
