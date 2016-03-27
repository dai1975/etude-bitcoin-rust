use std;
extern crate time;
use super::Address;
use ::serialize::{self, Serializable};

#[derive(Debug,Default)]
pub struct AddrMessage {
   pub addrs : Vec<Address>,
}
impl std::fmt::Display for AddrMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Addr(len={})", self.addrs.len())
   }
}

impl Serializable for AddrMessage {
   fn get_serialize_size(&self) -> usize {
      self.addrs.get_serialize_size()
   }
   fn serialize(&self, io:&mut std::io::Write, stype:i32) -> serialize::Result {
      self.addrs.serialize(io, stype)
   }
   fn unserialize(&mut self, io:&mut std::io::Read, stype:i32) -> serialize::Result {
      self.addrs.unserialize(io, stype)
   }
}

