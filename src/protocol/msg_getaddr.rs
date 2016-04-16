use std;
use ::serialize::{self, Serializable};

#[derive(Debug,Default,Clone)]
pub struct GetAddrMessage;

impl super::Message for GetAddrMessage {
   fn get_command(&self) -> [u8; super::message_header::COMMAND_SIZE] {
      super::message_header::COMMAND_GETADDR
   }
}

impl std::fmt::Display for GetAddrMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "GetAddr()")
   }
}

impl Serializable for GetAddrMessage {
   fn get_serialize_size(&self, _ser:&serialize::SerializeParam) -> usize {
      0
   }
   fn serialize(&self, _io:&mut std::io::Write, _ser:&serialize::SerializeParam) -> serialize::Result {
      Ok(0)
   }
   fn deserialize(&mut self, _io:&mut std::io::Read, _ser:&serialize::SerializeParam) -> serialize::Result {
      Ok(0)
   }
}

