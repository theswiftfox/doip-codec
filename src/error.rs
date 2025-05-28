use derive_more::From;

/// Custom Result type which allows for easier typing of errors across the API.
pub type Result<T> = core::result::Result<T, Error>;

/// Custom Error enum for deriving error types across the application and API.
///
/// Expand to fit new dependencies using `#[from]` and implement custom error types
/// with context.
#[derive(Debug, From)]
pub enum Error {
    /// Derived implementation for standard library IO errors
    #[from]
    #[cfg(feature = "std")]
    IoError(std::io::Error),

    /// Derived implementation for Doip Definition errors
    #[from]
    DefinitionError(doip_definitions::error::Error),

    /// Derived implementation for standard library Slice errors
    #[from]
    #[allow(clippy::enum_variant_names)]
    SliceError(core::array::TryFromSliceError),

    /// Derived for heapless extend from slice errors
    #[from]
    Heapless(()),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}
