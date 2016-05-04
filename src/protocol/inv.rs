use std;
extern crate time;
use primitive::{UInt256};
use ::serialize::{self, SerializeError, Serializable};

#[derive(Debug,Clone,PartialEq)]
pub enum InvType {
   Unknown       = 0,
   Tx            = 1,
   Block         = 2,
   FilteredBlock = 3,
}

impl Default for InvType {
   fn default() -> Self { InvType::Unknown }
}

impl std::fmt::Display for InvType {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match *self {
         InvType::Tx            => write!(f, "tx"),
         InvType::Block         => write!(f, "block"),
         InvType::FilteredBlock => write!(f, "filtered"),
         _ => write!(f, "{}", *self),
      }
   }
}
impl InvType {
   pub fn is_tx(&self)             -> bool { *self == InvType::Tx }
   pub fn is_block(&self)          -> bool { *self == InvType::Block }
   pub fn is_filtered_block(&self) -> bool { *self == InvType::FilteredBlock }
}

impl Serializable for InvType {
   fn get_serialize_size(&self, _ser:&serialize::SerializeParam) -> usize {
      4
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      let tmp:u32 = match *self {
         InvType::Tx => 1,
         InvType::Block => 2,
         InvType::FilteredBlock => 3,
         _ => return SerializeError::result::<usize>("malformed inv type".to_string())
      };
      tmp.serialize(io, ser)
   }
   fn deserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      let mut tmp:u32 = 0;
      r += try!(tmp.deserialize(io, ser));
      match tmp {
         1 => *self = InvType::Tx,
         2 => *self = InvType::Block,
         3 => *self = InvType::FilteredBlock,
         _ => return SerializeError::result::<usize>("malformed inv type".to_string())
      }
      Ok(r)
   }
}

#[derive(Debug,Clone,Default)]
pub struct Inv {
   pub invtype: InvType,
   pub hash: UInt256,
}
impl std::fmt::Display for Inv {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "{}:{}", self.invtype, self.hash)
   }
}
impl Inv {
   #[allow(dead_code)]
   pub fn new(invtype_:InvType, hash_: UInt256) -> Self {
      Inv {
         invtype: invtype_,
         hash:    hash_,
      }
   }
   pub fn new_tx(hash: UInt256)             -> Self { Self::new(InvType::Tx, hash) }
   pub fn new_block(hash: UInt256)          -> Self { Self::new(InvType::Block, hash) }
   pub fn new_filtered_block(hash: UInt256) -> Self { Self::new(InvType::FilteredBlock, hash) }
}
impl Serializable for Inv {
   fn get_serialize_size(&self, _ser:&serialize::SerializeParam) -> usize {
      4 + 32
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r = 0usize;
      r += try!(self.invtype.serialize(io, ser));
      r += try!(self.hash.serialize(io, ser));
      Ok(r)
   }
   fn deserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r = 0usize;
      r += try!(self.invtype.deserialize(io, ser));
      r += try!(self.hash.deserialize(io, ser));
      Ok(r)
   }
}

