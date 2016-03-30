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
         version : ::protocol::PROTOCOL_VERSION,
         services : 0,
         time : time::get_time().sec,
         addr_me : Address::new(0),
         addr_you : Address::new(0),
         nonce : 0,
         subversion : String::from("/dai-etude:0.1.0/"),
      }
   }
}
impl std::fmt::Display for VersionMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Version(ver={}, services={}, time={}, me={}, you={}, nonce={}, subv={})", self.version, self.services, self.time, self.addr_me, self.addr_you, self.nonce, self.subversion)
   }
}

impl Serializable for VersionMessage {
   fn get_serialize_size(&self, stype:i32) -> usize {
      self.version.get_serialize_size(stype)
         + self.services.get_serialize_size(stype)
         + self.time.get_serialize_size(stype)
         + self.addr_me.get_serialize_size(stype)
         + self.addr_you.get_serialize_size(stype)
         + self.nonce.get_serialize_size(stype)
         + LimitedString::GetSerializeSize(&*self.subversion, MAX_SUBVERSION_LENGTH, stype)
   }
   fn serialize(&self, io:&mut std::io::Write, stype:i32) -> serialize::Result {
      let mut r = 0usize;
      r += try!(self.version.serialize(io, stype));
      r += try!(self.services.serialize(io, stype));
      r += try!(self.time.serialize(io, stype));
      r += try!(self.addr_me.serialize(io, stype));
      r += try!(self.addr_you.serialize(io, stype));
      r += try!(self.nonce.serialize(io, stype));
      r += try!(LimitedString::new(&*self.subversion, MAX_SUBVERSION_LENGTH).serialize(io, stype));
      Ok(r)
   }
   fn unserialize(&mut self, io:&mut std::io::Read, stype:i32) -> serialize::Result {
      let mut r = 0usize;
      r += try!(self.version.unserialize(io, stype));
      r += try!(self.services.unserialize(io, stype));
      r += try!(self.time.unserialize(io, stype));
      r += try!(self.addr_me.unserialize(io, stype));
      r += try!(self.addr_you.unserialize(io, stype));
      r += try!(self.nonce.unserialize(io, stype));
      {
         let mut ls = LimitedString::new("", MAX_SUBVERSION_LENGTH);
         r += try!(ls.unserialize(io, stype));
         self.subversion = ls.string;
      }
      Ok(r)
   }
}
