use std;
use ::serialize::{self, Serializable};

#[derive(Debug,Default,Clone)]
pub struct MemPoolMessage;

impl std::fmt::Display for MemPoolMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "MemPool()")
   }
}
impl super::Message for MemPoolMessage {
   fn get_command(&self) -> [u8; super::message_header::COMMAND_SIZE] {
      super::message_header::COMMAND_MEMPOOL
   }
}

impl Serializable for MemPoolMessage {
   fn get_serialize_size(&self, _ser:&serialize::SerializeParam) -> usize {
      0usize
   }
   fn serialize(&self, _io:&mut std::io::Write, _ser:&serialize::SerializeParam) -> serialize::Result {
      Ok(0usize)
   }
   fn unserialize(&mut self, _io:&mut std::io::Read, _ser:&serialize::SerializeParam) -> serialize::Result {
      Ok(0usize)
   }
}
