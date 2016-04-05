use std;
use ::serialize::{self, Serializable, UInt256};

#[derive(Debug,Default,Clone)]
pub struct BlockHeader {
   pub version: i32,
   pub hash_prev_block: UInt256,
   pub hash_merkle_root: UInt256,
   pub time: u32,
   pub bits: u32,
   pub nonce: u32,
}

impl std::fmt::Display for BlockHeader {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "BlockHeader(version={}, prev={}, merkle={}, time={}, bits={}, nonce={})",
             self.version, self.hash_prev_block, self.hash_merkle_root, self.time, self.bits, self.nonce)
   }
}

impl Serializable for BlockHeader {
   fn get_serialize_size(&self, _ser:&serialize::SerializeParam) -> usize {
      4 + 32 + 32 + 4 + 4 + 4
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.version.serialize(io, ser));
      r += try!(self.hash_prev_block.serialize(io, ser));
      r += try!(self.hash_merkle_root.serialize(io, ser));
      r += try!(self.time.serialize(io, ser));
      r += try!(self.bits.serialize(io, ser));
      r += try!(self.nonce.serialize(io, ser));
      Ok(r)
   }
   fn unserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.version.unserialize(io, ser));
      r += try!(self.hash_prev_block.unserialize(io, ser));
      r += try!(self.hash_merkle_root.unserialize(io, ser));
      r += try!(self.time.unserialize(io, ser));
      r += try!(self.bits.unserialize(io, ser));
      r += try!(self.nonce.unserialize(io, ser));
      Ok(r)
   }
}

