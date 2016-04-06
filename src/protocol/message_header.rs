use std;
extern crate time;
extern crate crypto;
use self::crypto::digest::Digest;
use ::serialize::{Serializable};

pub const MESSAGE_START_SIZE:usize  =  4;
pub const COMMAND_SIZE:usize        = 12;
pub const MAX_SUBVERSION_LENGTH:u64 = 256;

pub const START_TESTNET:[u8; MESSAGE_START_SIZE] = [0x0b, 0x11, 0x09, 0x07];

pub const COMMAND_VERSION:[u8; COMMAND_SIZE]     = [0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x00, 0x00, 0x00, 0x00, 0x00];
pub const COMMAND_VERACK:[u8; COMMAND_SIZE]      = [0x76, 0x65, 0x72, 0x61, 0x63, 0x6b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
pub const COMMAND_ADDR:[u8; COMMAND_SIZE]        = [0x61, 0x64, 0x64, 0x72, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
pub const COMMAND_INV:[u8; COMMAND_SIZE]         = [0x69, 0x6e, 0x76, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
pub const COMMAND_GETDATA:[u8; COMMAND_SIZE]     = [0x67, 0x65, 0x74, 0x64, 0x61, 0x74, 0x61, 0x00, 0x00, 0x00, 0x00, 0x00];
pub const COMMAND_MERKLEBLOCK:[u8; COMMAND_SIZE] = [0x6d, 0x65, 0x72, 0x6b, 0x6c, 0x65, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x00];
pub const COMMAND_GETBLOCKS:[u8; COMMAND_SIZE]   = [0x67, 0x65, 0x74, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x73, 0x00, 0x00, 0x00];
pub const COMMAND_GETHEADERS:[u8; COMMAND_SIZE]  = [0x67, 0x65, 0x74, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x73, 0x00, 0x00];
pub const COMMAND_TX:[u8; COMMAND_SIZE]          = [0x74, 0x78, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
pub const COMMAND_HEADERS:[u8; COMMAND_SIZE]     = [0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x73, 0x00, 0x00, 0x00, 0x00, 0x00];
pub const COMMAND_BLOCK:[u8; COMMAND_SIZE]       = [0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
pub const COMMAND_GETADDR:[u8; COMMAND_SIZE]     = [0x67, 0x65, 0x74, 0x61, 0x64, 0x64, 0x72, 0x00, 0x00, 0x00, 0x00, 0x00];
pub const COMMAND_MEMPOOL:[u8; COMMAND_SIZE]     = [0x6d, 0x65, 0x6d, 0x70, 0x6f, 0x6f, 0x6c, 0x00, 0x00, 0x00, 0x00, 0x00];
pub const COMMAND_PING:[u8; COMMAND_SIZE]        = [0x70, 0x69, 0x6e, 0x67, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
pub const COMMAND_PONG:[u8; COMMAND_SIZE]        = [0x70, 0x6f, 0x6e, 0x67, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
pub const COMMAND_ALERT:[u8; COMMAND_SIZE]       = [0x61, 0x6c, 0x65, 0x72, 0x74, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
pub const COMMAND_NOTFOUND:[u8; COMMAND_SIZE]    = [0x6e, 0x6f, 0x74, 0x66, 0x6f, 0x75, 0x6e, 0x64, 0x00, 0x00, 0x00, 0x00];
pub const COMMAND_FILTERLOAD:[u8; COMMAND_SIZE]  = [0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x6c, 0x6f, 0x61, 0x64, 0x00, 0x00];
pub const COMMAND_FILTERADD:[u8; COMMAND_SIZE]   = [0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x61, 0x64, 0x64, 0x00, 0x00, 0x00];
pub const COMMAND_FILTERCLEAR:[u8; COMMAND_SIZE] = [0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x63, 0x6c, 0x65, 0x61, 0x72, 0x00];
pub const COMMAND_REJECT:[u8; COMMAND_SIZE]      = [0x72, 0x65, 0x6a, 0x65, 0x63, 0x74, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
pub const COMMAND_SENDHEADERS:[u8; COMMAND_SIZE] = [0x73, 0x65, 0x6e, 0x64, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x73, 0x00];

#[derive(Debug,Default)]
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
   pub fn new(start_:&[u8;MESSAGE_START_SIZE], command_:&[u8;COMMAND_SIZE], size_:u32, checksum_:u32) -> MessageHeader {
      let mut h = MessageHeader::default();
      h.start.clone_from_slice(start_);
      h.command.clone_from_slice(command_);
      h.size     = size_;
      h.checksum = checksum_;
      h
   }

   #[allow(non_snake_case)]
   pub fn GetSerializableSize() -> usize {
      MESSAGE_START_SIZE + COMMAND_SIZE + 4 + 4
   }

   pub fn set_data(&mut self, command_:&[u8], data:&[u8]) -> &mut MessageHeader {
      self.start.clone_from_slice(&START_TESTNET);
      if COMMAND_SIZE <= command_.len() {
         self.command.clone_from_slice(&command_[..COMMAND_SIZE]);
      } else {
         self.command.clone_from_slice(&[0u8; COMMAND_SIZE]);
         self.command[..command_.len()].clone_from_slice(command_);
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


#[test]
fn test_serialize_header() {
   let h = MessageHeader::new(&[1u8, 2u8, 3u8, 4u8], &COMMAND_VERSION, 123u32, 987u32);
   let buf = &mut vec![0; 0]; //Vec::with_capacity(128usize);
   let ser = ::serialize::SerializeParam::new_net();
   assert_matches!(h.serialize(buf, &ser), Ok(24usize));
   assert_eq!([1, 2, 3, 4, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0, 0, 0, 0, 0, 123, 0, 0, 0, 219, 3, 0, 0],
              buf.as_slice());
}

