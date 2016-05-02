use std;
use ::serialize::{self, Serializable};
use super::Inv;

#[derive(Debug,Default,Clone)]
pub struct NotFoundMessage {
   pub invs : Vec<Inv>,
}
impl super::Message for NotFoundMessage {
   fn get_command(&self) -> [u8; super::message_header::COMMAND_SIZE] {
      super::message_header::COMMAND_NOTFOUND
   }
}

impl std::fmt::Display for NotFoundMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "NotFound(len={})", self.invs.len())
   }
}

impl Serializable for NotFoundMessage {
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


