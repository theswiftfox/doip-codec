use doip_definitions::{
    definitions::{DOIP_HEADER_LEN, DOIP_POWER_MODE_LEN},
    payload::{DoipPayload, PowerInformationResponse, PowerMode},
};
use heapless::Vec;

use crate::{DecodeError, Decoder, EncodeError, Encoder, FromBytes, ToBytes};

#[derive(Debug)]
pub struct PowerInformationResponseCodec;

impl<const N: usize> Encoder<PowerInformationResponse, N> for PowerInformationResponseCodec {
    type Error = EncodeError;

    fn encode(
        &mut self,
        item: PowerInformationResponse,
        dst: &mut Vec<u8, N>,
    ) -> Result<(), Self::Error> {
        let PowerInformationResponse { power_mode } = item;

        let power_mode_bytes = power_mode.to_bytes();
        dst.extend_from_slice(power_mode_bytes).map_err(|_| EncodeError::BufferTooSmall)?;

        Ok(())
    }
}

impl ToBytes for PowerMode {
    fn to_bytes(self) -> &'static [u8] {
        match self {
            PowerMode::NotReady => &[PowerMode::NotReady as u8],
            PowerMode::Ready => &[PowerMode::Ready as u8],
            PowerMode::NotSupported => &[PowerMode::NotSupported as u8],
        }
    }
}

impl<const N: usize> Decoder<N> for PowerInformationResponseCodec {
    type Item = DoipPayload<N>;

    type Error = DecodeError;

    fn decode(&mut self, src: &mut Vec<u8, N>) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < DOIP_HEADER_LEN + DOIP_POWER_MODE_LEN {
            return Err(DecodeError::TooShort);
        }

        let power_mode_bytes = &src[DOIP_HEADER_LEN..=DOIP_HEADER_LEN];
        let power_mode =
            PowerMode::from_bytes(power_mode_bytes).ok_or(DecodeError::InvalidPowerMode)?;

        let item = PowerInformationResponse { power_mode };

        Ok(Some(DoipPayload::PowerInformationResponse(item)))
    }
}

impl FromBytes for PowerMode {
    fn from_bytes(bytes: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        let val = *bytes.first()?;

        match val {
            v if v == PowerMode::NotReady as u8 => Some(PowerMode::NotReady),
            v if v == PowerMode::Ready as u8 => Some(PowerMode::Ready),
            v if v == PowerMode::NotSupported as u8 => Some(PowerMode::NotSupported),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use doip_definitions::{
        header::{DoipHeader, PayloadType, ProtocolVersion},
        payload::{DoipPayload, PowerInformationResponse, PowerMode},
        DoipMessage,
    };
    use heapless::Vec;

    use crate::{DecodeError, Decoder, DoipCodec, Encoder, FromBytes, ToBytes};

    const BUFFER: usize = 4095;
    static SUCCESS_ROOT: DoipMessage<BUFFER> = DoipMessage {
        header: DoipHeader {
            protocol_version: ProtocolVersion::Iso13400_2012,
            inverse_protocol_version: 0xfd,
            payload_type: PayloadType::PowerInformationResponse,
            payload_length: 1u32,
        },
        payload: DoipPayload::PowerInformationResponse(PowerInformationResponse {
            power_mode: PowerMode::Ready,
        }),
    };

    #[test]
    fn test_power_mode_to_bytes() {
        let bytes = PowerMode::NotReady.to_bytes();
        assert_eq!(bytes, &[0x00]);
        let bytes = PowerMode::Ready.to_bytes();
        assert_eq!(bytes, &[0x01]);
        let bytes = PowerMode::NotSupported.to_bytes();
        assert_eq!(bytes, &[0x02]);
    }

    #[test]
    fn test_power_mode_from_bytes() {
        for a in u8::MIN..=u8::MAX {
            let bytes = &[a];
            let proto = PowerMode::from_bytes(bytes);
            match a {
                0x00 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), PowerMode::NotReady)
                }
                0x01 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), PowerMode::Ready)
                }

                0x02 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), PowerMode::NotSupported)
                }
                _ => {
                    assert!(proto.is_none())
                }
            }
        }
    }

    #[test]
    fn test_encode_power_information_response_success() {
        let mut encoder = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = encoder.encode(SUCCESS_ROOT.clone(), &mut dst);

        assert!(bytes.is_ok(), "Expected bytes to be ok.");
        assert_eq!(*dst, [0x02, 0xfd, 0x40, 0x04, 0x00, 0x00, 0x00, 0x01, 0x01]);
    }

    #[test]
    fn test_decode_power_information_response_success() {
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
    fn test_decode_power_information_response_invalid_power_mode() {
        let mut codec = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = &[0x02, 0xfd, 0x40, 0x04, 0x00, 0x00, 0x00, 0x01, 0x42];
        dst.extend_from_slice(bytes).unwrap();
        let msg = codec.decode(&mut dst);

        assert_eq!(msg.unwrap_err(), DecodeError::InvalidPowerMode);
    }

    #[test]
    fn test_decode_power_information_response_too_short() {
        let mut codec = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = &[0x02, 0xfd, 0x40, 0x04, 0x00, 0x00, 0x00, 0x01];
        dst.extend_from_slice(bytes).unwrap();
        let msg = codec.decode(&mut dst);

        assert_eq!(msg.unwrap_err(), DecodeError::TooShort);
    }
}
