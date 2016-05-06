use std;
use ::serialize::{self, Serializable};
use super::{Error, GenericError, ConsensusParams, UInt256, BlockHeader, Transaction};

#[derive(Debug,Default,Clone)]
pub struct Block {
   pub header: BlockHeader,
   pub transactions: Vec<Transaction>,
   pub checked: std::cell::Cell<bool>,
}

impl Block {
   pub fn check(&self, params:&ConsensusParams) -> Result<(), Error> {
      if self.checked.get() { return Ok(()); }

      try!(self.header.check(params));

      /* not implemented yet
      {
         if self.header.hash_merkle_root != self.calc_merkle_root() {
            try!(Err(GenericError::new("merkle root mismatch")))
         }
      } */

      for t in self.transactions.iter() {
         try!(t.check());
      }

      self.checked.set(true);
      Ok(())
   }

   pub fn calc_merkle_root(&self) -> UInt256 {
      UInt256::default()
   }
}

impl std::fmt::Display for Block {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Block(header={}, tx={})", self.header, self.transactions.len())
   }
}

impl Serializable for Block {
   fn get_serialize_size(&self, ser:&serialize::SerializeParam) -> usize {
      self.header.get_serialize_size(ser) +
         self.transactions.get_serialize_size(ser)
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.header.serialize(io, ser));
      r += try!(self.transactions.serialize(io, ser));
      Ok(r)
   }
   fn deserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.header.deserialize(io, ser));
      r += try!(self.transactions.deserialize(io, ser));
      self.checked.set(false);
      Ok(r)
   }
}

