use std;
extern crate time;
use primitive::UInt256;
use ::serialize::{self, Serializable};
use super::{Inv, InvType};

#[derive(Debug,Default)]
pub struct GetDataMessage {
   pub invs : Vec<Inv>,
}

impl GetDataMessage {
   pub fn new(invtype:InvType, hash: UInt256) -> Self {
      GetDataMessage {
         invs: vec![ Inv::new(invtype, hash) ]
      }
   }
   pub fn new_tx(hash: UInt256)           -> Self { Self::new(InvType::Tx, hash) }
   pub fn new_block(hash: UInt256)        -> Self { Self::new(InvType::Block, hash) }
   pub fn new_filter_block(hash: UInt256) -> Self { Self::new(InvType::FilteredBlock, hash) }
}

impl super::Message for GetDataMessage {
   fn get_command(&self) -> [u8; super::message_header::COMMAND_SIZE] {
      super::message_header::COMMAND_GETDATA
   }
}
impl std::fmt::Display for GetDataMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self.invs.len() {
         0 => write!(f, "GetData(len={})", self.invs.len()),
         1 => write!(f, "GetData(len={}, 0={})", self.invs.len(), self.invs[0]),
         l => write!(f, "GetData(len={}, 0={}, ...{})", self.invs.len(), self.invs[0], self.invs[l-1])
      }
   }
}

impl Serializable for GetDataMessage {
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

