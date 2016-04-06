use std;
use ::serialize::{self, Serializable};

#[derive(Debug,Default,Clone)]
pub struct FilterLoadMessage {
   pub data: Vec<u8>,
   pub hash_funcs: u32,
   pub tweak: u32,
   pub flags: u8,
}
impl super::Message for FilterLoadMessage {
   fn get_command(&self) -> [u8; super::message_header::COMMAND_SIZE] {
      super::message_header::COMMAND_FILTERLOAD
   }
}

impl std::fmt::Display for FilterLoadMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "FilterLoad(data={:?},funcs={},tweak={},flags={})", self.data, self.hash_funcs, self.tweak, self.flags)
   }
}

impl Serializable for FilterLoadMessage {
   fn get_serialize_size(&self, ser:&serialize::SerializeParam) -> usize {
      self.data.get_serialize_size(ser) +
         self.hash_funcs.get_serialize_size(ser) +
         self.tweak.get_serialize_size(ser) +
         self.flags.get_serialize_size(ser)
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.data.serialize(io,ser));
      r += try!(self.hash_funcs.serialize(io,ser));
      r += try!(self.tweak.serialize(io,ser));
      r += try!(self.flags.serialize(io,ser));
      Ok(r)
   }
   fn unserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.data.unserialize(io,ser));
      r += try!(self.hash_funcs.unserialize(io,ser));
      r += try!(self.tweak.unserialize(io,ser));
      r += try!(self.flags.unserialize(io,ser));
      Ok(r)
   }
}

