use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NackCodes {
    IncorrectPatternFormat = 0x00,
    UnknownPayloadType = 0x01,
    MessageTooLarge = 0x02,
    OutOfMemory = 0x03,
    InvalidPayloadLength = 0x04,
}

impl fmt::Display for NackCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let nack_string = match self {
            NackCodes::IncorrectPatternFormat => "Incorrect pattern format",
            NackCodes::UnknownPayloadType => "Unknown payload type",
            NackCodes::MessageTooLarge => "Message too large",
            NackCodes::OutOfMemory => "Out of memory",
            NackCodes::InvalidPayloadLength => "Invalid payload length",
        };
        write!(f, "{}", nack_string)
    }
}