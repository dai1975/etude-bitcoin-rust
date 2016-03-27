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
   fn serialize(&self, io:&mut std::io::Write, stype:i32) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.header.serialize(io, stype));
      r += try!(self.txn.serialize(io, stype));
      Ok(r)
   }
   fn unserialize(&mut self, io:&mut std::io::Read, stype:i32) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.header.unserialize(io, stype));
      r += try!(self.txn.unserialize(io, stype));
      Ok(r)
   }
}
