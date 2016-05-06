use std;
use ::serialize::{self, Serializable};

#[derive(Debug,Clone,Default)]
pub struct Script {
   pub bytecode: Vec<u8>,
}

impl Script {
   pub fn len(&self) -> usize { self.bytecode.len() }
}

impl std::fmt::Display for Script {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "script({:?})", self.bytecode)
   }
}

impl Serializable for Script {
   fn get_serialize_size(&self, ser:&serialize::SerializeParam) -> usize {
      self.bytecode.get_serialize_size(ser)
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      self.bytecode.serialize(io,ser)
   }
   fn deserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      self.bytecode.deserialize(io,ser)
   }
}

