use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActivationCode {
    DeniedUnknownSourceAddress = 0x00,
    DeniedTCPSocketsFull = 0x01,
    DeniedTCPSocketAlreadyConnected = 0x02,
    DeniedSourceIsAlreadyActive = 0x03,
    DeniedMissingAuthentication = 0x04,
    DeniedRejectedConfirmation = 0x05,
    DeniedUnsupportedRoutingActivationType = 0x06,
    DeniedRequestEncryptedTLSConnection = 0x07,
    ReservedByIso13400_08 = 0x08,
    ReservedByIso13400_09 = 0x09,
    ReservedByIso13400_0A = 0x0A,
    ReservedByIso13400_0B = 0x0B,
    ReservedByIso13400_0C = 0x0C,
    ReservedByIso13400_0D = 0x0D,
    ReservedByIso13400_0E = 0x0E,
    ReservedByIso13400_0F = 0x0F,
    SuccessfullyActivated = 0x10,
    ActivatedConfirmationRequired = 0x11,
}

impl fmt::Display for ActivationCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let activation_code_strings = match self {
        ActivationCode::DeniedUnknownSourceAddress => "Routing activation denied due to unknown source address.",
        ActivationCode::DeniedTCPSocketsFull => "Routing activation denied because all concurrently supported TCP_DATA sockets are registered and active.",
        ActivationCode::DeniedTCPSocketAlreadyConnected => "Routing activation denied because an SA different from the table connection entry was received on the already activated TCP_DATA socket.",
        ActivationCode::DeniedSourceIsAlreadyActive => "Routing activation denied because the SA is already registered and active on a different TCP_DATA socket.",
        ActivationCode::DeniedMissingAuthentication => "Routing activation denied due to missing authentication.",
        ActivationCode::DeniedRejectedConfirmation => "Routing activation denied due to rejected confirmation.",
        ActivationCode::DeniedUnsupportedRoutingActivationType => "Routing activation denied due to unsupported routing activation type.",
        ActivationCode::DeniedRequestEncryptedTLSConnection => "Routing activation denied due to request for encrypted connection via TLS.",
        ActivationCode::ReservedByIso13400_08 => "Reserved by ISO 13400.",
        ActivationCode::ReservedByIso13400_09 => "Reserved by ISO 13400.",
        ActivationCode::ReservedByIso13400_0A => "Reserved by ISO 13400.",
        ActivationCode::ReservedByIso13400_0B => "Reserved by ISO 13400.",
        ActivationCode::ReservedByIso13400_0C => "Reserved by ISO 13400.",
        ActivationCode::ReservedByIso13400_0D => "Reserved by ISO 13400.",
        ActivationCode::ReservedByIso13400_0E => "Reserved by ISO 13400.",
        ActivationCode::ReservedByIso13400_0F => "Reserved by ISO 13400.",
        ActivationCode::SuccessfullyActivated => "Routing successfully activated.",
        ActivationCode::ActivatedConfirmationRequired => "Routing will be activated; confirmation required.",
      };
        write!(f, "{}", activation_code_strings)
    }
}
