/// An error in this library.
///
/// ## Examples
/// ```no_run
/// let check_size = |size: usize| -> Result<(), rapiddb::errors::Error> {
///     if size > 4000000 {
///         return Err(rapiddb::errors::Error::FileFull);
///     }
///     if size > 9999 {
///         return Err(rapiddb::errors::Error::ArrayFull);
///     }
///
///     Ok(())
/// };
///
/// check_size(10000).unwrap_or_else(|error| match error {
///     rapiddb::errors::Error::FileFull => {
///         println!("handle FileFull here");
///     }
///     rapiddb::errors::Error::ArrayFull => {
///         println!("handle ArrayFull here");
///     }
///     _ => (),
/// });
/// ```
#[derive(Debug)]
pub enum Error {
  SizeCorrupted,
  SeekCorrupted,
  FileFull,
  ArrayFull,
  ArrayEmpty,
  IndexOutOfRange,
  IndexOutOfBounds,
  StdNumParseIntError(std::num::ParseIntError),
  StdIoError(std::io::Error),
  StdArrayTryFromSliceError(std::array::TryFromSliceError),
}

impl From<std::num::ParseIntError> for Error {
  fn from(error: std::num::ParseIntError) -> Self {
    Self::StdNumParseIntError(error)
  }
}

impl From<std::io::Error> for Error {
  fn from(error: std::io::Error) -> Self {
    Self::StdIoError(error)
  }
}

impl From<std::array::TryFromSliceError> for Error {
  fn from(error: std::array::TryFromSliceError) -> Self {
    Self::StdArrayTryFromSliceError(error)
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::SizeCorrupted => write!(f, "Size corrupted"),
      Self::SeekCorrupted => write!(f, "Seek corrupted"),
      Self::FileFull => write!(f, "File is full"),
      Self::ArrayFull => write!(f, "Array is full"),
      Self::ArrayEmpty => write!(f, "Array is empty"),
      Self::IndexOutOfRange => write!(f, "Index out of range"),
      Self::IndexOutOfBounds => write!(f, "Index out of bounds"),
      Self::StdNumParseIntError(e) => std::fmt::Display::fmt(e, f),
      Self::StdIoError(e) => std::fmt::Display::fmt(e, f),
      Self::StdArrayTryFromSliceError(e) => std::fmt::Display::fmt(e, f),
    }
  }
}

impl std::error::Error for Error {}
