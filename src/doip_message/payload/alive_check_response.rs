use doip_definitions::{
    definitions::{DOIP_ALIVE_CHECK_RESPONSE_SOURCE_LEN, DOIP_HEADER_LEN},
    payload::{AliveCheckResponse, DoipPayload},
};
use heapless::Vec;

use crate::{DecodeError, Decoder, EncodeError, Encoder};

#[derive(Debug)]
pub struct AliveCheckResponseCodec;

impl<const N: usize> Encoder<AliveCheckResponse, N> for AliveCheckResponseCodec {
    type Error = EncodeError;

    fn encode(
        &mut self,
        item: AliveCheckResponse,
        dst: &mut Vec<u8, N>,
    ) -> Result<(), Self::Error> {
        let AliveCheckResponse { source_address } = item;

        dst.extend_from_slice(&source_address).map_err(|()| EncodeError::BufferTooSmall)?;

        Ok(())
    }
}

impl<const N: usize> Decoder<N> for AliveCheckResponseCodec {
    type Item = DoipPayload<N>;

    type Error = DecodeError;

    fn decode(&mut self, src: &mut Vec<u8, N>) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < DOIP_HEADER_LEN + DOIP_ALIVE_CHECK_RESPONSE_SOURCE_LEN {
            return Err(DecodeError::TooShort);
        }

        let source_address = src
            [DOIP_HEADER_LEN..DOIP_HEADER_LEN + DOIP_ALIVE_CHECK_RESPONSE_SOURCE_LEN]
            .try_into()
            .expect("If failed, source has been manupulated at runtime.");

        let item = AliveCheckResponse { source_address };

        Ok(Some(DoipPayload::AliveCheckResponse(item)))
    }
}

#[cfg(test)]
mod tests {
    use doip_definitions::{
        header::{DoipHeader, PayloadType, ProtocolVersion},
        payload::{AliveCheckResponse, DoipPayload},
        DoipMessage,
    };
    use heapless::Vec;

    use crate::{DecodeError, Decoder, DoipCodec, Encoder};
    const BUFFER: usize = 4095;

    static SUCCESS_ROOT: DoipMessage<BUFFER> = DoipMessage {
        header: DoipHeader {
            protocol_version: ProtocolVersion::Iso13400_2012,
            inverse_protocol_version: 0xfd,
            payload_type: PayloadType::AliveCheckResponse,
            payload_length: 2u32,
        },
        payload: DoipPayload::AliveCheckResponse(AliveCheckResponse {
            source_address: [0x00, 0x00],
        }),
    };

    #[test]
    fn test_encode_alive_check_response_success() {
        let mut encoder = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = encoder.encode(SUCCESS_ROOT.clone(), &mut dst);

        assert!(bytes.is_ok(), "Expected bytes to be ok.");
        assert_eq!(
            *dst,
            [0x02, 0xfd, 0x00, 0x08, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00]
        );
    }

    #[test]
    fn test_decode_alive_check_response_success() {
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
    fn test_decode_alive_check_response_too_short() {
        let mut codec = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = &[0x02, 0xfd, 0x00, 0x08, 0x00, 0x00, 0x00, 0x02, 0x00];
        dst.extend_from_slice(bytes).unwrap();
        let msg = codec.decode(&mut dst);

        assert_eq!(msg.unwrap_err(), DecodeError::TooShort);
    }
}
