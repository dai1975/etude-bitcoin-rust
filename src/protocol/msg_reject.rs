use std;
use ::serialize::{self, Serializable, LimitedString};
use super::message_header::COMMAND_SIZE;

pub const MAX_REJECT_MESSAGE_LENGTH:usize = 111;

#[derive(Debug,Default,Clone)]
pub struct RejectMessage {
   pub command : [u8; COMMAND_SIZE],
   pub code    : u8,
   pub reason  : String,
}

impl RejectMessage {
   pub fn get_command_str(&self) -> &str {
      let s =
         match self.command.iter().position(|&x| x == 0) {
            Some(pos) => { &self.command[0..pos] }
            None      => { &self.command[..] }
         };
      std::str::from_utf8(s).unwrap()
   }
}

impl std::fmt::Display for RejectMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Reject(cmd={}, code={}, reason={})",
             self.get_command_str(), self.code, self.reason)
   }
}

impl Serializable for RejectMessage {
   fn get_serialize_size(&self, ser:&serialize::SerializeParam) -> usize {
      LimitedString::GetSerializeSize(self.get_command_str(), COMMAND_SIZE as u64, ser) +
         self.code.get_serialize_size(ser) +
         LimitedString::GetSerializeSize(&self.reason, MAX_REJECT_MESSAGE_LENGTH as u64, ser)
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r = 0usize;
      r += try!(LimitedString::Serialize(self.get_command_str(), COMMAND_SIZE as u64, io, ser));
      r += try!(self.code.serialize(io, ser));
      r += try!(LimitedString::Serialize(&self.reason, MAX_REJECT_MESSAGE_LENGTH as u64, io, ser));
      Ok(r)
   }
   fn unserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r = 0usize;
      {
         let mut s:LimitedString = LimitedString::default();
         r += try!(s.unserialize(io, ser));
         let bytes = s.string.into_bytes();
         if COMMAND_SIZE <= bytes.len() {
            self.command.clone_from_slice(&bytes[..COMMAND_SIZE]);
         } else {
            self.command.clone_from_slice(&[0u8; COMMAND_SIZE]);
            self.command[..bytes.len()].clone_from_slice(&bytes[..]);
         }
      }
      r += try!(self.code.unserialize(io, ser));
      r += try!(LimitedString::Unserialize(&mut self.reason, MAX_REJECT_MESSAGE_LENGTH as u64, io, ser));
      Ok(r)
   }
}
