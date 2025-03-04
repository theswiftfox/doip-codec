use doip_definitions::{
    definitions::{
        DOIP_COMMON_EID_LEN, DOIP_COMMON_VIN_LEN, DOIP_HEADER_LEN, DOIP_VEHICLE_ANNOUNCEMENT_ACTION_OFFSET,
        DOIP_VEHICLE_ANNOUNCEMENT_ADDRESS_LEN, DOIP_VEHICLE_ANNOUNCEMENT_ADDRESS_OFFSET,
        DOIP_VEHICLE_ANNOUNCEMENT_EID_OFFSET, DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN,
        DOIP_VEHICLE_ANNOUNCEMENT_GID_OFFSET, DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG,
        DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT,
        DOIP_VEHICLE_ANNOUNCEMENT_SYNC_OFFSET,
    },
    payload::{ActionCode, DoipPayload, SyncStatus, VehicleAnnouncementMessage},
};
use heapless::Vec;

use crate::{
    doip_message::header::HeaderCodec, DecodeError, Decoder, EncodeError, Encoder, FromBytes,
    ToBytes,
};

#[derive(Debug)]
pub struct VehicleAnnouncementMessageCodec;

impl<const N: usize> Encoder<VehicleAnnouncementMessage, N> for VehicleAnnouncementMessageCodec {
    type Error = EncodeError;

    fn encode(
        &mut self,
        item: VehicleAnnouncementMessage,
        dst: &mut Vec<u8, N>,
    ) -> Result<(), Self::Error> {
        let VehicleAnnouncementMessage {
            vin,
            logical_address,
            eid,
            gid,
            further_action,
            vin_gid_sync,
        } = item;

        dst.extend_from_slice(&vin).map_err(|_| EncodeError::BufferTooSmall)?;

        dst.extend_from_slice(&logical_address).map_err(|_| EncodeError::BufferTooSmall)?;

        dst.extend_from_slice(&eid).map_err(|_| EncodeError::BufferTooSmall)?;

        dst.extend_from_slice(&gid).map_err(|_| EncodeError::BufferTooSmall)?;

        let further_action_bytes = further_action.to_bytes();
        dst.extend_from_slice(further_action_bytes).map_err(|_| EncodeError::BufferTooSmall)?;

        if let Some(sync_status) = vin_gid_sync {
            let sync_status_bytes = sync_status.to_bytes();
            dst.extend_from_slice(sync_status_bytes).map_err(|_| EncodeError::BufferTooSmall)?;
        }

        Ok(())
    }
}

impl ToBytes for ActionCode {
    fn to_bytes(self) -> &'static [u8] {
        match self {
            ActionCode::NoFurtherActionRequired => &[ActionCode::NoFurtherActionRequired as u8],
            ActionCode::ReservedByIso13400_01 => &[ActionCode::ReservedByIso13400_01 as u8],
            ActionCode::ReservedByIso13400_02 => &[ActionCode::ReservedByIso13400_02 as u8],
            ActionCode::ReservedByIso13400_03 => &[ActionCode::ReservedByIso13400_03 as u8],
            ActionCode::ReservedByIso13400_04 => &[ActionCode::ReservedByIso13400_04 as u8],
            ActionCode::ReservedByIso13400_05 => &[ActionCode::ReservedByIso13400_05 as u8],
            ActionCode::ReservedByIso13400_06 => &[ActionCode::ReservedByIso13400_06 as u8],
            ActionCode::ReservedByIso13400_07 => &[ActionCode::ReservedByIso13400_07 as u8],
            ActionCode::ReservedByIso13400_08 => &[ActionCode::ReservedByIso13400_08 as u8],
            ActionCode::ReservedByIso13400_09 => &[ActionCode::ReservedByIso13400_09 as u8],
            ActionCode::ReservedByIso13400_0A => &[ActionCode::ReservedByIso13400_0A as u8],
            ActionCode::ReservedByIso13400_0B => &[ActionCode::ReservedByIso13400_0B as u8],
            ActionCode::ReservedByIso13400_0C => &[ActionCode::ReservedByIso13400_0C as u8],
            ActionCode::ReservedByIso13400_0D => &[ActionCode::ReservedByIso13400_0D as u8],
            ActionCode::ReservedByIso13400_0E => &[ActionCode::ReservedByIso13400_0E as u8],
            ActionCode::ReservedByIso13400_0F => &[ActionCode::ReservedByIso13400_0F as u8],
            ActionCode::RoutingActivationRequired => &[ActionCode::RoutingActivationRequired as u8],
        }
    }
}

