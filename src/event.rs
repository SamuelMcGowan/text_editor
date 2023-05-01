use std::io;
use std::time::Instant;

use bitflags::bitflags;

use crate::input::{Bytes, PollingStdin};

#[derive(Debug, Clone)]
pub struct Event {
    pub bytes: Bytes,
    pub kind: EventKind,
}

#[derive(Debug, Clone)]
pub enum EventKind {
    Key(KeyEvent),
    String(String),
    Unknown,
}

impl EventKind {
    fn just_key(key_code: KeyCode) -> Self {
        Self::Key(KeyEvent {
            key_code,
            modifiers: Modifiers::empty(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyEvent {
    pub key_code: KeyCode,
    pub modifiers: Modifiers,
}

impl KeyEvent {
    pub fn key(key_code: KeyCode) -> Self {
        Self {
            key_code,
            modifiers: Modifiers::empty(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    Backspace,

    PageUp,
    PageDown,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    pub fn read_with_deadline(&self, deadline: Instant) -> io::Result<Option<Event>> {
        let Some(bytes) = self.stdin.read_with_deadline(deadline)? else {
            return Ok(None);
        };
        let kind = parse_event(bytes.as_slice()).unwrap_or(EventKind::Unknown);
        Ok(Some(Event { kind, bytes }))
    }
}

fn parse_event(bytes: &[u8]) -> Option<EventKind> {
    // print!("bytes: {bytes:?}\r\n");

    let (&first, rest) = bytes.split_first()?;

    let event = match first {
        b'\x1b' => {
            match rest {
                b"" | b"\x1b" => EventKind::just_key(KeyCode::Escape),

                b"[" => EventKind::Key(KeyEvent {
                    key_code: KeyCode::Char('['),
                    modifiers: Modifiers::ALT,
                }),

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

                    EventKind::Key(KeyEvent {
                        key_code,
                        modifiers,
                    })
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

                    EventKind::Key(KeyEvent {
                        key_code,
                        modifiers,
                    })
                }

                [c] => {
                    let mut key_event = decode_byte(*c);
                    key_event.modifiers |= Modifiers::ALT;
                    EventKind::Key(key_event)
                }

                _ => return None,
            }
        }

        _ => decode_bytes(bytes)?,
    };

    Some(event)
}

fn parse_modifiers(bytes: &[u8]) -> Option<Modifiers> {
    std::str::from_utf8(bytes)
        .ok()
        .and_then(|s| s.parse::<u8>().ok())
        .map(|byte| Modifiers::from_bits_truncate(byte.saturating_sub(1)))
}

/// Decode a character, handling the control keys,
/// control characters* and utf-8.
///
/// Panics if the first byte isn't present.
///
/// *Not to be confused with one another.
fn decode_bytes(bytes: &[u8]) -> Option<EventKind> {
    match bytes {
        [] => None,
        [b] => Some(EventKind::Key(decode_byte(*b))),
        _ => {
            let s = std::str::from_utf8(bytes).ok()?;
            let kind = if s.chars().nth(1).is_some() {
                EventKind::String(s.to_owned())
            } else {
                EventKind::Key(KeyEvent::key(KeyCode::Char(s.chars().next()?)))
            };
            Some(kind)
        }
    }
}

fn decode_byte(byte: u8) -> KeyEvent {
    match byte {
        b'\t' => KeyEvent::key(KeyCode::Tab),
        b'\n' => KeyEvent::key(KeyCode::Newline),
        b'\r' => KeyEvent::key(KeyCode::Return),

        b'\x7f' => KeyEvent::key(KeyCode::Backspace),
        b'\x17' => KeyEvent {
            key_code: KeyCode::Backspace,
            modifiers: Modifiers::CTRL,
        },

        b if b < 27 => KeyEvent {
            key_code: KeyCode::Char((b'A' + b - 1) as char),
            modifiers: Modifiers::CTRL,
        },

        _ => KeyEvent::key(KeyCode::Char(byte as char)),
    }
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
