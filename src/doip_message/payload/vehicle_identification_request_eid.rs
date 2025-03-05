use doip_definitions::{
    definitions::{DOIP_COMMON_EID_LEN, DOIP_HEADER_LEN},
    payload::{DoipPayload, VehicleIdentificationRequestEid},
};
use heapless::Vec;

use crate::{DecodeError, Decoder, EncodeError, Encoder};

#[derive(Debug)]
pub struct VehicleIdentificationRequestEidCodec;

impl<const N: usize> Encoder<VehicleIdentificationRequestEid, N>
    for VehicleIdentificationRequestEidCodec
{
    type Error = EncodeError;

    fn encode(
        &mut self,
        item: VehicleIdentificationRequestEid,
        dst: &mut Vec<u8, N>,
    ) -> Result<(), Self::Error> {
        let VehicleIdentificationRequestEid { eid } = item;

        dst.extend_from_slice(&eid).map_err(|()| EncodeError::BufferTooSmall)?;

        Ok(())
    }
}

impl<const N: usize> Decoder<N> for VehicleIdentificationRequestEidCodec {
    type Item = DoipPayload<N>;

    type Error = DecodeError;

    fn decode(&mut self, src: &mut Vec<u8, N>) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < DOIP_HEADER_LEN + DOIP_COMMON_EID_LEN {
            return Err(DecodeError::TooShort);
        }

        let item = VehicleIdentificationRequestEid {
            eid: src[DOIP_HEADER_LEN..DOIP_HEADER_LEN + DOIP_COMMON_EID_LEN]
                .try_into()
                .expect("If failed, source has been manipulated at runtime."),
        };

        Ok(Some(DoipPayload::VehicleIdentificationRequestEid(item)))
    }
}

#[cfg(test)]
mod tests {
    use doip_definitions::{
        header::{DoipHeader, PayloadType, ProtocolVersion},
        payload::{DoipPayload, VehicleIdentificationRequestEid},
        DoipMessage,
    };
    use heapless::Vec;

    use crate::{DecodeError, Decoder, DoipCodec, Encoder};

    const BUFFER: usize = 4095;

    static SUCCESS_ROOT: DoipMessage<BUFFER> = DoipMessage {
        header: DoipHeader {
            protocol_version: ProtocolVersion::Iso13400_2012,
            inverse_protocol_version: 0xfd,
            payload_type: PayloadType::VehicleIdentificationRequestEid,
            payload_length: 6u32,
        },
        payload: DoipPayload::VehicleIdentificationRequestEid(VehicleIdentificationRequestEid {
            eid: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06],
        }),
    };

    #[test]
    fn test_encode_vehicle_identification_request_eid_success() {
        let mut encoder = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = encoder.encode(SUCCESS_ROOT.clone(), &mut dst);

        assert!(bytes.is_ok(), "Expected bytes to be ok.");
        assert_eq!(
            *dst,
            [0x02, 0xfd, 0x00, 0x02, 0x00, 0x00, 0x00, 0x06, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06]
        )
    }

    #[test]
    fn test_decode_vehicle_identification_request_eid_success() {
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
    fn test_decode_vehicle_identification_request_eid_too_short() {
        let mut codec = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = &[0x02, 0xfd, 0x00, 0x02, 0x00, 0x00, 0x00, 0x06, 0xff];
        dst.extend_from_slice(bytes).unwrap();
        let msg = codec.decode(&mut dst);

        assert!(msg.is_err());
    }
}
