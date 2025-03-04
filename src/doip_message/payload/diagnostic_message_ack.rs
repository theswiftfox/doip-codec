use doip_definitions::{
    definitions::{
        DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN, DOIP_DIAG_COMMON_TARGET_OFFSET,
        DOIP_DIAG_MESSAGE_ACK_CODE_LEN, DOIP_DIAG_MESSAGE_ACK_CODE_OFFSET, DOIP_HEADER_LEN,
    },
    payload::{DiagnosticAckCode, DiagnosticMessageAck, DoipPayload},
};
use heapless::Vec;

use crate::{DecodeError, Decoder, EncodeError, Encoder, FromBytes, ToBytes};

#[derive(Debug)]
pub struct DiagnosticMessageAckCodec;

impl<const N: usize> Encoder<DiagnosticMessageAck, N> for DiagnosticMessageAckCodec {
    type Error = EncodeError;

    fn encode(
        &mut self,
        item: DiagnosticMessageAck,
        dst: &mut Vec<u8, N>,
    ) -> Result<(), Self::Error> {
        let DiagnosticMessageAck {
            source_address,
            target_address,
            ack_code,
        } = item;

        dst.extend_from_slice(&source_address).map_err(|_| EncodeError::BufferTooSmall)?;

        dst.extend_from_slice(&target_address).map_err(|_| EncodeError::BufferTooSmall)?;

        let ack_code_bytes = ack_code.to_bytes();
        dst.extend_from_slice(ack_code_bytes).map_err(|_| EncodeError::BufferTooSmall)?;

        Ok(())
    }
}

impl ToBytes for DiagnosticAckCode {
    fn to_bytes(self) -> &'static [u8] {
        match self {
            DiagnosticAckCode::Acknowledged => &[DiagnosticAckCode::Acknowledged as u8],
        }
    }
}

impl<const N: usize> Decoder<N> for DiagnosticMessageAckCodec {
    type Item = DoipPayload<N>;

    type Error = DecodeError;

    fn decode(&mut self, src: &mut Vec<u8, N>) -> Result<Option<Self::Item>, Self::Error> {
        if src.len()
            < DOIP_HEADER_LEN
                + DOIP_DIAG_COMMON_SOURCE_LEN
                + DOIP_DIAG_COMMON_TARGET_LEN
                + DOIP_DIAG_MESSAGE_ACK_CODE_LEN
        {
            return Err(DecodeError::TooShort);
        }

        let source_address = src[DOIP_HEADER_LEN..DOIP_HEADER_LEN + DOIP_DIAG_COMMON_SOURCE_LEN]
            .try_into()
            .expect("If failed, source has been manupulated at runtime.");

        let target_address = src[DOIP_DIAG_COMMON_TARGET_OFFSET
            ..DOIP_DIAG_COMMON_TARGET_OFFSET + DOIP_DIAG_COMMON_TARGET_LEN]
            .try_into()
            .expect("If failed, source has been manupulated at runtime.");

        let ack_code_bytes = &src[DOIP_DIAG_MESSAGE_ACK_CODE_OFFSET..=DOIP_DIAG_MESSAGE_ACK_CODE_OFFSET];
        let ack_code = DiagnosticAckCode::from_bytes(ack_code_bytes)
            .ok_or(DecodeError::InvalidDiagnosticAckCode)?;

        let item = DiagnosticMessageAck {
            source_address,
            target_address,
            ack_code,
        };

        Ok(Some(DoipPayload::DiagnosticMessageAck(item)))
    }
}

impl FromBytes for DiagnosticAckCode {
    fn from_bytes(bytes: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        let val = *bytes.first()?;

        if val == DiagnosticAckCode::Acknowledged as u8 {
            Some(DiagnosticAckCode::Acknowledged)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use doip_definitions::{
        header::{DoipHeader, PayloadType, ProtocolVersion},
        payload::{DiagnosticAckCode, DiagnosticMessageAck, DoipPayload},
        DoipMessage,
    };
    use heapless::Vec;

    use crate::{DecodeError, Decoder, DoipCodec, Encoder, FromBytes, ToBytes};

    const BUFFER: usize = 4095;

    static SUCCESS_ROOT: DoipMessage<BUFFER> = DoipMessage {
        header: DoipHeader {
            protocol_version: ProtocolVersion::Iso13400_2012,
            inverse_protocol_version: 0xfd,
            payload_type: PayloadType::DiagnosticMessageAck,
            payload_length: 5u32,
        },
        payload: DoipPayload::DiagnosticMessageAck(DiagnosticMessageAck {
            source_address: [0x00, 0x00],
            target_address: [0x00, 0x00],
            ack_code: DiagnosticAckCode::Acknowledged,
        }),
    };

    #[test]
    fn test_ack_code_to_bytes() {
        let bytes = DiagnosticAckCode::Acknowledged.to_bytes();
        assert_eq!(bytes, [0x00]);
    }

    #[test]
    fn test_node_type_from_bytes() {
        for a in u8::MIN..=u8::MAX {
            let bytes = &[a];
            let proto = DiagnosticAckCode::from_bytes(bytes);

            match a {
                0x00 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), DiagnosticAckCode::Acknowledged)
                }
                _ => assert!(proto.is_none()),
            }
        }
    }

    #[test]
    fn test_encode_diagnostic_message_ack_success() {
        let mut encoder = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = encoder.encode(SUCCESS_ROOT.clone(), &mut dst);

        assert!(bytes.is_ok(), "Expected bytes to be ok.");
        assert_eq!(
            *dst,
            [0x02, 0xfd, 0x80, 0x02, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00]
        );
    }

    #[test]
    fn test_decode_diagnostic_message_ack_success() {
        let mut codec = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let _ = codec.encode(SUCCESS_ROOT.clone(), &mut dst);
        let msg = codec.decode(&mut dst);

        assert!(msg.is_ok());
        let opt = msg.unwrap();

        assert!(opt.is_some());
        let res = opt.unwrap();

        assert_eq!(res, SUCCESS_ROOT)
    }

    #[test]
    fn test_decode_diagnostic_message_ack_invalid_diagnostic_ack_code() {
        let mut codec = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = &[
            0x02, 0xfd, 0x80, 0x02, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x42,
        ];
        dst.extend_from_slice(bytes).unwrap();
        let msg = codec.decode(&mut dst);

        assert_eq!(msg.unwrap_err(), DecodeError::InvalidDiagnosticAckCode);
    }

    #[test]
    fn test_decode_diagnostic_message_ack_too_short() {
        let mut codec = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = &[
            0x02, 0xfd, 0x80, 0x02, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00,
        ];
        dst.extend_from_slice(bytes).unwrap();
        let msg = codec.decode(&mut dst);

        assert_eq!(msg.unwrap_err(), DecodeError::TooShort);
    }
}
