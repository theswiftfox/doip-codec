mod decoder;
mod encoder;

// region:      --- no_std

/// A simple Decoder and Encoder implementation for Diagnostics over Internet
/// Protocol.
///
/// Can be used independently via `encode` and `decode` methods, however is best
/// utilised during.
#[cfg(not(feature = "std"))]
#[derive(Debug)]
pub struct DoipCodec<const N: usize> {}

// endregion:   --- no_std

// region:      --- std

/// A simple Decoder and Encoder implementation for Diagnostics over Internet
/// Protocol.
///
/// Can be used independently via `encode` and `decode` methods, however is best
/// utilised during.
#[cfg(feature = "std")]
#[derive(Debug)]
#[pyo3::pyclass]
pub struct DoipCodec {}

// endregion:   --- std