impl ToBytes for SyncStatus {
    fn to_bytes(self) -> &'static [u8] {
        match self {
            SyncStatus::VinGidSynchronized => &[SyncStatus::VinGidSynchronized as u8],
            SyncStatus::ReservedByIso13400_01 => &[SyncStatus::ReservedByIso13400_01 as u8],
            SyncStatus::ReservedByIso13400_02 => &[SyncStatus::ReservedByIso13400_02 as u8],
            SyncStatus::ReservedByIso13400_03 => &[SyncStatus::ReservedByIso13400_03 as u8],
            SyncStatus::ReservedByIso13400_04 => &[SyncStatus::ReservedByIso13400_04 as u8],
            SyncStatus::ReservedByIso13400_05 => &[SyncStatus::ReservedByIso13400_05 as u8],
            SyncStatus::ReservedByIso13400_06 => &[SyncStatus::ReservedByIso13400_06 as u8],
            SyncStatus::ReservedByIso13400_07 => &[SyncStatus::ReservedByIso13400_07 as u8],
            SyncStatus::ReservedByIso13400_08 => &[SyncStatus::ReservedByIso13400_08 as u8],
            SyncStatus::ReservedByIso13400_09 => &[SyncStatus::ReservedByIso13400_09 as u8],
            SyncStatus::ReservedByIso13400_0A => &[SyncStatus::ReservedByIso13400_0A as u8],
            SyncStatus::ReservedByIso13400_0B => &[SyncStatus::ReservedByIso13400_0B as u8],
            SyncStatus::ReservedByIso13400_0C => &[SyncStatus::ReservedByIso13400_0C as u8],
            SyncStatus::ReservedByIso13400_0D => &[SyncStatus::ReservedByIso13400_0D as u8],
            SyncStatus::ReservedByIso13400_0E => &[SyncStatus::ReservedByIso13400_0E as u8],
            SyncStatus::ReservedByIso13400_0F => &[SyncStatus::ReservedByIso13400_0F as u8],
            SyncStatus::VinGidNotSynchronised => &[SyncStatus::VinGidNotSynchronised as u8],
        }
    }
}

impl<const N: usize> Decoder<N> for VehicleAnnouncementMessageCodec {
    type Item = DoipPayload<N>;

    type Error = DecodeError;

    fn decode(&mut self, src: &mut Vec<u8, N>) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < DOIP_HEADER_LEN + DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT {
            return Err(DecodeError::TooShort);
        }

        let mut h_codec = HeaderCodec {};

        let header = h_codec.decode(src)?.expect("Should never return Ok(None)");

        let vin = src[DOIP_HEADER_LEN..DOIP_HEADER_LEN + DOIP_COMMON_VIN_LEN]
            .try_into()
            .expect("If failed, source has been manipulated at runtime.");

        let logical_address = src[DOIP_VEHICLE_ANNOUNCEMENT_ADDRESS_OFFSET
            ..DOIP_VEHICLE_ANNOUNCEMENT_ADDRESS_OFFSET + DOIP_VEHICLE_ANNOUNCEMENT_ADDRESS_LEN]
            .try_into()
            .expect("If failed, source has been manipulated at runtime.");

