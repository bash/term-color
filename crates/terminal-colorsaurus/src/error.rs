use crate::fmt::CaretNotation;
use std::time::Duration;
use std::{error, fmt, io};

/// An error returned by this library.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// I/O error
    Io(io::Error),
    /// The terminal responded using an unsupported response format.
    Parse(Vec<u8>),
    /// The query timed out. This can happen because \
    /// either the terminal does not support querying for colors \
    /// or the terminal has a lot of latency (e.g. when connected via SSH).
    Timeout(Duration),
    /// Stdout is not connected to a terminal, but [`QueryOptions::require_terminal_on_stdout`] was set.
    ///
    /// [`QueryOptions::require_terminal_on_stdout`]: `crate::QueryOptions::require_terminal_on_stdout`
    NotATerminal(NotATerminalError),
    /// The terminal does not support querying for the foreground or background color.
    UnsupportedTerminal(UnsupportedTerminalError),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Io(source) => Some(source),
            Error::NotATerminal(source) => Some(source),
            Error::UnsupportedTerminal(source) => Some(source),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {e}"),
            Error::Parse(data) => write!(
                f,
                "failed to parse response: {0}",
                // FIXME(msrv): Use `.utf8_chunks()` to avoid allocating.
                CaretNotation(String::from_utf8_lossy(data).as_ref()),
            ),
            #[allow(clippy::use_debug)]
            Error::Timeout(timeout) => {
                write!(f, "operation did not complete within {timeout:?}")
            }
            Error::NotATerminal(e) => fmt::Display::fmt(e, f),
            Error::UnsupportedTerminal(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl From<io::Error> for Error {
    fn from(source: io::Error) -> Self {
        Error::Io(source)
    }
}

impl Error {
    pub(crate) fn unsupported() -> Self {
        Error::UnsupportedTerminal(UnsupportedTerminalError)
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct NotATerminalError;

impl error::Error for NotATerminalError {}

impl fmt::Display for NotATerminalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("stdout is not connected to a terminal")
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct UnsupportedTerminalError;

impl error::Error for UnsupportedTerminalError {}

impl fmt::Display for UnsupportedTerminalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("the terminal does not support querying for its colors")
    }
}
