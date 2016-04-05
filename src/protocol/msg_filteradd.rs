use std;
use ::serialize::{self, Serializable};

#[derive(Debug,Default,Clone)]
pub struct FilterAddMessage {
   pub data: Vec<u8>,
}
impl std::fmt::Display for FilterAddMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "FilterAdd(data={:?})", self.data)
   }
}

impl Serializable for FilterAddMessage {
   fn get_serialize_size(&self, ser:&serialize::SerializeParam) -> usize {
      self.data.get_serialize_size(ser)
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      self.data.serialize(io,ser)
   }
   fn unserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      self.data.unserialize(io,ser)
   }
}

