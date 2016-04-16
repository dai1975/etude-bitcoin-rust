use std;
use ::serialize::{self, Serializable, UInt256};
use ::blockchain::{self};

#[derive(Debug,Default)]
pub struct GetBlocksMessage {
   pub locator   : blockchain::BlockLocator,
   pub hash_stop : UInt256,
}
impl super::Message for GetBlocksMessage {
   fn get_command(&self) -> [u8; super::message_header::COMMAND_SIZE] {
      super::message_header::COMMAND_GETBLOCKS
   }
}
impl std::fmt::Display for GetBlocksMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "GetBlocks(locator={}, stop={})", self.locator, self.hash_stop)
   }
}

impl Serializable for GetBlocksMessage {
   fn get_serialize_size(&self, ser:&serialize::SerializeParam) -> usize {
      self.locator.get_serialize_size(ser) +
         self.hash_stop.get_serialize_size(ser)
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.locator.serialize(io, ser));
      r += try!(self.hash_stop.serialize(io, ser));
      Ok(r)
   }
   fn deserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.locator.deserialize(io, ser));
      r += try!(self.hash_stop.deserialize(io, ser));
      Ok(r)
   }
}

