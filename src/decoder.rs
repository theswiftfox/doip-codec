use crate::{doip::message::message::DoipMessage, error::DecodeError};
struct DoipCodec;

impl tokio_util::codec::Decoder for DoipCodec {
    type Item = DoipMessage;
    type Error = DecodeError;

    fn decode(
        &mut self,
        _src: &mut tokio_util::bytes::BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
        todo!()
    }
}
