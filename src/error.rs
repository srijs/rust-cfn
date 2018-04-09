use std::fmt;

#[derive(Clone, Copy, Debug)]
/// A list specifying general categories of CloudFormation errors.
///
/// This list is intended to grow over time and it is not recommended to
/// exhaustively match against it.
///
/// It is used with the [`Error`] type.
///
/// [`Error`]: struct.Error.html
pub enum ErrorKind {
    /// Serialization or deserialization failed.
    Serialization,
    /// An item was not found, often a resource or output.
    NotFound,
    /// Any error not part of this list.
    Other,
    #[doc(hidden)]
    __Nonexhaustive
}

#[derive(Debug)]
/// The error type for CloudFormation operations.
pub struct Error {
    kind: ErrorKind,
    message: String
}

impl Error {
    pub(crate) fn new<E: fmt::Display>(kind: ErrorKind, err: E) -> Error {
        Error { kind, message: err.to_string() }
    }

    /// Returns the corresponding `ErrorKind` for this error.
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::Serialization =>
                write!(f, "An error occurred during serialization or deserialization: {}", self.message),
            ErrorKind::NotFound =>
                write!(f, "An item was not found: {}", self.message),
            ErrorKind::Other =>
                write!(f, "An unknown error occurred: {}", self.message),
            ErrorKind::__Nonexhaustive => unreachable!()
        }
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}
