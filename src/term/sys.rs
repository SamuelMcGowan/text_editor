use std::fs::File;
use std::io::Write;
use std::mem::ManuallyDrop;
use std::os::fd::{FromRawFd, RawFd};
use std::{io, mem};

use libc::{termios as Termios, winsize as Winsize, STDIN_FILENO, STDOUT_FILENO};

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

fn get_size(fd: RawFd) -> io::Result<(usize, usize)> {
    let mut size: Winsize = unsafe { mem::zeroed() };
    cvt!(unsafe { libc::ioctl(fd, libc::TIOCGWINSZ, &mut size) })?;
    Ok((size.ws_col as usize, size.ws_row as usize))
}

pub(super) struct RawTerm {
    termios_prev: Termios,
}

impl RawTerm {
    pub fn new() -> io::Result<Self> {
        let mut termios = get_termios(STDIN_FILENO)?;
        let termios_prev = termios;

        unsafe { libc::cfmakeraw(&mut termios as *mut Termios) };
        set_termios(STDIN_FILENO, &termios)?;

        Ok(Self { termios_prev })
    }

    pub fn get_size(&self) -> io::Result<(usize, usize)> {
        get_size(STDIN_FILENO)
    }
}

impl Drop for RawTerm {
    fn drop(&mut self) {
        let _ = set_termios(STDIN_FILENO, &self.termios_prev);
    }
}

#[derive(Default)]
pub struct RawStdout;

impl Write for RawStdout {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unsafe { use_stdout(|stdout| stdout.write(buf)) }
    }

    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        unsafe { use_stdout(|stdout| stdout.write_vectored(bufs)) }
    }

    fn flush(&mut self) -> io::Result<()> {
        unsafe { use_stdout(|stdout| stdout.flush()) }
    }
}

/// Safety: don't close stdout by dropping the file.
unsafe fn use_stdout<T>(f: impl Fn(&mut ManuallyDrop<File>) -> T) -> T {
    let mut stdout = ManuallyDrop::new(File::from_raw_fd(STDOUT_FILENO));
    f(&mut stdout)
}
