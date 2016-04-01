use std;
use ::serialize::{self, Serializable};
use ::blockchain::{Transaction};

#[derive(Debug,Default)]
pub struct TxMessage {
   pub tx: Transaction,
}
impl std::fmt::Display for TxMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Tx({})", self.tx)
   }
}

impl Serializable for TxMessage {
   fn get_serialize_size(&self, ser:&serialize::SerializeParam) -> usize {
      self.tx.get_serialize_size(ser)
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      self.tx.serialize(io, ser)
   }
   fn unserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      self.tx.unserialize(io, ser)
   }
}

