pub mod action_code;
pub mod activation_code;
pub mod activation_type;
pub mod diagnostic_ack;
pub mod diagnostic_nack;
pub mod message;
pub mod nack_codes;
pub mod node_type;
pub mod power_mode;
pub mod sync_status;

#[cfg(test)]
mod size_tests {
    use std::mem;

    use crate::doip::message::{
        action_code::ActionCode, activation_code::ActivationCode, activation_type::ActivationType,
        diagnostic_ack::DiagnosticAckCode, diagnostic_nack::DiagnosticNackCode,
        message::DoipMessage, nack_codes::NackCodes, node_type::NodeType, power_mode::PowerMode,
        sync_status::SyncStatus,
    };

    #[test]
    fn test_struct_sizes() {
        dbg!(mem::size_of::<ActionCode>());
        dbg!(mem::size_of::<ActivationCode>());
        dbg!(mem::size_of::<ActivationType>());
        dbg!(mem::size_of::<DiagnosticAckCode>());
        dbg!(mem::size_of::<DiagnosticNackCode>());
        dbg!(mem::size_of::<DoipMessage>());
        dbg!(mem::size_of::<NackCodes>());
        dbg!(mem::size_of::<NodeType>());
        dbg!(mem::size_of::<PowerMode>());
        dbg!(mem::size_of::<SyncStatus>());
    }
}
