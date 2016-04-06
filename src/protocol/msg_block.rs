use std;
use ::serialize::{self, Serializable};
use ::blockchain::{Block};

#[derive(Debug,Default,Clone)]
pub struct BlockMessage {
   pub block: Block,
}
impl super::Message for BlockMessage {
   fn get_command(&self) -> [u8; super::message_header::COMMAND_SIZE] {
      super::message_header::COMMAND_BLOCK
   }
}

impl std::fmt::Display for BlockMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Block({})", self.block)
   }
}

impl Serializable for BlockMessage {
   fn get_serialize_size(&self, ser:&serialize::SerializeParam) -> usize {
      self.block.get_serialize_size(ser)
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      self.block.serialize(io, ser)
   }
   fn unserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      self.block.unserialize(io, ser)
   }
}

