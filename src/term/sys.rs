use std::os::fd::{AsRawFd, RawFd};
use std::{io, mem};

use libc::termios as Termios;

macro_rules! cvt {
    ($res:expr) => {{
        match $res {
            -1 => Err(io::Error::last_os_error()),
            res => Ok(res),
        }
    }};
}

fn get_termios(fd: RawFd) -> io::Result<Termios> {
    unsafe {
        let mut termios: Termios = mem::zeroed();
        cvt!(libc::tcgetattr(fd, &mut termios))?;
        Ok(termios)
    }
}

fn set_termios(fd: RawFd, termios: &Termios) -> io::Result<()> {
    cvt!(unsafe { libc::tcsetattr(fd, libc::TCSANOW, termios) })?;
    Ok(())
}

pub(super) struct RawTermGuard {
    fd: RawFd,
    termios_prev: Termios,
}

impl RawTermGuard {
    pub fn new(fd: impl AsRawFd) -> io::Result<Self> {
        let fd = fd.as_raw_fd();

        let mut termios = get_termios(fd)?;
        let termios_prev = termios;

        unsafe { libc::cfmakeraw(&mut termios as *mut Termios) };
        set_termios(fd, &termios)?;

        Ok(Self { fd, termios_prev })
    }
}

impl Drop for RawTermGuard {
    fn drop(&mut self) {
        let _ = set_termios(self.fd, &self.termios_prev);
    }
}