        let eid = src[DOIP_VEHICLE_ANNOUNCEMENT_EID_OFFSET
            ..DOIP_VEHICLE_ANNOUNCEMENT_EID_OFFSET + DOIP_COMMON_EID_LEN]
            .try_into()
            .expect("If failed, source has been manipulated at runtime.");

        let gid = src[DOIP_VEHICLE_ANNOUNCEMENT_GID_OFFSET
            ..DOIP_VEHICLE_ANNOUNCEMENT_GID_OFFSET + DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN]
            .try_into()
            .expect("If failed, source has been manipulated at runtime.");

        let further_action_bytes = src[DOIP_VEHICLE_ANNOUNCEMENT_ACTION_OFFSET..=DOIP_VEHICLE_ANNOUNCEMENT_ACTION_OFFSET]
            .try_into()
            .expect("If failed, source has been manipulated at runtime.");

        let further_action =
            ActionCode::from_bytes(further_action_bytes).ok_or(DecodeError::InvalidActionCode)?;

        // Determine if the sync status byte is present based on payload length
        let expected_payload_length = DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG;
        let vin_gid_sync = if header.payload_length as usize == expected_payload_length {
            let bytes = &src[DOIP_VEHICLE_ANNOUNCEMENT_SYNC_OFFSET..=DOIP_VEHICLE_ANNOUNCEMENT_SYNC_OFFSET];
            Some(SyncStatus::from_bytes(bytes).ok_or(DecodeError::InvalidSyncStatus)?)
        } else {
            None
        };

        let item = VehicleAnnouncementMessage {
            vin,
            logical_address,
            eid,
            gid,
            further_action,
            vin_gid_sync,
        };

        Ok(Some(DoipPayload::VehicleAnnouncementMessage(item)))
    }
}

impl FromBytes for ActionCode {
    fn from_bytes(bytes: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        let val = *bytes.first()?;

        match val {
            v if v == ActionCode::NoFurtherActionRequired as u8 => {
                Some(ActionCode::NoFurtherActionRequired)
            }
            v if v == ActionCode::ReservedByIso13400_01 as u8 => {
                Some(ActionCode::ReservedByIso13400_01)
            }
            v if v == ActionCode::ReservedByIso13400_02 as u8 => {
                Some(ActionCode::ReservedByIso13400_02)
            }
            v if v == ActionCode::ReservedByIso13400_03 as u8 => {
                Some(ActionCode::ReservedByIso13400_03)
            }
            v if v == ActionCode::ReservedByIso13400_04 as u8 => {
                Some(ActionCode::ReservedByIso13400_04)
            }
            v if v == ActionCode::ReservedByIso13400_05 as u8 => {
                Some(ActionCode::ReservedByIso13400_05)
            }
            v if v == ActionCode::ReservedByIso13400_06 as u8 => {
                Some(ActionCode::ReservedByIso13400_06)
            }
            v if v == ActionCode::ReservedByIso13400_07 as u8 => {
                Some(ActionCode::ReservedByIso13400_07)
            }
            v if v == ActionCode::ReservedByIso13400_08 as u8 => {
                Some(ActionCode::ReservedByIso13400_08)
            }
            v if v == ActionCode::ReservedByIso13400_09 as u8 => {
                Some(ActionCode::ReservedByIso13400_09)
            }
            v if v == ActionCode::ReservedByIso13400_0A as u8 => {
                Some(ActionCode::ReservedByIso13400_0A)
            }
            v if v == ActionCode::ReservedByIso13400_0B as u8 => {
                Some(ActionCode::ReservedByIso13400_0B)
            }
            v if v == ActionCode::ReservedByIso13400_0C as u8 => {
                Some(ActionCode::ReservedByIso13400_0C)
            }
            v if v == ActionCode::ReservedByIso13400_0D as u8 => {
                Some(ActionCode::ReservedByIso13400_0D)
            }
            v if v == ActionCode::ReservedByIso13400_0E as u8 => {
                Some(ActionCode::ReservedByIso13400_0E)
            }
            v if v == ActionCode::ReservedByIso13400_0F as u8 => {
                Some(ActionCode::ReservedByIso13400_0F)
            }
            v if v == ActionCode::RoutingActivationRequired as u8 => {
                Some(ActionCode::RoutingActivationRequired)
            }
            _ => None,
        }
    }
}

