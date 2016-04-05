use std;
use ::serialize::{self, Serializable, UInt256};
use ::script::{Script};

pub type Amount = i64;

#[derive(Debug,Default,Clone)]
pub struct OutPoint {
   pub hash: UInt256,
   pub n:    i32,
}

#[derive(Debug,Default,Clone)]
pub struct TxIn {
   prevout:    OutPoint,
   script_sig: Script,
   sequence:   u32,
}

#[derive(Debug,Default,Clone)]
pub struct TxOut {
   value:         Amount,
   script_pubkey: Script,
}

#[derive(Debug,Default,Clone)]
pub struct Transaction {
   version:  i32,
   ins:      Vec<TxIn>,
   outs:     Vec<TxOut>,
   locktime: u32,
}

impl std::fmt::Display for OutPoint {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "OutPoint(hash={}, n={})", self.hash, self.n)
   }
}
impl std::fmt::Display for TxIn {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "TxIn(prevout={}, sig={}, seq={})", self.prevout, self.script_sig, self.sequence)
   }
}
impl std::fmt::Display for TxOut {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "TxOut(val={}, pubkey={})", self.value, self.script_pubkey)
   }
}
impl std::fmt::Display for Transaction {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Tx(ver={}, ins={:?}, outs={:?}, locktime={})", self.version, self.ins, self.outs, self.locktime)
   }
}

impl Serializable for OutPoint {
   fn get_serialize_size(&self, ser:&serialize::SerializeParam) -> usize {
      self.hash.get_serialize_size(ser) +
         self.n.get_serialize_size(ser)
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.hash.serialize(io, ser));
      r += try!(self.n.serialize(io, ser));
      Ok(r)
   }
   fn unserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.hash.unserialize(io, ser));
      r += try!(self.n.unserialize(io, ser));
      Ok(r)
   }
}

impl Serializable for TxIn {
   fn get_serialize_size(&self, ser:&serialize::SerializeParam) -> usize {
      self.prevout.get_serialize_size(ser) +
         self.script_sig.get_serialize_size(ser) +
         self.sequence.get_serialize_size(ser)
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.prevout.serialize(io, ser));
      r += try!(self.script_sig.serialize(io, ser));
      r += try!(self.sequence.serialize(io, ser));
      Ok(r)
   }
   fn unserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.prevout.unserialize(io, ser));
      r += try!(self.script_sig.unserialize(io, ser));
      r += try!(self.sequence.unserialize(io, ser));
      Ok(r)
   }
}

impl Serializable for TxOut {
   fn get_serialize_size(&self, ser:&serialize::SerializeParam) -> usize {
      self.value.get_serialize_size(ser) +
         self.script_pubkey.get_serialize_size(ser)
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.value.serialize(io, ser));
      r += try!(self.script_pubkey.serialize(io, ser));
      Ok(r)
   }
   fn unserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.value.unserialize(io, ser));
      r += try!(self.script_pubkey.unserialize(io, ser));
      Ok(r)
   }
}

impl Serializable for Transaction {
   fn get_serialize_size(&self, ser:&serialize::SerializeParam) -> usize {
      self.version.get_serialize_size(ser) +
         self.ins.get_serialize_size(ser) +
         self.outs.get_serialize_size(ser) +
         self.locktime.get_serialize_size(ser)
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.version.serialize(io, ser));
      r += try!(self.ins.serialize(io, ser));
      r += try!(self.outs.serialize(io, ser));
      r += try!(self.locktime.serialize(io, ser));
      Ok(r)
   }
   fn unserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.version.unserialize(io, ser));
      r += try!(self.ins.unserialize(io, ser));
      r += try!(self.outs.unserialize(io, ser));
      r += try!(self.locktime.unserialize(io, ser));
      Ok(r)
   }
}



