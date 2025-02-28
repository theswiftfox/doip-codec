use doip_definitions::payload::DoipMessage;
use tokio_util::bytes::BytesMut;

use crate::{doip_message::HeaderCodec, error::EncodeError, DoipCodec, Encoder};

impl Encoder<DoipMessage<'static>> for DoipCodec {
    type Error = EncodeError;

    fn encode(
        &mut self,
        item: DoipMessage,
        dst: &mut tokio_util::bytes::BytesMut,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}
