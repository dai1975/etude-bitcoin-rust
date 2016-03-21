use std;
extern crate time;
use ::serialize::{self, Serializable, LimitedString};
use super::Address;
use super::header::MAX_SUBVERSION_LENGTH;

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

impl VersionMessage {
   pub fn new() -> VersionMessage {
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

impl Serializable for VersionMessage {
   fn get_serialize_size(&self) -> usize {
      self.version.get_serialize_size()
         + self.services.get_serialize_size()
         + self.time.get_serialize_size()
         + self.addr_me.get_serialize_size()
         + self.addr_you.get_serialize_size()
         + self.nonce.get_serialize_size()
         + LimitedString::GetSerializeSize(&*self.subversion, MAX_SUBVERSION_LENGTH)
   }
   fn serialize(&self, io:&mut std::io::Write) -> serialize::Result {
      let mut r = 0usize;
      r += try!(self.version.serialize(io));
      r += try!(self.services.serialize(io));
      r += try!(self.time.serialize(io));
      r += try!(self.addr_me.serialize(io));
      r += try!(self.addr_you.serialize(io));
      r += try!(self.nonce.serialize(io));
      r += try!(LimitedString::new(&*self.subversion, MAX_SUBVERSION_LENGTH).serialize(io));
      Ok(r)
   }
   fn unserialize(&mut self, io:&mut std::io::Read) -> serialize::Result {
      let mut r = 0usize;
      r += try!(self.version.unserialize(io));
      r += try!(self.services.unserialize(io));
      r += try!(self.time.unserialize(io));
      r += try!(self.addr_me.unserialize(io));
      r += try!(self.addr_you.unserialize(io));
      r += try!(self.nonce.unserialize(io));
      {
         let mut ls = LimitedString::new("", MAX_SUBVERSION_LENGTH);
         r += try!(ls.unserialize(io));
         self.subversion = ls.string;
      }
      Ok(r)
   }
}
