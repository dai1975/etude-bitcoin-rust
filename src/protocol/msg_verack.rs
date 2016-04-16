use std;
use ::serialize::{self, Serializable};

#[derive(Debug,Default)]
pub struct VerAckMessage;

impl super::Message for VerAckMessage {
   fn get_command(&self) -> [u8; super::message_header::COMMAND_SIZE] {
      super::message_header::COMMAND_VERACK
   }
}

impl std::fmt::Display for VerAckMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "VerAck()")
   }
}

impl Serializable for VerAckMessage {
   fn get_serialize_size(&self, _ser:&serialize::SerializeParam) -> usize {
      0usize
   }
   fn serialize(&self, _io:&mut std::io::Write, _ser:&serialize::SerializeParam) -> serialize::Result {
      Ok(0usize)
   }
   fn deserialize(&mut self, _io:&mut std::io::Read, _ser:&serialize::SerializeParam) -> serialize::Result {
      Ok(0usize)
   }
}
