use std;
extern crate rand;
use ::serialize::{self, Serializable};
use super::BIP0031_VERSION;

#[derive(Debug,Default,Clone)]
pub struct PingMessage
{
   pub nonce: u64,
}
impl PingMessage {
   pub fn reset_nonce(&mut self) {
      use self::rand::Rng;
      let mut rng = rand::os::OsRng::new().unwrap(); // This rng is cryptographic level, is it too secure?
      self.nonce = rng.next_u64();
   }
}
impl super::Message for PingMessage {
   fn get_command(&self) -> [u8; super::message_header::COMMAND_SIZE] {
      super::message_header::COMMAND_PING
   }
}

impl std::fmt::Display for PingMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Ping(nonce={})", self.nonce)
   }
}

impl Serializable for PingMessage {
   fn get_serialize_size(&self, ser:&serialize::SerializeParam) -> usize {
      if BIP0031_VERSION < ser.version {
         self.nonce.get_serialize_size(ser)
      } else {
         0usize
      }
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      if BIP0031_VERSION < ser.version {
         self.nonce.serialize(io, ser)
      } else {
         Ok(0usize)
      }
   }
   fn deserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      if BIP0031_VERSION < ser.version {
         self.nonce.deserialize(io, ser)
      } else {
         Ok(0usize)
      }
   }
}
