use std;
extern crate time;
use ::serialize::{self, Serializable, LimitedString};
use super::Address;
use super::message_header::MAX_SUBVERSION_LENGTH;

#[derive(Debug)]
pub struct VersionMessage {
   pub version   : i32,
   pub services  : u64,
   pub time      : i64,
   pub addr_me   : Address,
   pub addr_you  : Address,
   pub nonce     : u64,
   pub subversion: String,
}

impl Default for VersionMessage {
   fn default() -> VersionMessage {
      VersionMessage {
         version : 0,
         services : 0,
         time : time::get_time().sec,
         addr_me : Address::new(0),
         addr_you : Address::new(0),
         nonce : 0,
         subversion : String::from("/dai-etude:0.1.0/"),
      }
   }
}
impl super::Message for VersionMessage {
   fn get_command(&self) -> [u8; super::message_header::COMMAND_SIZE] {
      super::message_header::COMMAND_VERSION
   }
}

impl std::fmt::Display for VersionMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Version(ver={}, services={}, time={}, me={}, you={}, nonce={}, subv={})", self.version, self.services, self.time, self.addr_me, self.addr_you, self.nonce, self.subversion)
   }
}

impl Serializable for VersionMessage {
   fn get_serialize_size(&self, ser:&serialize::SerializeParam) -> usize {
      self.version.get_serialize_size(ser)
         + self.services.get_serialize_size(ser)
         + self.time.get_serialize_size(ser)
         + self.addr_me.get_serialize_size(ser)
         + self.addr_you.get_serialize_size(ser)
         + self.nonce.get_serialize_size(ser)
         + LimitedString::GetSerializeSize(&*self.subversion, MAX_SUBVERSION_LENGTH, ser)
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r = 0usize;
      r += try!(self.version.serialize(io, ser));
      r += try!(self.services.serialize(io, ser));
      r += try!(self.time.serialize(io, ser));
      r += try!(self.addr_me.serialize(io, ser));
      r += try!(self.addr_you.serialize(io, ser));
      r += try!(self.nonce.serialize(io, ser));
      r += try!(LimitedString::new(&*self.subversion, MAX_SUBVERSION_LENGTH).serialize(io, ser));
      Ok(r)
   }
   fn deserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r = 0usize;
      r += try!(self.version.deserialize(io, ser));
      r += try!(self.services.deserialize(io, ser));
      r += try!(self.time.deserialize(io, ser));
      r += try!(self.addr_me.deserialize(io, ser));
      r += try!(self.addr_you.deserialize(io, ser));
      r += try!(self.nonce.deserialize(io, ser));
      {
         let mut ls = LimitedString::new("", MAX_SUBVERSION_LENGTH);
         r += try!(ls.deserialize(io, ser));
         self.subversion = ls.string;
      }
      Ok(r)
   }
}
