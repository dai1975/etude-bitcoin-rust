use std;
use ::serialize::{self, Serializable, UInt256};

#[derive(Debug,Default,Clone)]
pub struct BlockLocator {
   pub haves: Vec<UInt256>,
}

impl std::fmt::Display for BlockLocator {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "BlockLocator(len={})", self.haves.len())
   }
}

impl Serializable for BlockLocator {
   fn get_serialize_size(&self, ser:&serialize::SerializeParam) -> usize {
      let mut r:usize = 4 + 32 + 32 + 4 + 4 + 4;
      if ser.sertype & serialize::SER_GETHASH != 0 {
         r += ser.version.get_serialize_size(ser)
      }
      r
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      if ser.sertype & serialize::SER_GETHASH != 0 {
         r += try!(ser.version.serialize(io, ser));
      }
      r += try!(self.haves.serialize(io, ser));
      Ok(r)
   }
   fn deserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      if ser.sertype & serialize::SER_GETHASH != 0 {
         let mut version:i32 = 0;
         r += try!(version.deserialize(io, ser));
      }
      r += try!(self.haves.deserialize(io, ser));
      Ok(r)
   }
}

