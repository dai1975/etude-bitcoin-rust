use std;
extern crate time;
use super::Address;
use ::serialize::{self, Serializable};

#[derive(Debug,Default)]
pub struct GetDataMessage {
   pub addrs : Vec<Address>,
}
impl super::Message for GetDataMessage {
   fn get_command(&self) -> [u8; super::message_header::COMMAND_SIZE] {
      super::message_header::COMMAND_GETDATA
   }
}
impl std::fmt::Display for GetDataMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "GetData(len={})", self.addrs.len())
   }
}

impl Serializable for GetDataMessage {
   fn get_serialize_size(&self, ser:&serialize::SerializeParam) -> usize {
      self.addrs.get_serialize_size(ser)
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      self.addrs.serialize(io, ser)
   }
   fn deserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      self.addrs.deserialize(io, ser)
   }
}

