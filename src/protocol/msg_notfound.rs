use std;
use ::serialize::{self, Serializable};
use super::Inv;

#[derive(Debug,Default,Clone)]
pub struct NotFoundMessage {
   pub invs : Vec<Inv>,
}
impl super::Message for NotFoundMessage {
   fn get_command(&self) -> [u8; super::message_header::COMMAND_SIZE] {
      super::message_header::COMMAND_NOTFOUND
   }
}

impl std::fmt::Display for NotFoundMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "NotFound(len={})", self.invs.len())
   }
}

impl Serializable for NotFoundMessage {
   fn get_serialize_size(&self, ser:&serialize::SerializeParam) -> usize {
      self.invs.get_serialize_size(ser)
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      self.invs.serialize(io, ser)
   }
   fn deserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      self.invs.deserialize(io, ser)
   }
}


#[test]
fn test_serialize_inv() {
   let mut h = InvMessage::default();
   {
      h.invs.push(Inv::new(MessageBlockType::Tx, UInt256::default()));
      h.invs.push(Inv::new(MessageBlockType::Block, UInt256::default()));
      h.invs.push(Inv::new(MessageBlockType::FilteredBlock, UInt256::default()));
      h.invs[0].hash[0] = 128u8;
      h.invs[1].hash[0] = 129u8;
      h.invs[2].hash[0] = 130u8;
   }

   let exp = [
      3u8,
      1,0,0,0, 128,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
      2,0,0,0, 129,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
      3,0,0,0, 130,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
   ];

   let buf = &mut vec![0; 0];
   let ser = serialize::SerializeParam::new_net();
   h.serialize(buf, &ser).unwrap();
   assert_eq!(buf.len(), exp.len());

   {
      let a = buf.as_slice();
      for i in 0..exp.len() {
         assert_eq!(exp[i], a[i]);
      }
   }
}

#[test]
fn test_deserialize_inv() {
   let exp = [
      3u8,
      1,0,0,0, 128,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
      2,0,0,0, 129,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
      3,0,0,0, 130,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
   ];

   let mut h = InvMessage::default();
   let ser = serialize::SerializeParam::new_net();
   h.deserialize(&mut &exp[..], &ser).unwrap();
   assert_eq!(3, h.invs.len());
   {
      let inv = &h.invs[0];
      assert_eq!(MessageBlockType::Tx, inv.blocktype);
      assert_eq!([128u8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,], inv.hash.data);
   }
   {
      let inv = &h.invs[1];
      assert_eq!(MessageBlockType::Block, inv.blocktype);
      assert_eq!([129u8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,], inv.hash.data);
   }
   {
      let inv = &h.invs[2];
      assert_eq!(MessageBlockType::FilteredBlock, inv.blocktype);
      assert_eq!([130,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,], inv.hash.data);
   }
}

