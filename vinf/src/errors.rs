use std::fmt;

#[derive(Debug)]
pub enum Error {
  NotImplemented,

  InvalidFormat,

  Io(std::io::Error),

  Other(String),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Error::NotImplemented => write!(f, "not implemented"),
      Error::InvalidFormat => write!(f, "invalid VINF format"),
      Error::Io(e) => write!(f, "io error: {}", e),
      Error::Other(s) => write!(f, "{}", s),
    }
  }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
  fn from(e: std::io::Error) -> Self {
    Error::Io(e)
  }
}
