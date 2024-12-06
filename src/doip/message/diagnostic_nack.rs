use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DiagnosticNackNode {
    ReservedByIso13400_00 = 0x00,
    ReservedByIso13400_01 = 0x01,
    InvalidSourceAddress = 0x02,
    UnknownTargetAddress = 0x03,
    DiagnosticMessageTooLarge = 0x04,
    OutOfMemory = 0x05,
    TargetUnreachable = 0x06,
    UnknownNetwork = 0x07,
    TransportProtocolError = 0x08,
}

impl fmt::Display for DiagnosticNackNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let diag_strings = match self {
            DiagnosticNackNode::ReservedByIso13400_00 => "Reserved by ISO 13400",
            DiagnosticNackNode::ReservedByIso13400_01 => "Reserved by ISO 13400",
            DiagnosticNackNode::InvalidSourceAddress => "Invalid source address",
            DiagnosticNackNode::UnknownTargetAddress => "Unknown target address",
            DiagnosticNackNode::DiagnosticMessageTooLarge => "Diagnostic message too large",
            DiagnosticNackNode::OutOfMemory => "Out of memory",
            DiagnosticNackNode::TargetUnreachable => "Target unreachable",
            DiagnosticNackNode::UnknownNetwork => "Unknown network",
            DiagnosticNackNode::TransportProtocolError => "Transport protocol error",
        };
        write!(f, "{}", diag_strings)
    }
}
