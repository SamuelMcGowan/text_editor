use std::io::{Read, Write};
use std::{io, thread};

use crossbeam_channel::Receiver;

struct PollingStdin {
    recv: Receiver<io::Result<Bytes>>,
}

#[derive(Default)]
struct Bytes {
    len: usize,
    buf: [u8; 32],
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

    pub fn read_while_available(&mut self, mut dest: impl Write) -> io::Result<usize> {
        let mut bytes_written = 0;

        for res in self.recv.try_iter() {
            let bytes = res?;

            if bytes.len == 0 {
                break;
            }

            dest.write_all(&bytes.buf[..bytes.len])?;
            bytes_written += bytes.len;
        }

        Ok(bytes_written)
    }
}

#[test]
fn foo() {
    let _term = crate::term::RawTermGuard::new(libc::STDIN_FILENO);

    let mut stdin = PollingStdin::new();

    loop {
        let mut buf = vec![];
        stdin.read_while_available(&mut buf).unwrap();

        if !buf.is_empty() {
            print!("{buf:?}\r\n");
        }
    }
}
