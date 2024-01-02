use educe::Educe;
use tokio_util::{
    bytes::{Buf, BufMut, BytesMut},
    codec::{Decoder, Encoder},
};
use typed_builder::TypedBuilder;

use super::message_type::MessageType;
#[derive(Educe)]
#[educe(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[derive(TypedBuilder)]
pub struct Message {
    pub id: i64,
    pub data: Vec<u8>,
    pub opaque: i64,
    #[builder(default = MessageType::RESPONSE)]
    pub message_type: MessageType,
}
#[derive(Debug)]
pub struct MessageCodec;

impl Decoder for MessageCodec {
    type Item = Message;
    type Error = std::io::Error;
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 24 {
            return Ok(None);
        }
        let id = src.get_i64();
        let len = src.get_i64();
        let opaque = src.get_i64();
        let message_type = src.get_u8();
        if src.len() < len as usize {
            return Ok(None);
        }
        let data = src.split_to(len as usize).to_vec();
        let message_type = MessageType::from(message_type);
        Ok(Some(Message {
            id,
            data,
            opaque,
            message_type,
        }))
    }
}

// impl Encoder<Message> for MessageCodec{
//     type Error = std::io::Error;
//     fn encode(&mut self, item: Message, dst: &mut BytesMut) -> Result<(), Self::Error> {
//         dst.put_i64(item.id);
//         dst.put_i64(item.data.len() as i64);
//         dst.put_i64(item.opaque);
//         dst.put_slice(&item.data);
//         Ok(())
//     }
// }
impl Encoder<Message> for MessageCodec {
    type Error = std::io::Error;
    fn encode(&mut self, item: Message, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put_i64(item.id);
        dst.put_i64(item.data.len() as i64);
        dst.put_i64(item.opaque);
        dst.put_u8(item.message_type as u8);
        dst.put_slice(&item.data);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder() {
        let msg = Message::builder()
            .id(1)
            .opaque(2)
            .data(vec![1, 2, 3, 4, 5])
            .message_type(MessageType::RESPONSE)
            .build();
        let mut buf = BytesMut::new();
        println!("{:?}", msg);
        MessageCodec.encode(msg.clone(), &mut buf).unwrap();
        println!("{:?}", buf.len());
        let msg2 = MessageCodec.decode(&mut buf).unwrap_or_default();
        assert_eq!(Some(msg), msg2);
    }
}
