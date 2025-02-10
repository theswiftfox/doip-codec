/// A wrapper to encapsulate Parser and IO errors which can occur
#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    /// Exceeded length
    #[error("exceeded available length")]
    ExceededLength,
}

/// A wrapper to encapsulate IO errors which can occur
#[derive(thiserror::Error, Debug)]
pub enum EncodeError {}
