use doip_definitions::{
    definitions::{DOIP_COMMON_VIN_LEN, DOIP_HEADER_LEN},
    payload::{DoipPayload, VehicleIdentificationRequestVin},
};

use crate::{DecodeError, Decoder, EncodeError, Encoder};

#[derive(Debug)]
pub struct VehicleIdentificationRequestVinCodec;

impl Encoder<VehicleIdentificationRequestVin> for VehicleIdentificationRequestVinCodec {
    type Error = EncodeError;

    fn to_bytes(
        &mut self,
        item: VehicleIdentificationRequestVin,
        dst: &mut Vec<u8>,
    ) -> Result<(), Self::Error> {
        let VehicleIdentificationRequestVin { vin } = item;

        dst.extend_from_slice(&vin);

        Ok(())
    }
}

impl Decoder for VehicleIdentificationRequestVinCodec {
    type Item = DoipPayload;

    type Error = DecodeError;

    fn decode_from_bytes(&mut self, src: &mut Vec<u8>) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < DOIP_HEADER_LEN + DOIP_COMMON_VIN_LEN {
            return Err(DecodeError::TooShort);
        }

        let item = VehicleIdentificationRequestVin {
            vin: src[DOIP_HEADER_LEN..DOIP_HEADER_LEN + DOIP_COMMON_VIN_LEN]
                .try_into()
                .expect("If failed, source has been manipulated at runtime."),
        };

        Ok(Some(DoipPayload::VehicleIdentificationRequestVin(item)))
    }
}

#[cfg(test)]
mod tests {
    use doip_definitions::{
        header::{DoipHeader, PayloadType, ProtocolVersion},
        message::DoipMessage,
        payload::{DoipPayload, VehicleIdentificationRequestVin},
    };

    use crate::{Decoder, DoipCodec, Encoder};

    static SUCCESS_ROOT: DoipMessage = DoipMessage {
        header: DoipHeader {
            protocol_version: ProtocolVersion::Iso13400_2012,
            inverse_protocol_version: 0xfd,
            payload_type: PayloadType::VehicleIdentificationRequestVin,
            payload_length: 17u32,
        },
        payload: DoipPayload::VehicleIdentificationRequestVin(VehicleIdentificationRequestVin {
            vin: [0u8; 17],
        }),
    };

    #[test]
    fn test_encode_vehicle_identification_request_vin_success() {
        let mut encoder = DoipCodec {};
        let mut dst = Vec::<u8>::new();

        let bytes = encoder.to_bytes(SUCCESS_ROOT.clone(), &mut dst);

        assert!(bytes.is_ok(), "Expected bytes to be ok.");
        assert_eq!(
            *dst,
            [
                0x02, 0xfd, 0x00, 0x03, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
            ]
        );
    }

    #[test]
    fn test_decode_vehicle_identification_request_vin_success() {
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
    fn test_decode_vehicle_identification_request_vin_too_short() {
        let mut codec = DoipCodec {};
        let mut dst = Vec::<u8>::new();

        let bytes = &[0x02, 0xfd, 0x00, 0x03, 0x00, 0x00, 0x00, 0x06, 0xff];
        dst.extend_from_slice(bytes);
        let msg = codec.decode_from_bytes(&mut dst);

        assert!(msg.is_err());
    }
}
