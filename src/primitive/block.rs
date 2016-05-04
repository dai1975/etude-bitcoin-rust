use std;
use ::serialize::{self, Serializable};
use super::{BlockHeader, Transaction};

#[derive(Debug,Default,Clone)]
pub struct Block {
   pub header: BlockHeader,
   pub transactions: Vec<Transaction>,
   pub checked: bool,
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
      self.checked = false;
      Ok(r)
   }
}

impl Block {
   pub fn check(&self) -> bool {
      if self.checked { return true; }
      true
   }
}
