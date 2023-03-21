use bitflags::bitflags;

use std::io;

use crate::input::PollingStdin;

#[derive(Debug)]
pub enum Event {
    Key {
        key_code: KeyCode,
        modifiers: Modifiers,
    },
}

impl Event {
    fn just_key(key_code: KeyCode) -> Self {
        Self::Key {
            key_code,
            modifiers: Modifiers::empty(),
        }
    }
}

#[derive(Debug)]
pub enum KeyCode {
    Char(char),
    Fn(u8),

    Tab,
    Newline,
    Return,

    Escape,

    Up,
    Down,
    Right,
    Left,

    End,
    Home,

    Insert,
    Delete,

    PageUp,
    PageDown,
}

bitflags! {
    #[derive(Debug)]
    pub struct Modifiers: u8 {
        const SHIFT = 0b0001;
        const ALT   = 0b0010;
        const CTRL  = 0b0100;
        const META  = 0b1000;
    }
}

#[derive(Default)]
pub struct EventReader {
    stdin: PollingStdin,
}

impl EventReader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read_event(&self) -> io::Result<Option<Event>> {
        let Some(bytes) = self.stdin.read_while_available()? else {
            return Ok(None);
        };
        Ok(parse_event(bytes.as_slice()))
    }
}

fn parse_event(bytes: &[u8]) -> Option<Event> {
    // print!("bytes: {bytes:?}\r\n");

    let (&first, bytes) = bytes.split_first()?;

    let event = match first {
        b'\t' => Event::just_key(KeyCode::Tab),
        b'\n' => Event::just_key(KeyCode::Newline),
        b'\r' => Event::just_key(KeyCode::Return),

        byte if byte < 27 => Event::Key {
            key_code: KeyCode::Char((64 + byte) as char),
            modifiers: Modifiers::CTRL,
        },

        b'\x1b' => {
            match bytes {
                b"" | b"\x1b" => Event::just_key(KeyCode::Escape),

                b"[" => Event::Key {
                    key_code: KeyCode::Char('['),
                    modifiers: Modifiers::ALT,
                },

                // vt sequence
                [b'[', rest @ .., b'~'] => {
                    let (key_code, modifiers) =
                        if let Some(idx) = rest.iter().position(|&byte| byte == b';') {
                            let (key_code, modifiers) = rest.split_at(idx);
                            Some((key_code, parse_modifiers(modifiers)?))
                        } else {
                            Some((rest, Modifiers::empty()))
                        }?;

                    let key_code = match key_code {
                        b"1" => KeyCode::Home,
                        b"2" => KeyCode::Insert,
                        b"3" => KeyCode::Delete,
                        b"4" => KeyCode::End,
                        b"5" => KeyCode::PageUp,
                        b"6" => KeyCode::PageDown,
                        b"7" => KeyCode::Home,
                        b"8" => KeyCode::End,

                        b"11" => KeyCode::Fn(1),
                        b"12" => KeyCode::Fn(2),
                        b"13" => KeyCode::Fn(3),
                        b"14" => KeyCode::Fn(4),
                        b"15" => KeyCode::Fn(5),

                        // no this isn't a typo, `16` is skipped
                        b"17" => KeyCode::Fn(6),
                        b"18" => KeyCode::Fn(7),
                        b"19" => KeyCode::Fn(8),
                        b"20" => KeyCode::Fn(9),
                        b"21" => KeyCode::Fn(10),

                        // who needs more than 10 function keys?
                        // let's leave it at that.
                        _ => return None,
                    };

                    Event::Key {
                        key_code,
                        modifiers,
                    }
                }

                // xterm sequence
                [b'[', modifiers @ .., key_code] => {
                    let key_code = match key_code {
                        b'A' => KeyCode::Up,
                        b'B' => KeyCode::Down,
                        b'C' => KeyCode::Right,
                        b'D' => KeyCode::Left,

                        b'F' => KeyCode::End,
                        b'H' => KeyCode::Home,

                        b'P' => KeyCode::Fn(1),
                        b'Q' => KeyCode::Fn(2),
                        b'R' => KeyCode::Fn(3),
                        b'S' => KeyCode::Fn(4),

                        _ => return None,
                    };

                    let modifiers =
                        if let Some(index) = modifiers.iter().position(|&byte| byte == b';') {
                            modifiers.split_at(index.saturating_add(1)).1
                        } else {
                            modifiers
                        };

                    let modifiers = if modifiers.is_empty() {
                        Modifiers::empty()
                    } else {
                        parse_modifiers(modifiers)?
                    };

                    Event::Key {
                        key_code,
                        modifiers,
                    }
                }

                [c] => Event::Key {
                    key_code: KeyCode::Char(*c as char),
                    modifiers: Modifiers::ALT,
                },

                _ => return None,
            }
        }

        byte if byte >= 32 => Event::Key {
            key_code: KeyCode::Char(byte as char),
            modifiers: Modifiers::empty(),
        },

        _ => return None,
    };

    Some(event)
}

fn parse_modifiers(bytes: &[u8]) -> Option<Modifiers> {
    std::str::from_utf8(bytes)
        .ok()
        .and_then(|s| s.parse::<u8>().ok())
        .map(|byte| Modifiers::from_bits_truncate(byte.saturating_sub(1)))
}

// #[test]
// fn foo() {
//     let _term = crate::term::RawTermGuard::new(libc::STDIN_FILENO);
//     let events = EventReader::new();

//     loop {
//         if let Some(event) = events.read_event().unwrap() {
//             print!("{event:?}\r\n");
//         }
//     }
// }
