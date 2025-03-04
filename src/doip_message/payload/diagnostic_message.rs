use doip_definitions::{
    definitions::{
        DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN, DOIP_DIAG_COMMON_TARGET_OFFSET,
        DOIP_DIAG_MESSAGE_DATA_OFFSET, DOIP_HEADER_LEN,
    },
    payload::{DiagnosticMessage, DoipPayload},
};
use heapless::Vec;

use crate::{doip_message::header::HeaderCodec, DecodeError, Decoder, EncodeError, Encoder};

#[derive(Debug)]
pub struct DiagnosticMessageCodec {}

impl<const N: usize> Encoder<DiagnosticMessage<N>, N> for DiagnosticMessageCodec {
    type Error = EncodeError;

    fn encode(
        &mut self,
        item: DiagnosticMessage<N>,
        dst: &mut Vec<u8, N>,
    ) -> Result<(), Self::Error> {
        let DiagnosticMessage {
            source_address,
            target_address,
            message,
        } = item;

        dst.extend_from_slice(&source_address).map_err(|()| EncodeError::BufferTooSmall)?;

        dst.extend_from_slice(&target_address).map_err(|()| EncodeError::BufferTooSmall)?;

        dst.extend_from_slice(&message).map_err(|()| EncodeError::BufferTooSmall)?;

        Ok(())
    }
}

impl<const N: usize> Decoder<N> for DiagnosticMessageCodec {
    type Item = DoipPayload<N>;

    type Error = DecodeError;

    fn decode(&mut self, src: &mut Vec<u8, N>) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < DOIP_HEADER_LEN + DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN {
            return Err(DecodeError::TooShort);
        };

        let mut h_codec = HeaderCodec {};
        let header = h_codec.decode(src)?.expect("Should never return Ok(None)");

        let source_address = src[DOIP_HEADER_LEN..DOIP_HEADER_LEN + DOIP_DIAG_COMMON_SOURCE_LEN]
            .try_into()
            .expect("If failed, source has been manupulated at runtime.");

        let target_address = src[DOIP_DIAG_COMMON_TARGET_OFFSET
            ..DOIP_DIAG_COMMON_TARGET_OFFSET + DOIP_DIAG_COMMON_TARGET_LEN]
            .try_into()
            .expect("If failed, source has been manupulated at runtime.");

        let message = src[DOIP_DIAG_MESSAGE_DATA_OFFSET
            ..(header.payload_length as usize) - DOIP_DIAG_COMMON_TARGET_LEN
                + DOIP_DIAG_COMMON_SOURCE_LEN]
            .try_into()
            .expect("If failed, source has been manupulated at runtime.");

        let item = DiagnosticMessage {
            source_address,
            target_address,
            message,
        };

        Ok(Some(DoipPayload::DiagnosticMessage(item)))
    }
}
