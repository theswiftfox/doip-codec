use doip_definitions::{
    definitions::{
        DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN, DOIP_DIAG_COMMON_TARGET_OFFSET,
        DOIP_DIAG_MESSAGE_DATA_OFFSET, DOIP_HEADER_LEN,
    },
    payload::{DiagnosticMessage, DoipPayload},
};

use crate::{doip_message::header::HeaderCodec, DecodeError, Decoder, EncodeError, Encoder};

#[derive(Debug)]
pub struct DiagnosticMessageCodec {}

impl Encoder<DiagnosticMessage> for DiagnosticMessageCodec {
    type Error = EncodeError;

    fn to_bytes(&mut self, item: DiagnosticMessage, dst: &mut Vec<u8>) -> Result<(), Self::Error> {
        let DiagnosticMessage {
            source_address,
            target_address,
            message,
        } = item;

        dst.extend_from_slice(&source_address);

        dst.extend_from_slice(&target_address);

        dst.extend_from_slice(&message);

        Ok(())
    }
}

impl Decoder for DiagnosticMessageCodec {
    type Item = DoipPayload;

    type Error = DecodeError;

    fn decode_from_bytes(&mut self, src: &mut Vec<u8>) -> Result<Option<Self::Item>, Self::Error> {
        const BASE_MSG_LEN: usize = DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN;
        if src.len() < DOIP_HEADER_LEN + BASE_MSG_LEN {
            return Err(DecodeError::TooShort);
        }

        let mut h_codec = HeaderCodec {};
        let header = h_codec
            .decode_from_bytes(src)?
            .expect("Should never return Ok(None)");

        let source_address = src[DOIP_HEADER_LEN..DOIP_HEADER_LEN + DOIP_DIAG_COMMON_SOURCE_LEN]
            .try_into()
            .expect("If failed, source has been manupulated at runtime.");

        let target_address = src[DOIP_DIAG_COMMON_TARGET_OFFSET
            ..DOIP_DIAG_COMMON_TARGET_OFFSET + DOIP_DIAG_COMMON_TARGET_LEN]
            .try_into()
            .expect("If failed, source has been manupulated at runtime.");

        let data_size = header.payload_length as usize - BASE_MSG_LEN;
        let message = if src.len() >= DOIP_DIAG_MESSAGE_DATA_OFFSET + data_size {
            src[DOIP_DIAG_MESSAGE_DATA_OFFSET..DOIP_DIAG_MESSAGE_DATA_OFFSET + data_size].to_vec()
        } else {
            vec![]
        };

        let item = DiagnosticMessage {
            source_address,
            target_address,
            message,
        };

        Ok(Some(DoipPayload::DiagnosticMessage(item)))
    }
}
