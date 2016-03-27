use std;
use super::{BlockHeader, PartialMerkleTree};
use ::serialize::{self,Serializable};

#[derive(Debug,Default,Clone)]
pub struct MerkleBlock {
   header: BlockHeader,
   txn:    PartialMerkleTree,
}

impl std::fmt::Display for MerkleBlock {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "MerkleBlock(header={}, txn={})", self.header, self.txn)
   }
}

impl Serializable for MerkleBlock {
   fn get_serialize_size(&self) -> usize {
      self.header.get_serialize_size() + self.txn.get_serialize_size()
   }
   fn serialize(&self, io:&mut std::io::Write) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.header.serialize(io));
      r += try!(self.txn.serialize(io));
      Ok(r)
   }
   fn unserialize(&mut self, io:&mut std::io::Read) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.header.unserialize(io));
      r += try!(self.txn.unserialize(io));
      Ok(r)
   }
}
