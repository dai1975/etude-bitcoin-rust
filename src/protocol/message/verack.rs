use std;
use ::serialize::{self, Serializable};

#[derive(Debug,Default)]
pub struct VerAckMessage;

impl Serializable for VerAckMessage {
   fn get_serialize_size(&self) -> usize {
      0usize
   }
   fn serialize(&self, _io:&mut std::io::Write) -> serialize::Result {
      Ok(0usize)
   }
   fn unserialize(&mut self, _io:&mut std::io::Read) -> serialize::Result {
      Ok(0usize)
   }
}
