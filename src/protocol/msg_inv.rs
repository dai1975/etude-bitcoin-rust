use std;
extern crate time;
use ::serialize::{self, SerializeError, Serializable, UInt256};

#[derive(Debug,Clone,PartialEq)]
pub enum MessageBlockType {
   Tx            = 1,
   Block         = 2,
   FilteredBlock = 3,
}
impl Default for MessageBlockType {
   fn default() -> MessageBlockType {
      MessageBlockType::Tx
   }
}
impl MessageBlockType {
   pub fn is_tx(&self) -> bool { *self == MessageBlockType::Tx }
   pub fn is_block(&self) -> bool { *self == MessageBlockType::Block }
   pub fn is_filtered_block(&self) -> bool { *self == MessageBlockType::FilteredBlock }
}

impl Serializable for MessageBlockType {
   fn get_serialize_size(&self, _ser:&serialize::SerializeParam) -> usize {
      4
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      let tmp:u32 = match *self {
         MessageBlockType::Tx => 1,
         MessageBlockType::Block => 2,
         MessageBlockType::FilteredBlock => 3,
      };
      tmp.serialize(io, ser)
   }
   fn deserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      let mut tmp:u32 = 0;
      r += try!(tmp.deserialize(io, ser));
      match tmp {
         1 => *self = MessageBlockType::Tx,
         2 => *self = MessageBlockType::Block,
         3 => *self = MessageBlockType::FilteredBlock,
         _ => return Err(serialize::Error::Serialize(SerializeError::new("unexpected block type")))
      }
      Ok(r)
   }
}

#[derive(Debug,Clone,Default)]
pub struct Inv {
   pub blocktype: MessageBlockType,
   pub hash:      UInt256,
}
impl Inv {
   #[allow(dead_code)]
   pub fn new(blocktype_: MessageBlockType, hash_: UInt256) -> Inv {
      Inv {
         blocktype: blocktype_,
         hash: hash_,
      }
   }
}
impl Serializable for Inv {
   fn get_serialize_size(&self, _ser:&serialize::SerializeParam) -> usize {
      4 + 32
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r = 0usize;
      r += try!(self.blocktype.serialize(io, ser));
      r += try!(self.hash.serialize(io, ser));
      Ok(r)
   }
   fn deserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r = 0usize;
      r += try!(self.blocktype.deserialize(io, ser));
      r += try!(self.hash.deserialize(io, ser));
      Ok(r)
   }
}

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
      write!(f, "Inv(len={})", self.invs.len())
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

