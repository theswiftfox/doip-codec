/// A wrapper to encapsulate Parser and IO errors which can occur
#[derive(thiserror::Error, Debug, Clone, PartialEq)]
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

    /// Unable to convert to target bytes
    #[error("unable to convert to target bytes")]
    TryFromBytes,

    /// failed protocol validation
    #[error("failed protocol validation")]
    FailedProtocolValidation,

    /// invalid header
    #[error("invalid header")]
    InvalidHeader,

    /// invalid payload
    #[error("invalid payload")]
    InvalidPayload,

    /// invalid nack code
    #[error("invalid nack code")]
    InvalidNackCode,

    /// invalid invalid protocol version
    #[error("invalid invalid protocol version")]
    InvalidProtocolVersion,

    /// invalid invalid payload type
    #[error("invalid invalid payload type")]
    InvalidPayloadType,

    /// invalid invalid action code
    #[error("invalid invalid action code")]
    InvalidActionCode,

    /// invalid invalid sync status
    #[error("invalid invalid sync status")]
    InvalidSyncStatus,

    /// invalid invalid activation type
    #[error("invalid invalid activation type")]
    InvalidActivationType,

    /// invalid invalid activation code
    #[error("invalid invalid activation code")]
    InvalidActivationCode,

    /// invalid invalid node type
    #[error("invalid invalid node type")]
    InvalidNodeType,

    /// invalid invalid power mode
    #[error("invalid invalid power mode")]
    InvalidPowerMode,

    /// invalid invalid diagnostic ack code
    #[error("invalid invalid diagnostic ack code")]
    InvalidDiagnosticAckCode,

    /// invalid invalid diagnostic nack code
    #[error("invalid invalid diagnostic nack code")]
    InvalidDiagnosticNackCode,

    /// message received is too large for buffer
    #[error("message received is too large for buffer")]
    MessageTooLarge,
}

/// A wrapper to encapsulate IO errors which can occur
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum EncodeError {
    /// failed protocol validation
    #[error("failed protocol validation")]
    FailedProtocolValidation,

    /// payload type mismatch
    #[error("payload type mismatch")]
    PayloadTypeValidation,

    /// payload length mismatch
    #[error("payload length mismatch")]
    PayloadLengthValidation,

    /// buffer provided too small
    #[error("buffer provided too small")]
    BufferTooSmall,
}
