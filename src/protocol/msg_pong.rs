use std;
use ::serialize::{self, Serializable};
use super::PingMessage;
use super::BIP0031_VERSION;

#[derive(Debug,Default,Clone)]
pub struct PongMessage
{
   pub nonce: u64,
}
impl PongMessage {
   pub fn new(ping:&PingMessage) -> PongMessage {
      PongMessage{ nonce: ping.nonce }
   }
}

impl super::Message for PongMessage {
   fn get_command(&self) -> [u8; super::message_header::COMMAND_SIZE] {
      super::message_header::COMMAND_PONG
   }
}
impl std::fmt::Display for PongMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Pong(nonce={})", self.nonce)
   }
}

impl Serializable for PongMessage {
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
