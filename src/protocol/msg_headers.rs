use std;
use ::serialize::{self, Serializable, CompactSize};
use ::blockchain::{BlockHeader};

#[derive(Debug,Default,Clone)]
pub struct HeadersMessageElement {
   pub header: BlockHeader,
}
impl super::Message for HeadersMessage {
   fn get_command(&self) -> [u8; super::message_header::COMMAND_SIZE] {
      super::message_header::COMMAND_HEADERS
   }
}


#[derive(Debug,Default,Clone)]
pub struct HeadersMessage {
   pub headers: Vec< HeadersMessageElement >,
}


impl std::fmt::Display for HeadersMessageElement {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      self.header.fmt(f)
   }
}
impl std::fmt::Display for HeadersMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Headers(len={})", self.headers.len())
   }
}

impl Serializable for HeadersMessageElement {
   fn get_serialize_size(&self, ser:&serialize::SerializeParam) -> usize {
      self.header.get_serialize_size(ser) +
         CompactSize::GetSerializeSize(0u64, ser)
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.header.serialize(io, ser));
      r += try!(CompactSize::Serialize(0u64, io, ser));
      Ok(r)
   }
   fn unserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.header.unserialize(io, ser));
      {
         let mut txlen:u64 = 0;
         r += try!(CompactSize::Unserialize(&mut txlen, io, ser));
      }
      Ok(r)
   }
}

impl Serializable for HeadersMessage {
   fn get_serialize_size(&self, ser:&serialize::SerializeParam) -> usize {
      self.headers.get_serialize_size(ser)
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      self.headers.serialize(io, ser)
   }
   fn unserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      self.headers.unserialize(io, ser)
   }
}

