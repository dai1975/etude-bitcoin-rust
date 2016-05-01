use std;
extern crate time;
use ::serialize::{self, Serializable, LimitedString};
use super::Address;

pub const MAX_SUBVERSION_LENGTH:u64 = 256;

// https://en.bitcoin.it/wiki/Protocol_documentation#version
#[derive(Debug)]
pub struct VersionMessage {
   pub version        : i32,
   pub services       : u64,
   pub timestamp      : i64,
   pub addr_recv      : Address,
   pub addr_from      : Address,
   pub nonce          : u64,
   pub user_agent     : String,
   pub start_height   : i32,
   pub relay          : bool,
}

impl Default for VersionMessage {
   fn default() -> VersionMessage {
      VersionMessage {
         version      : 0,
         services     : 0,
         timestamp    : time::get_time().sec,
         addr_recv    : Address::new(0),
         addr_from    : Address::new(0),
         nonce        : 0,
         user_agent   : String::from("/dai-etude:0.1.0/"),
         start_height : 0,
         relay        : false,
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
      write!(f, "Version(ver={}, blocks={}, us={}, them={}, ua={})", self.version, self.start_height, self.addr_recv, self.addr_from, self.user_agent)
   }
}

impl Serializable for VersionMessage {
   fn get_serialize_size(&self, ser:&serialize::SerializeParam) -> usize {
      self.version.get_serialize_size(ser)
         + self.services.get_serialize_size(ser)
         + self.timestamp.get_serialize_size(ser)
         + self.addr_recv.get_serialize_size(ser)
         + self.addr_from.get_serialize_size(ser)
         + self.nonce.get_serialize_size(ser)
         + LimitedString::GetSerializeSize(&*self.user_agent, MAX_SUBVERSION_LENGTH, ser)
         + self.start_height.get_serialize_size(ser)
         + self.relay.get_serialize_size(ser)
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r = 0usize;
      r += try!(self.version.serialize(io, ser));
      r += try!(self.services.serialize(io, ser));
      r += try!(self.timestamp.serialize(io, ser));
      r += try!(self.addr_recv.serialize(io, ser));
      r += try!(self.addr_from.serialize(io, ser));
      r += try!(self.nonce.serialize(io, ser));
      r += try!(LimitedString::new(&*self.user_agent, MAX_SUBVERSION_LENGTH).serialize(io, ser));
      r += try!(self.start_height.serialize(io, ser));
      r += try!(self.relay.serialize(io, ser));
      Ok(r)
   }
   fn deserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r = 0usize;
      r += try!(self.version.deserialize(io, ser));
      r += try!(self.services.deserialize(io, ser));
      r += try!(self.timestamp.deserialize(io, ser));
      r += try!(self.addr_recv.deserialize(io, ser));
      if self.version < 106 { return Ok(r) }

      r += try!(self.addr_from.deserialize(io, ser));
      r += try!(self.nonce.deserialize(io, ser));
      {
         let mut ls = LimitedString::new("", MAX_SUBVERSION_LENGTH);
         r += try!(ls.deserialize(io, ser));
         self.user_agent = ls.string;
      }
      r += try!(self.start_height.deserialize(io, ser));
      if self.version < 70001 { return Ok(r) }

      r += try!(self.relay.deserialize(io, ser));
      Ok(r)
   }
}
