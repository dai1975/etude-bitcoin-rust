use std;
use super::{Error,GenericError,UInt256};
use ::serialize::{self, Serializable, SerializeParam};
use ::script::{Script,Interpreter};

pub type Amount = i64;

const COIN:Amount = 100000000;
const CENT:Amount = 1000000;
const MAX_MONEY:Amount = 21000000 * COIN;

#[derive(Debug,Default,Clone,Eq,PartialEq,PartialOrd,Ord)]
pub struct OutPoint {
   pub hash: UInt256,
   pub n:    u32,
}
impl OutPoint {
   pub fn set_null(&mut self)    { self.hash.set_null();  self.n = std::u32::MAX }
   pub fn is_null(&self) -> bool { self.hash.is_null() && self.n == std::u32::MAX }
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

impl Transaction {
   pub fn is_coin_base(&self) -> bool {
      self.ins.len() == 1 && self.ins[0].prevout.is_null()
   }

   pub fn check(&self) -> Result<(), Error> {
      if self.ins.is_empty()  { try!(Err(GenericError::new("empty tx inputs"))); }
      if self.outs.is_empty() { try!(Err(GenericError::new("empty tx outputs"))); }
      
      {
         let s = self.get_serialize_size(&SerializeParam::new_net());
         if s > super::MAX_BLOCK_SIZE { try!(Err(GenericError::new("oversize tx"))); }
      }

      {
         let mut amount:Amount = 0;
         for pout in self.outs.iter() {
            if pout.value < 0 { try!(Err(GenericError::new("negative tx out"))); }
            if MAX_MONEY < pout.value { try!(Err(GenericError::new("toolarge tx out"))); }
            amount += pout.value;
            if MAX_MONEY < amount { try!(Err(GenericError::new("toolarge total tx out"))); }
         }
      }
         
      {
         let mut set = std::collections::BTreeSet::<&OutPoint>::new();
         for pin in self.ins.iter() {
            if !set.insert(&pin.prevout) { try!(Err(GenericError::new("duplicated tx in"))); }
         }
      }

      println!("tx: i={}, o={}, cb={}", self.ins.len(), self.outs.len(), self.is_coin_base());
      if self.is_coin_base() {
         let s = self.ins[0].script_sig.len();
         if s < 2 || 100 < s { try!(Err(GenericError::new("bad coinbase length"))); }
      } else {
         for (i,pin) in self.ins.iter().enumerate() {
            println!("  check in[{}]...", i);
            if pin.prevout.is_null() { try!(Err(GenericError::new("txin has null prevout"))); }
            let ip = Interpreter::new();
            try!(ip.eval(&pin.script_sig));
         }
      }

      Ok(())
   }
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
   fn deserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.hash.deserialize(io, ser));
      r += try!(self.n.deserialize(io, ser));
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
   fn deserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.prevout.deserialize(io, ser));
      r += try!(self.script_sig.deserialize(io, ser));
      r += try!(self.sequence.deserialize(io, ser));
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
   fn deserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.value.deserialize(io, ser));
      r += try!(self.script_pubkey.deserialize(io, ser));
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
   fn deserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.version.deserialize(io, ser));
      r += try!(self.ins.deserialize(io, ser));
      r += try!(self.outs.deserialize(io, ser));
      r += try!(self.locktime.deserialize(io, ser));
      Ok(r)
   }
}



