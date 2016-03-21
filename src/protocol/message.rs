use std;
use super::Address;
use super::super::serialize::*;
extern crate time;
extern crate crypto;
use self::crypto::digest::Digest;

pub const MESSAGE_START_SIZE:usize  =  4;
pub const COMMAND_SIZE:usize        = 12;
pub const MAX_SUBVERSION_LENGTH:u64 = 256;
pub const PROTOCOL_VERSION:i32      = 70012;

pub const START_TESTNET:[u8; MESSAGE_START_SIZE] = [0x0b, 0x11, 0x09, 0x07];
pub const COMMAND_VERSION:[u8; COMMAND_SIZE]     = [0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x00, 0x00, 0x00, 0x00, 0x00];

#[derive(Debug)]
pub struct MessageHeader {
   pub start    : [u8; MESSAGE_START_SIZE],
   pub command  : [u8; COMMAND_SIZE],
   pub size     : u32,
   pub checksum : u32,
}

impl std::fmt::Display for MessageHeader {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      let com = String::from_utf8_lossy(&self.command);
      write!(f, "Header(start={:?}, command={}, size={}, sum={})", self.start, com, self.size, self.checksum)
   }
}

impl MessageHeader {
   pub fn new() -> MessageHeader {
      MessageHeader {
         start    : [0u8; MESSAGE_START_SIZE],
         command  : [0u8; COMMAND_SIZE],
         size     : 0,
         checksum : 0,
      }
   }

   #[allow(non_snake_case)]
   pub fn GetSerializableSize() -> usize {
      MESSAGE_START_SIZE + COMMAND_SIZE + 4 + 4
   }

   pub fn set_data(&mut self, command_:&str, data:&[u8]) -> &mut MessageHeader {
      self.start.clone_from_slice(&START_TESTNET);
      if COMMAND_SIZE <= command_.len() {
         self.command.clone_from_slice(&command_.as_bytes()[..COMMAND_SIZE]);
      } else {
         self.command.clone_from_slice(&[0u8; COMMAND_SIZE]);
         self.command[..command_.len()].clone_from_slice(command_.as_bytes());
      }
      self.size = data.len() as u32;

      {
         let mut hasher = crypto::sha2::Sha256::new();
         let mut tmp = [0u8; 32];
         hasher.input(&data);
         hasher.result(&mut tmp);
         hasher.reset();
         hasher.input(&tmp);
         hasher.result(&mut tmp);
         let mut sum:u32 = 0;
         let psum: &mut [u8;4] = unsafe { std::mem::transmute(&mut sum) };
         psum.clone_from_slice(&tmp[0..4]);
         self.checksum = u32::from_le(sum);
      }

      self
   }
}

/// MessageHeader serializer implementation
impl Serializable for MessageHeader {
   ADD_SERIALIZE_METHODS!(start, command, size, checksum);
}


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
         version : PROTOCOL_VERSION,
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


#[test]
fn test_serialize_header() {
   let h = MessageHeader::new(&[0x01, 0x02, 0x03, 0x04], "command", 123u32, 987u32);
   let buf = &mut vec![0; 0]; //Vec::with_capacity(128usize);
   assert_matches!(h.serialize(buf), Ok(()));
   assert_eq!([1, 2, 3, 4, 99, 111, 109, 109, 97, 110, 100, 0, 0, 0, 0, 0, 123, 0, 0, 0, 219, 3, 0, 0],
              buf.as_slice());
}

