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
   fn get_serialize_size(&self) -> usize {
      4 + 32 + 32 + 4 + 4 + 4
   }
   fn serialize(&self, io:&mut std::io::Write) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.version.serialize(io));
      r += try!(self.hash_prev_block.serialize(io));
      r += try!(self.hash_merkle_root.serialize(io));
      r += try!(self.time.serialize(io));
      r += try!(self.bits.serialize(io));
      r += try!(self.nonce.serialize(io));
      Ok(r)
   }
   fn unserialize(&mut self, io:&mut std::io::Read) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.version.unserialize(io));
      r += try!(self.hash_prev_block.unserialize(io));
      r += try!(self.hash_merkle_root.unserialize(io));
      r += try!(self.time.unserialize(io));
      r += try!(self.bits.unserialize(io));
      r += try!(self.nonce.unserialize(io));
      Ok(r)
   }
}

