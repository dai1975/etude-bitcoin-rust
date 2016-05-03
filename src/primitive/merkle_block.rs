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
   fn get_serialize_size(&self, ser:&serialize::SerializeParam) -> usize {
      self.header.get_serialize_size(ser) + self.txn.get_serialize_size(ser)
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.header.serialize(io, ser));
      r += try!(self.txn.serialize(io, ser));
      Ok(r)
   }
   fn deserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.header.deserialize(io, ser));
      r += try!(self.txn.deserialize(io, ser));
      Ok(r)
   }
}
