//! Error conditions.

use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
};

use url::ParseError;

/// An error encountered when trying to parse an invalid ID string.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
    /// The domain part of the the ID string is not a valid IP address or DNS name.
    InvalidHost,
    /// The ID exceeds 255 bytes (or 32 codepoints for a room version ID.)
    MaximumLengthExceeded,
    /// The ID is less than 4 characters (or is an empty room version ID.)
    MinimumLengthNotSatisfied,
    /// The ID is missing the colon delimiter between localpart and server name.
    MissingDelimiter,
    /// The ID is missing the leading sigil.
    MissingSigil,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let message = match *self {
            Error::InvalidHost => "server name is not a valid IP address or domain name",
            Error::MaximumLengthExceeded => "ID exceeds 255 bytes",
            Error::MinimumLengthNotSatisfied => "ID must be at least 4 characters",
            Error::MissingDelimiter => "colon is required between localpart and server name",
            Error::MissingSigil => "leading sigil is missing",
        };

        write!(f, "{}", message)
    }
}

impl StdError for Error {}

impl From<ParseError> for Error {
    fn from(_: ParseError) -> Self {
        Error::InvalidHost
    }
}
