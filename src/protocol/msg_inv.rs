use std;
extern crate time;
use ::serialize::{self, Serializable};
use super::Inv;

#[derive(Debug,Default)]
pub struct InvMessage {
   pub invs : Vec<Inv>,
}
impl super::Message for InvMessage {
   fn get_command(&self) -> [u8; super::message_header::COMMAND_SIZE] {
      super::message_header::COMMAND_INV
   }
}
impl std::fmt::Display for InvMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self.invs.len() {
         0 => write!(f, "Inv(len={})", self.invs.len()),
         1 => write!(f, "Inv(len={}, 0={})", self.invs.len(), self.invs[0]),
         l => write!(f, "Inv(len={}, 0={}, ...{})", self.invs.len(), self.invs[0], self.invs[l-1])
      }
   }
}

impl Serializable for InvMessage {
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
   use primitive::UInt256;
   let mut h = InvMessage::default();
   {
      h.invs.push(Inv::new_tx(UInt256::default()));
      h.invs.push(Inv::new_block(UInt256::default()));
      h.invs.push(Inv::new_filtered_block(UInt256::default()));
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
      assert!(inv.invtype.is_tx());
      assert_eq!([128u8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,], inv.hash.data);
   }
   {
      let inv = &h.invs[1];
      assert!(inv.invtype.is_block());
      assert_eq!([129u8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,], inv.hash.data);
   }
   {
      let inv = &h.invs[2];
      assert!(inv.invtype.is_filtered_block());
      assert_eq!([130,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,], inv.hash.data);
   }
}

