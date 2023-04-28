use std::io::Read;
use std::time::Instant;
use std::{fmt, io, thread};

use crossbeam_channel::{Receiver, RecvTimeoutError};

pub struct PollingStdin {
    recv: Receiver<io::Result<Bytes>>,
}

#[derive(Default, Clone)]
pub struct Bytes {
    len: usize,
    buf: [u8; 32],
}

impl Bytes {
    pub fn as_slice(&self) -> &[u8] {
        &self.buf[..self.len]
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl fmt::Debug for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = String::from_utf8_lossy(self.as_slice());
        s.fmt(f)
    }
}

impl Default for PollingStdin {
    fn default() -> Self {
        Self::new()
    }
}

impl PollingStdin {
    pub fn new() -> Self {
        let (send, recv) = crossbeam_channel::bounded(8);

        let mut stdin = io::stdin();
        thread::spawn(move || {
            loop {
                let mut bytes = Bytes::default();

                match stdin.read(&mut bytes.buf) {
                    // Some bytes were written, so send them to the main thread.
                    Ok(len) => {
                        bytes.len = len;
                        send.send(Ok(bytes)).unwrap()
                    }

                    // Interrupted - continue reading.
                    Err(err) if err.kind() == io::ErrorKind::Interrupted => {}

                    // Eww, an error.
                    Err(err) => send.send(Err(err)).unwrap(),
                }
            }
        });

        Self { recv }
    }

    pub fn read_with_deadline(&self, deadline: Instant) -> io::Result<Option<Bytes>> {
        match self.recv.recv_deadline(deadline) {
            Ok(bytes) => bytes.map(Some),
            Err(RecvTimeoutError::Timeout) => Ok(None),
            Err(RecvTimeoutError::Disconnected) => panic!("sender disconnected"),
        }
    }
}

// #[test]
// fn foo() {
//     let _term = crate::term::RawTermGuard::new(libc::STDIN_FILENO);
//     let stdin = PollingStdin::new();

//     loop {
//         let Some(bytes) = stdin.read_while_available().unwrap() else {
//             continue;
//         };
//         print!("{:?}\r\n", bytes.as_slice());
//     }
// }
