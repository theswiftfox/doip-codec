use doip_definitions::{
    definitions::DOIP_HEADER_LEN,
    payload::{DoipPayload, PowerInformationRequest},
};
use heapless::Vec;

use crate::{DecodeError, Decoder, EncodeError, Encoder};

#[derive(Debug)]
pub struct PowerInformationRequestCodec;

impl<const N: usize> Encoder<PowerInformationRequest, N> for PowerInformationRequestCodec {
    type Error = EncodeError;

    fn encode(
        &mut self,
        item: PowerInformationRequest,
        _dst: &mut Vec<u8, N>,
    ) -> Result<(), Self::Error> {
        let PowerInformationRequest {} = item;

        Ok(())
    }
}

impl<const N: usize> Decoder<N> for PowerInformationRequestCodec {
    type Item = DoipPayload<N>;

    type Error = DecodeError;

    fn decode(&mut self, src: &mut Vec<u8, N>) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < DOIP_HEADER_LEN {
            return Err(DecodeError::TooShort);
        }

        Ok(Some(DoipPayload::PowerInformationRequest(
            PowerInformationRequest {},
        )))
    }
}

#[cfg(test)]
mod tests {
    use doip_definitions::{
        header::{DoipHeader, PayloadType, ProtocolVersion},
        payload::{DoipPayload, PowerInformationRequest},
        DoipMessage,
    };
    use heapless::Vec;

    use crate::{
        doip_message::payload::power_information_request::PowerInformationRequestCodec,
        DecodeError, Decoder, DoipCodec, Encoder,
    };
    const BUFFER: usize = 4095;

    static SUCCESS_ROOT: DoipMessage<BUFFER> = DoipMessage {
        header: DoipHeader {
            protocol_version: ProtocolVersion::Iso13400_2012,
            inverse_protocol_version: 0xfd,
            payload_type: PayloadType::PowerInformationRequest,
            payload_length: 0u32,
        },
        payload: DoipPayload::PowerInformationRequest(PowerInformationRequest {}),
    };

    #[test]
    fn test_encode_power_information_request_success() {
        let mut encoder = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = encoder.encode(SUCCESS_ROOT.clone(), &mut dst);

        assert!(bytes.is_ok(), "Expected bytes to be ok.");
        assert_eq!(*dst, [0x02, 0xfd, 0x40, 0x03, 0x00, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_decode_power_information_request_success() {
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
    fn test_decode_power_information_request_too_short() {
        let mut codec = PowerInformationRequestCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = &[0x02, 0xfd, 0x40, 0x03, 0x00, 0x00, 0x00];
        dst.extend_from_slice(bytes).unwrap();
        let msg = codec.decode(&mut dst);

        assert_eq!(msg.unwrap_err(), DecodeError::TooShort);
    }
}