impl FromBytes for SyncStatus {
    fn from_bytes(bytes: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        let val = *bytes.first()?;

        match val {
            v if v == SyncStatus::VinGidSynchronized as u8 => Some(SyncStatus::VinGidSynchronized),
            v if v == SyncStatus::ReservedByIso13400_01 as u8 => {
                Some(SyncStatus::ReservedByIso13400_01)
            }
            v if v == SyncStatus::ReservedByIso13400_02 as u8 => {
                Some(SyncStatus::ReservedByIso13400_02)
            }
            v if v == SyncStatus::ReservedByIso13400_03 as u8 => {
                Some(SyncStatus::ReservedByIso13400_03)
            }
            v if v == SyncStatus::ReservedByIso13400_04 as u8 => {
                Some(SyncStatus::ReservedByIso13400_04)
            }
            v if v == SyncStatus::ReservedByIso13400_05 as u8 => {
                Some(SyncStatus::ReservedByIso13400_05)
            }
            v if v == SyncStatus::ReservedByIso13400_06 as u8 => {
                Some(SyncStatus::ReservedByIso13400_06)
            }
            v if v == SyncStatus::ReservedByIso13400_07 as u8 => {
                Some(SyncStatus::ReservedByIso13400_07)
            }
            v if v == SyncStatus::ReservedByIso13400_08 as u8 => {
                Some(SyncStatus::ReservedByIso13400_08)
            }
            v if v == SyncStatus::ReservedByIso13400_09 as u8 => {
                Some(SyncStatus::ReservedByIso13400_09)
            }
            v if v == SyncStatus::ReservedByIso13400_0A as u8 => {
                Some(SyncStatus::ReservedByIso13400_0A)
            }
            v if v == SyncStatus::ReservedByIso13400_0B as u8 => {
                Some(SyncStatus::ReservedByIso13400_0B)
            }
            v if v == SyncStatus::ReservedByIso13400_0C as u8 => {
                Some(SyncStatus::ReservedByIso13400_0C)
            }
            v if v == SyncStatus::ReservedByIso13400_0D as u8 => {
                Some(SyncStatus::ReservedByIso13400_0D)
            }
            v if v == SyncStatus::ReservedByIso13400_0E as u8 => {
                Some(SyncStatus::ReservedByIso13400_0E)
            }
            v if v == SyncStatus::ReservedByIso13400_0F as u8 => {
                Some(SyncStatus::ReservedByIso13400_0F)
            }
            v if v == SyncStatus::VinGidNotSynchronised as u8 => {
                Some(SyncStatus::VinGidNotSynchronised)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{DecodeError, Decoder, DoipCodec, Encoder, FromBytes, ToBytes};
    use doip_definitions::{
        header::{DoipHeader, PayloadType, ProtocolVersion},
        payload::{ActionCode, DoipPayload, SyncStatus, VehicleAnnouncementMessage},
        DoipMessage,
    };
    use heapless::Vec;

    const BUFFER: usize = 4095;

    static SUCCESS_ROOT_NO_SYNC: DoipMessage<BUFFER> = DoipMessage {
        header: DoipHeader {
            protocol_version: ProtocolVersion::Iso13400_2012,
            inverse_protocol_version: 0xfd,
            payload_type: PayloadType::VehicleAnnouncementMessage,
            payload_length: 32u32,
        },
        payload: DoipPayload::VehicleAnnouncementMessage(VehicleAnnouncementMessage {
            vin: [0u8; 17],
            logical_address: [0u8; 2],
            eid: [0u8; 6],
            gid: [0u8; 6],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: None,
        }),
    };

    static SUCCESS_ROOT_WITH_SYNC: DoipMessage<BUFFER> = DoipMessage {
        header: DoipHeader {
            protocol_version: ProtocolVersion::Iso13400_2012,
            inverse_protocol_version: 0xfd,
            payload_type: PayloadType::VehicleAnnouncementMessage,
            payload_length: 33u32,
        },
        payload: DoipPayload::VehicleAnnouncementMessage(VehicleAnnouncementMessage {
            vin: [0u8; 17],
            logical_address: [0u8; 2],
            eid: [0u8; 6],
            gid: [0u8; 6],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: Some(SyncStatus::VinGidSynchronized),
        }),
    };

    #[test]
    fn test_action_code_to_bytes() {
        let bytes = ActionCode::NoFurtherActionRequired.to_bytes();
        assert_eq!(bytes, &[0x0]);
        let bytes = ActionCode::ReservedByIso13400_01.to_bytes();
        assert_eq!(bytes, &[0x01]);
        let bytes = ActionCode::ReservedByIso13400_02.to_bytes();
        assert_eq!(bytes, &[0x02]);
        let bytes = ActionCode::ReservedByIso13400_03.to_bytes();
        assert_eq!(bytes, &[0x03]);
        let bytes = ActionCode::ReservedByIso13400_04.to_bytes();
        assert_eq!(bytes, &[0x04]);
        let bytes = ActionCode::ReservedByIso13400_05.to_bytes();
        assert_eq!(bytes, &[0x05]);
        let bytes = ActionCode::ReservedByIso13400_06.to_bytes();
        assert_eq!(bytes, &[0x06]);
        let bytes = ActionCode::ReservedByIso13400_07.to_bytes();
        assert_eq!(bytes, &[0x07]);
        let bytes = ActionCode::ReservedByIso13400_08.to_bytes();
        assert_eq!(bytes, &[0x08]);
        let bytes = ActionCode::ReservedByIso13400_09.to_bytes();
        assert_eq!(bytes, &[0x09]);
        let bytes = ActionCode::ReservedByIso13400_0A.to_bytes();
        assert_eq!(bytes, &[0x0a]);
        let bytes = ActionCode::ReservedByIso13400_0B.to_bytes();
        assert_eq!(bytes, &[0x0b]);
        let bytes = ActionCode::ReservedByIso13400_0C.to_bytes();
        assert_eq!(bytes, &[0x0c]);
        let bytes = ActionCode::ReservedByIso13400_0D.to_bytes();
        assert_eq!(bytes, &[0x0d]);
        let bytes = ActionCode::ReservedByIso13400_0E.to_bytes();
        assert_eq!(bytes, &[0x0e]);
        let bytes = ActionCode::ReservedByIso13400_0F.to_bytes();
        assert_eq!(bytes, &[0x0f]);
        let bytes = ActionCode::RoutingActivationRequired.to_bytes();
        assert_eq!(bytes, &[0x10]);
    }

    #[test]
    fn test_sync_status_to_bytes() {
        let bytes = SyncStatus::VinGidSynchronized.to_bytes();
        assert_eq!(bytes, &[0x00]);
        let bytes = SyncStatus::ReservedByIso13400_01.to_bytes();
        assert_eq!(bytes, &[0x01]);
        let bytes = SyncStatus::ReservedByIso13400_02.to_bytes();
        assert_eq!(bytes, &[0x02]);
        let bytes = SyncStatus::ReservedByIso13400_03.to_bytes();
        assert_eq!(bytes, &[0x03]);
        let bytes = SyncStatus::ReservedByIso13400_04.to_bytes();
        assert_eq!(bytes, &[0x04]);
        let bytes = SyncStatus::ReservedByIso13400_05.to_bytes();
        assert_eq!(bytes, &[0x05]);
        let bytes = SyncStatus::ReservedByIso13400_06.to_bytes();
        assert_eq!(bytes, &[0x06]);
        let bytes = SyncStatus::ReservedByIso13400_07.to_bytes();
        assert_eq!(bytes, &[0x07]);
        let bytes = SyncStatus::ReservedByIso13400_08.to_bytes();
        assert_eq!(bytes, &[0x08]);
        let bytes = SyncStatus::ReservedByIso13400_09.to_bytes();
        assert_eq!(bytes, &[0x09]);
        let bytes = SyncStatus::ReservedByIso13400_0A.to_bytes();
        assert_eq!(bytes, &[0x0A]);
        let bytes = SyncStatus::ReservedByIso13400_0B.to_bytes();
        assert_eq!(bytes, &[0x0B]);
        let bytes = SyncStatus::ReservedByIso13400_0C.to_bytes();
        assert_eq!(bytes, &[0x0C]);
        let bytes = SyncStatus::ReservedByIso13400_0D.to_bytes();
        assert_eq!(bytes, &[0x0D]);
        let bytes = SyncStatus::ReservedByIso13400_0E.to_bytes();
        assert_eq!(bytes, &[0x0E]);
        let bytes = SyncStatus::ReservedByIso13400_0F.to_bytes();
        assert_eq!(bytes, &[0x0F]);
        let bytes = SyncStatus::VinGidNotSynchronised.to_bytes();
        assert_eq!(bytes, &[0x10]);
    }

    #[test]
    fn test_action_code_from_bytes() {
        for a in u8::MIN..=u8::MAX {
            let bytes = &[a];
            let proto = ActionCode::from_bytes(bytes);

            match a {
                0x00 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActionCode::NoFurtherActionRequired)
                }
                0x01 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActionCode::ReservedByIso13400_01)
                }
                0x02 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActionCode::ReservedByIso13400_02)
                }
                0x03 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActionCode::ReservedByIso13400_03)
                }
                0x04 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActionCode::ReservedByIso13400_04)
                }
                0x05 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActionCode::ReservedByIso13400_05)
                }
                0x06 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActionCode::ReservedByIso13400_06)
                }
                0x07 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActionCode::ReservedByIso13400_07)
                }
                0x08 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActionCode::ReservedByIso13400_08)
                }
                0x09 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActionCode::ReservedByIso13400_09)
                }
                0x0a => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActionCode::ReservedByIso13400_0A)
                }
                0x0b => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActionCode::ReservedByIso13400_0B)
                }
                0x0c => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActionCode::ReservedByIso13400_0C)
                }
                0x0d => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActionCode::ReservedByIso13400_0D)
                }
                0x0e => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActionCode::ReservedByIso13400_0E)
                }
                0x0f => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActionCode::ReservedByIso13400_0F)
                }
                0x10 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), ActionCode::RoutingActivationRequired)
                }
                _ => {
                    assert!(proto.is_none())
                }
            }
        }
    }

    #[test]
    fn test_sync_status_from_bytes() {
        for a in u8::MIN..=u8::MAX {
            let bytes = &[a];
            let proto = SyncStatus::from_bytes(bytes);

            match a {
                0x00 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), SyncStatus::VinGidSynchronized)
                }
                0x01 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), SyncStatus::ReservedByIso13400_01)
                }
                0x02 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), SyncStatus::ReservedByIso13400_02)
                }
                0x03 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), SyncStatus::ReservedByIso13400_03)
                }
                0x04 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), SyncStatus::ReservedByIso13400_04)
                }
                0x05 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), SyncStatus::ReservedByIso13400_05)
                }
                0x06 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), SyncStatus::ReservedByIso13400_06)
                }
                0x07 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), SyncStatus::ReservedByIso13400_07)
                }
                0x08 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), SyncStatus::ReservedByIso13400_08)
                }
                0x09 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), SyncStatus::ReservedByIso13400_09)
                }
                0x0a => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), SyncStatus::ReservedByIso13400_0A)
                }
                0x0b => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), SyncStatus::ReservedByIso13400_0B)
                }
                0x0c => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), SyncStatus::ReservedByIso13400_0C)
                }
                0x0d => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), SyncStatus::ReservedByIso13400_0D)
                }
                0x0e => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), SyncStatus::ReservedByIso13400_0E)
                }
                0x0f => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), SyncStatus::ReservedByIso13400_0F)
                }
                0x10 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), SyncStatus::VinGidNotSynchronised)
                }
                _ => {
                    assert!(proto.is_none())
                }
            }
        }
    }

    #[test]
    fn test_encode_vehicle_announcement_message_vin_no_sync_success() {
        let mut encoder = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = encoder.encode(SUCCESS_ROOT_NO_SYNC.clone(), &mut dst);

        assert!(bytes.is_ok(), "Expected bytes to be ok.");
        assert_eq!(
            *dst,
            [
                0x02, 0xfd, 0x00, 0x04, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
            ]
        );
    }

    #[test]
    fn test_encode_vehicle_announcement_message_vin_with_sync_success() {
        let mut encoder = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = encoder.encode(SUCCESS_ROOT_WITH_SYNC.clone(), &mut dst);

        assert!(bytes.is_ok(), "Expected bytes to be ok.");
        assert_eq!(
            *dst,
            [
                0x02, 0xfd, 0x00, 0x04, 0x00, 0x00, 0x00, 0x21, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
            ]
        );
    }

    #[test]
    fn test_decode_vehicle_announcement_message_vin_no_sync_success() {
        let mut codec = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let _ = codec.encode(SUCCESS_ROOT_NO_SYNC.clone(), &mut dst);
        let msg = codec.decode(&mut dst);

        assert!(msg.is_ok());
        let opt = msg.unwrap();

        assert!(opt.is_some());
        let res = opt.unwrap();

        assert_eq!(res, SUCCESS_ROOT_NO_SYNC)
    }

    #[test]
    fn test_decode_vehicle_announcement_message_vin_with_sync_success() {
        let mut codec = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let _ = codec.encode(SUCCESS_ROOT_WITH_SYNC.clone(), &mut dst);
        let msg = codec.decode(&mut dst);

        assert!(msg.is_ok());
        let opt = msg.unwrap();

        assert!(opt.is_some());
        let res = opt.unwrap();

        assert_eq!(res, SUCCESS_ROOT_WITH_SYNC)
    }

    #[test]
    fn test_decode_vehicle_announcement_message_vin_with_sync_too_short() {
        let mut codec = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = &[0x02, 0xfd, 0x00, 0x04, 0x00, 0x00, 0x00, 0x06, 0xff];
        dst.extend_from_slice(bytes).unwrap();
        let msg = codec.decode(&mut dst);

        assert_eq!(msg.unwrap_err(), DecodeError::TooShort);
    }

    #[test]
    fn test_decode_vehicle_announcement_message_vin_with_sync_invalid_action_code() {
        let mut codec = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = &[
            0x02, 0xfd, 0x00, 0x04, 0x00, 0x00, 0x00, 0x21, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x42, 0x00,
        ];
        dst.extend_from_slice(bytes).unwrap();
        let msg = codec.decode(&mut dst);

        assert_eq!(msg.unwrap_err(), DecodeError::InvalidActionCode);
    }

    #[test]
    fn test_decode_vehicle_announcement_message_vin_with_sync_invalid_sync_status() {
        let mut codec = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = &[
            0x02, 0xfd, 0x00, 0x04, 0x00, 0x00, 0x00, 0x21, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x42,
        ];
        dst.extend_from_slice(bytes).unwrap();
        let msg = codec.decode(&mut dst);

        assert_eq!(msg.unwrap_err(), DecodeError::InvalidSyncStatus);
    }
}
