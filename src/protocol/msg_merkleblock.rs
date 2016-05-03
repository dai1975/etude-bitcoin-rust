use std;
use ::serialize::{self, Serializable};
use ::primitive::MerkleBlock;

#[derive(Debug,Default)]
pub struct MerkleBlockMessage {
   pub block : MerkleBlock,
}
impl super::Message for MerkleBlockMessage {
   fn get_command(&self) -> [u8; super::message_header::COMMAND_SIZE] {
      super::message_header::COMMAND_MERKLEBLOCK
   }
}
impl std::fmt::Display for MerkleBlockMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "MerkleBlock(block={})", self.block)
   }
}

impl Serializable for MerkleBlockMessage {
   fn get_serialize_size(&self, ser:&serialize::SerializeParam) -> usize {
      self.block.get_serialize_size(ser)
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      self.block.serialize(io, ser)
   }
   fn deserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      self.block.deserialize(io, ser)
   }
}

