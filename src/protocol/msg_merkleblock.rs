use std;
use ::serialize::{self, Serializable};
use ::blockchain;

#[derive(Debug,Default)]
pub struct MerkleBlockMessage {
   pub block : blockchain::MerkleBlock,
}
impl std::fmt::Display for MerkleBlockMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "MerkleBlock(block={})", self.block)
   }
}

impl Serializable for MerkleBlockMessage {
   fn get_serialize_size(&self, stype:i32) -> usize {
      self.block.get_serialize_size(stype)
   }
   fn serialize(&self, io:&mut std::io::Write, stype:i32) -> serialize::Result {
      self.block.serialize(io, stype)
   }
   fn unserialize(&mut self, io:&mut std::io::Read, stype:i32) -> serialize::Result {
      self.block.unserialize(io, stype)
   }
}

