use std;
use ::serialize::{self, Serializable};

#[derive(Debug,Clone,Default)]
pub struct Address {
   pub services: u64,
   pub time : u32,
   pub port : u16, //host order
   pub ip   : [u8;16], //network order
}
impl std::fmt::Display for Address {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "ip={:?}, port={}", self.ip, self.port)
   }
}

impl Address {
   pub fn new(services_:u64) -> Address {
      Address{ services:services_, time:100000000, port:0, ip:[0u8;16] }
   }
   pub fn set_services(&mut self, services_:u64) -> &mut Address {
      self.services = services_;
      self
   }
   pub fn set_ip(&mut self, addr: &std::net::SocketAddr) -> &mut Address {
      match addr {
         &std::net::SocketAddr::V4(v4) => {
            self.port = v4.port();
            self.ip[..12].clone_from_slice(&[0,0,0,0,0,0,0,0,0,0,0xff,0xff]);
            self.ip[12..16].clone_from_slice(&v4.ip().octets());
         }
         &std::net::SocketAddr::V6(v6) => {
            self.port = v6.port();
            self.ip = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]; //TODO
         }
      }
      self
   }
}
impl Serializable for Address {
   fn get_serialize_size(&self, _ser:&serialize::SerializeParam) -> usize {
      8 + 2 + 16
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r = 0usize;
      r += try!(self.services.serialize(io, ser));
      r += try!(self.ip.serialize(io, ser));
      r += try!(self.port.to_le().serialize(io, ser));
      Ok(r)
   }
   fn unserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r = 0usize;
      r += try!(self.services.unserialize(io, ser));
      r += try!(self.ip.unserialize(io, ser));
      {
         let mut p:u16 = 0;
         r += try!(p.unserialize(io, ser));
         self.port = u16::from_le(p);
      }
      Ok(r)
   }
}
