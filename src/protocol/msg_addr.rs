use std;
extern crate time;
use super::Address;
use ::serialize::{self, Serializable};

#[derive(Debug,Default)]
pub struct AddrMessage {
   pub addrs : Vec<Address>,
}
impl super::Message for AddrMessage {
   fn get_command(&self) -> [u8; super::message_header::COMMAND_SIZE] {
      super::message_header::COMMAND_ADDR
   }
}

impl std::fmt::Display for AddrMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Addr(len={})", self.addrs.len())
   }
}

impl Serializable for AddrMessage {
   fn get_serialize_size(&self, ser:&serialize::SerializeParam) -> usize {
      self.addrs.get_serialize_size(ser)
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      self.addrs.serialize(io, ser)
   }
   fn unserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      self.addrs.unserialize(io, ser)
   }
}

