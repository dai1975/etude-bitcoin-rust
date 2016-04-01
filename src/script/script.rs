use std;
use ::serialize::{self, Serializable};

#[derive(Debug,Clone,Default)]
pub struct Script {
   pub base: Vec<u8>,
}

impl std::fmt::Display for Script {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "script({:?})", self.base)
   }
}

impl Serializable for Script {
   fn get_serialize_size(&self, ser:&serialize::SerializeParam) -> usize {
      self.base.get_serialize_size(ser)
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      self.base.serialize(io,ser)
   }
   fn unserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      self.base.unserialize(io,ser)
   }
}

