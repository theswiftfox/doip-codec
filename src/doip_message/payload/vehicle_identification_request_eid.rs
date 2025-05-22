use doip_definitions::{
    definitions::{DOIP_COMMON_EID_LEN, DOIP_HEADER_LEN},
    payload::{DoipPayload, VehicleIdentificationRequestEid},
};

use crate::{DecodeError, Decoder, EncodeError, Encoder};

#[derive(Debug)]
pub struct VehicleIdentificationRequestEidCodec;

impl Encoder<VehicleIdentificationRequestEid> for VehicleIdentificationRequestEidCodec {
    type Error = EncodeError;

    fn to_bytes(
        &mut self,
        item: VehicleIdentificationRequestEid,
        dst: &mut Vec<u8>,
    ) -> Result<(), Self::Error> {
        let VehicleIdentificationRequestEid { eid } = item;

        dst.extend_from_slice(&eid);

        Ok(())
    }
}

impl Decoder for VehicleIdentificationRequestEidCodec {
    type Item = DoipPayload;

    type Error = DecodeError;

    fn decode_from_bytes(&mut self, src: &[u8]) -> Result<Option<Self::Item>, Self::Error> {
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
        message::DoipMessage,
        payload::{DoipPayload, VehicleIdentificationRequestEid},
    };

    use crate::{Decoder, DoipCodec, Encoder};

    static SUCCESS_ROOT: DoipMessage = DoipMessage {
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
        let mut dst = Vec::<u8>::new();

        let bytes = encoder.to_bytes(SUCCESS_ROOT.clone(), &mut dst);

        assert!(bytes.is_ok(), "Expected bytes to be ok.");
        assert_eq!(
            *dst,
            [0x02, 0xfd, 0x00, 0x02, 0x00, 0x00, 0x00, 0x06, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06]
        );
    }

    #[test]
    fn test_decode_vehicle_identification_request_eid_success() {
        let mut codec = DoipCodec {};
        let mut dst = Vec::<u8>::new();

        let _ = codec.to_bytes(SUCCESS_ROOT.clone(), &mut dst);
        let msg = codec.decode_from_bytes(&mut dst);

        assert!(msg.is_ok());
        let opt = msg.unwrap();

        assert!(opt.is_some());
        let res = opt.unwrap();

        assert_eq!(res, SUCCESS_ROOT);
    }

    #[test]
    fn test_decode_vehicle_identification_request_eid_too_short() {
        let mut codec = DoipCodec {};
        let mut dst = Vec::<u8>::new();

        let bytes = &[0x02, 0xfd, 0x00, 0x02, 0x00, 0x00, 0x00, 0x06, 0xff];
        dst.extend_from_slice(bytes);
        let msg = codec.decode_from_bytes(&mut dst);

        assert!(msg.is_err());
    }
}
