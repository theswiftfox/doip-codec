/// A wrapper to encapsulate Parser and IO errors which can occur
#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    /// Exceeded length
    #[error("exceeded available length")]
    ExceededLength,

    /// Buffer too short
    #[error("buffer too short")]
    TooShort,

    /// Expected to be unreachable
    #[error("expected to be unreachable")]
    Unreachable,
}

/// A wrapper to encapsulate IO errors which can occur
#[derive(thiserror::Error, Debug)]
pub enum EncodeError {
    /// failed protocol validation
    #[error("failed protocol validation")]
    FailedProtocolValidation,
}
