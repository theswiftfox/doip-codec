use doip_definitions::{
    definitions::{
        DOIP_ENTITY_STATUS_RESPONSE_LEN,
        DOIP_ENTITY_STATUS_RESPONSE_MCTS_OFFSET, DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN,
        DOIP_ENTITY_STATUS_RESPONSE_MDS_OFFSET,
        DOIP_ENTITY_STATUS_RESPONSE_NCTS_OFFSET,
        DOIP_HEADER_LEN,
    },
    payload::{DoipPayload, EntityStatusResponse, NodeType},
};
use heapless::Vec;

use crate::{DecodeError, Decoder, EncodeError, Encoder, FromBytes, ToBytes};

#[derive(Debug)]
pub struct EntityStatusResponseCodec;

impl<const N: usize> Encoder<EntityStatusResponse, N> for EntityStatusResponseCodec {
    type Error = EncodeError;

    fn encode(
        &mut self,
        item: EntityStatusResponse,
        dst: &mut Vec<u8, N>,
    ) -> Result<(), Self::Error> {
        let EntityStatusResponse {
            node_type,
            max_concurrent_sockets,
            currently_open_sockets,
            max_data_size,
        } = item;

        let node_type_bytes = node_type.to_bytes();
        dst.extend_from_slice(node_type_bytes).map_err(|()| EncodeError::BufferTooSmall)?;

        dst.extend_from_slice(&max_concurrent_sockets).map_err(|()| EncodeError::BufferTooSmall)?;

        dst.extend_from_slice(&currently_open_sockets).map_err(|()| EncodeError::BufferTooSmall)?;

        dst.extend_from_slice(&max_data_size).map_err(|()| EncodeError::BufferTooSmall)?;

        Ok(())
    }
}

impl ToBytes for NodeType {
    fn to_bytes(self) -> &'static [u8] {
        match self {
            NodeType::DoipGateway => &[NodeType::DoipGateway as u8],
            NodeType::DoipNode => &[NodeType::DoipNode as u8],
        }
    }
}

impl<const N: usize> Decoder<N> for EntityStatusResponseCodec {
    type Item = DoipPayload<N>;

    type Error = DecodeError;

    fn decode(&mut self, src: &mut Vec<u8, N>) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < DOIP_HEADER_LEN + DOIP_ENTITY_STATUS_RESPONSE_LEN {
            return Err(DecodeError::TooShort);
        }

        let node_type_bytes =
            &src[DOIP_HEADER_LEN..=DOIP_HEADER_LEN];
        let node_type =
            NodeType::from_bytes(node_type_bytes).ok_or(DecodeError::InvalidNodeType)?;

        let max_concurrent_sockets = src[DOIP_ENTITY_STATUS_RESPONSE_MCTS_OFFSET..=DOIP_ENTITY_STATUS_RESPONSE_MCTS_OFFSET]
            .try_into()
            .expect("If failed, source has been manupulated at runtime.");

        let currently_open_sockets = src[DOIP_ENTITY_STATUS_RESPONSE_NCTS_OFFSET..=DOIP_ENTITY_STATUS_RESPONSE_NCTS_OFFSET]
            .try_into()
            .expect("If failed, source has been manupulated at runtime.");

        let max_data_size = src[DOIP_ENTITY_STATUS_RESPONSE_MDS_OFFSET
            ..DOIP_ENTITY_STATUS_RESPONSE_MDS_OFFSET + DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN]
            .try_into()
            .expect("If failed, source has been manupulated at runtime.");

        let item = EntityStatusResponse {
            node_type,
            max_concurrent_sockets,
            currently_open_sockets,
            max_data_size,
        };

        Ok(Some(DoipPayload::EntityStatusResponse(item)))
    }
}

impl FromBytes for NodeType {
    fn from_bytes(bytes: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        let val = *bytes.first()?;

        match val {
            v if v == NodeType::DoipGateway as u8 => Some(NodeType::DoipGateway),
            v if v == NodeType::DoipNode as u8 => Some(NodeType::DoipNode),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use doip_definitions::{
        header::{DoipHeader, PayloadType, ProtocolVersion},
        payload::{DoipPayload, EntityStatusResponse, NodeType},
        DoipMessage,
    };
    use heapless::Vec;

    use crate::{DecodeError, Decoder, DoipCodec, Encoder, FromBytes, ToBytes};

    const BUFFER: usize = 4095;

    static SUCCESS_ROOT: DoipMessage<BUFFER> = DoipMessage {
        header: DoipHeader {
            protocol_version: ProtocolVersion::Iso13400_2012,
            inverse_protocol_version: 0xfd,
            payload_type: PayloadType::EntityStatusResponse,
            payload_length: 7u32,
        },
        payload: DoipPayload::EntityStatusResponse(EntityStatusResponse {
            node_type: NodeType::DoipGateway,
            max_concurrent_sockets: [0x00],
            currently_open_sockets: [0x00],
            max_data_size: [0x00, 0x00, 0x00, 0x00],
        }),
    };

    #[test]
    fn test_node_type_to_bytes() {
        let bytes = NodeType::DoipGateway.to_bytes();
        assert_eq!(bytes, &[0x00]);
        let bytes = NodeType::DoipNode.to_bytes();
        assert_eq!(bytes, &[0x01]);
    }

    #[test]
    fn test_node_type_from_bytes() {
        for a in u8::MIN..=u8::MAX {
            let bytes = &[a];
            let proto = NodeType::from_bytes(bytes);
            match a {
                0x00 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), NodeType::DoipGateway)
                }
                0x01 => {
                    assert!(proto.is_some());
                    assert_eq!(proto.unwrap(), NodeType::DoipNode)
                }
                _ => {
                    assert!(proto.is_none())
                }
            }
        }
    }

    #[test]
    fn test_encode_entity_status_response_success() {
        let mut encoder = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = encoder.encode(SUCCESS_ROOT.clone(), &mut dst);

        assert!(bytes.is_ok(), "Expected bytes to be ok.");
        assert_eq!(
            *dst,
            [
                0x02, 0xfd, 0x40, 0x02, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00
            ]
        );
    }

    #[test]
    fn test_decode_entity_status_response_success() {
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
    fn test_decode_entity_status_response_invalid_node_type() {
        let mut codec = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = &[
            0x02, 0xfd, 0x40, 0x02, 0x00, 0x00, 0x00, 0x07, 0x42, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00,
        ];
        dst.extend_from_slice(bytes).unwrap();
        let msg = codec.decode(&mut dst);

        assert!(msg.is_err());
    }

    #[test]
    fn test_decode_entity_status_response_too_short() {
        let mut codec = DoipCodec {};
        let mut dst = Vec::<u8, BUFFER>::new();

        let bytes = &[
            0x02, 0xfd, 0x40, 0x02, 0x00, 0x00, 0x00, 0x07, 0x42, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        dst.extend_from_slice(bytes).unwrap();
        let msg = codec.decode(&mut dst);

        assert!(msg.is_err());
    }
}
