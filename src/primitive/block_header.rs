use std;
use super::{Error,UInt256,pow,ConsensusParams};
use ::serialize::{self, Serializable};
extern crate crypto;
use self::crypto::digest::Digest;

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

impl BlockHeader {
   pub fn calc_hash(&self) -> UInt256 {
      let serpara = serialize::SerializeParam::new_net();
      let mut mem = &mut [0u8; 80];
      let _ = self.serialize(&mut &mut mem[..], &serpara);

      let mut hasher = crypto::sha2::Sha256::new();
      let out = &mut [0u8; 32];
      hasher.input(mem);
      hasher.result(out);
      hasher.reset();
      hasher.input(out);
      hasher.result(out);

      UInt256::new(out)
   }

   pub fn check(&self, params:&ConsensusParams) -> Result<(), Error> {
      try!(pow::check_proof_of_work(&self.calc_hash(), self.bits, params));

      // TODO: timestamp check
      Ok(())
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
   fn deserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.version.deserialize(io, ser));
      r += try!(self.hash_prev_block.deserialize(io, ser));
      r += try!(self.hash_merkle_root.deserialize(io, ser));
      r += try!(self.time.deserialize(io, ser));
      r += try!(self.bits.deserialize(io, ser));
      r += try!(self.nonce.deserialize(io, ser));
      Ok(r)
   }
}

