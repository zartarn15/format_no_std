//! Implements `write_str` to get `write_fmt`, which is used in the `format!()` and
//! `format_args!()` macros. For `no_std` formatting in a bare metal environment.
//!
//! This code is based on
//! https://stackoverflow.com/questions/50200268/how-can-i-use-the-format-macro-in-a-no-std-environment
//!
//! ``` rust
//! let mut buf = [0u8; 64];
//! let s = format_no_std::show(
//!     &mut buf,
//!     format_args!("Test String {}: {}", "foo", 42),
//! ).unwrap();
//!
//! assert_eq!("Test String foo: 42", s);
//! ```

#![no_std]

use core::cmp::min;
use core::fmt;
use core::str::from_utf8;

/// A struct representing a writer that appends formatted data to a byte buffer.
pub struct WriteTo<'a> {
    buf: &'a mut [u8],
    len: usize,
}

impl<'a> WriteTo<'a> {
    /// Constructs a new `WriteTo` instance wrapping the provided byte buffer.
    pub fn new(buf: &'a mut [u8]) -> Self {
        WriteTo { buf, len: 0 }
    }

    /// Converts the written portion of the buffer into a string slice, if possible.
    pub fn as_str(self) -> Option<&'a str> {
        if self.len <= self.buf.len() {
            from_utf8(&self.buf[..self.len]).ok()
        } else {
            None
        }
    }

    /// Get the number of bytes written to buffer, unless there where errors.
    pub fn len(&self) -> Option<usize> {
        if self.len <= self.buf.len() {
            Some(self.len)
        } else {
            None
        }
    }

    /// Returns true if self has a length of zero bytes, unless there where errors.
    pub fn is_empty(&self) -> Option<bool> {
        if self.len <= self.buf.len() {
            Some(self.len == 0)
        } else {
            None
        }
    }
}

impl<'a> fmt::Write for WriteTo<'a> {
    /// Writes a string slice into the buffer, updating the length accordingly.
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.len > self.buf.len() {
            return Err(fmt::Error);
        }

        let rem = &mut self.buf[self.len..];
        let raw_s = s.as_bytes();
        let num = min(raw_s.len(), rem.len());

        rem[..num].copy_from_slice(&raw_s[..num]);
        self.len += raw_s.len();

        if num < raw_s.len() {
            Err(fmt::Error)
        } else {
            Ok(())
        }
    }
}

/// Formats data using `format_args!` (`arg` argument) and writes it to a byte buffer `buf`.
pub fn show<'a>(buf: &'a mut [u8], arg: fmt::Arguments) -> Result<&'a str, fmt::Error> {
    let mut w = WriteTo::new(buf);
    fmt::write(&mut w, arg)?;
    w.as_str().ok_or(fmt::Error)
}

#[test]
fn test() {
    let mut buf = [0u8; 64];
    let s = show(&mut buf, format_args!("Test String {}: {}", "foo", 42)).unwrap();

    assert_eq!("Test String foo: 42", s);
}

#[test]
fn test_to_long() {
    let mut buf = [0u8; 8];
    let ret = show(&mut buf, format_args!("Too long string"));

    assert_eq!(Err(fmt::Error), ret);
}

#[test]
fn test_len() {
    use fmt::Write;
    let mut buf = [0u8; 64];
    let mut w = WriteTo::new(&mut buf);
    write!(&mut w, "Test String {}: {}", "foo", 42).unwrap();

    assert_eq!(w.len(), Some(19));
    assert_eq!(w.is_empty(), Some(false));
}

#[test]
fn test_len_empty() {
    use fmt::Write;
    let mut buf = [0u8; 64];
    let mut w = WriteTo::new(&mut buf);
    write!(&mut w, "").unwrap();

    assert_eq!(w.len(), Some(0));
    assert_eq!(w.is_empty(), Some(true));
}

#[test]
fn test_len_to_long() {
    use fmt::Write;
    let mut buf = [0u8; 8];
    let mut w = WriteTo::new(&mut buf);
    let res = write!(&mut w, "Tooo long string");

    assert_eq!(res, Err(core::fmt::Error));
    assert_eq!(w.len(), None);
    assert_eq!(w.is_empty(), None);
}
