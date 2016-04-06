use std;
use ::serialize::{self, Serializable};

#[derive(Debug,Clone,Default)]
pub struct AlertMessage {
   pub msg: Vec<u8>,
   pub sig: Vec<u8>,
}
impl super::Message for AlertMessage {
   fn get_command(&self) -> [u8; super::message_header::COMMAND_SIZE] {
      super::message_header::COMMAND_ALERT
   }
}

impl std::fmt::Display for AlertMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Alert(msg={:?}, sig={})", self.msg, self.sig.len())
   }
}

impl Serializable for AlertMessage {
   fn get_serialize_size(&self, ser:&serialize::SerializeParam) -> usize {
      self.msg.get_serialize_size(ser) +
         self.sig.get_serialize_size(ser)
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.msg.serialize(io, ser));
      r += try!(self.sig.serialize(io, ser));
      Ok(r)
   }
   fn unserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.msg.unserialize(io, ser));
      r += try!(self.sig.unserialize(io, ser));
      Ok(r)
   }
}
