use std;
use super::{ScriptError, Parser, Parsed};
use super::opcode::*;
use super::flags::*;
use primitive::{Transaction, TxOut, pubkey, UInt256};
use ::serialize::{self, Serializable, SerializeParam, CompactSize};
use std::error::Error;

pub struct Checker<'a> {
   tx:     &'a Transaction,
   in_idx: usize,
}

impl <'a> Checker<'a> {
   pub fn new(tx:&'a Transaction, in_idx:usize) -> Checker {
      Checker { tx:tx, in_idx:in_idx } 
   }
   pub fn verify(&self, target:&[u8], pk:&[u8], sig:&[u8], flags:u32) -> Result<(), ScriptError> {
      try!(check_signature_encoding(sig, flags));
      try!(check_pubkey_encoding(pk, flags));

      let (hash_type, _sig) = match sig.split_last() {
         None              => return Err(ScriptError::new("short sig")),
         Some((last, pre)) => (*last, pre)
      };
      let hash = hash_target(target, self.tx, self.in_idx, hash_type);

      match pubkey::verify(pk, &hash, sig) {
         Ok(_)  => Ok(()),
         Err(e) => Err(ScriptError::new(e.description())),
      }
   }
}

#[allow(dead_code)]
struct ByteBuf<'a>(&'a [u8]);
impl<'a> std::fmt::LowerHex for ByteBuf<'a> {
    fn fmt(&self, fmtr: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for byte in self.0 {
            try!( fmtr.write_fmt(format_args!("{:02x}", byte)));
        }
        Ok(())
    }
}

fn is_compressed_or_uncompressed_pubkey(vch:&[u8]) -> bool {
   let len = vch.len();

   if len < 33 { return false; }
   match vch[0] {
      0x02 => len == 33,
      0x03 => len == 33,
      0x04 => len == 65,
      _ => false
   }
}

fn is_valid_signature_encoding(vch:&[u8]) -> bool {
   let len = vch.len();
   if len < 9 { return false; }
   if len >73 { return false; }

   if vch[0] != 0x30 { return false; }
   if (vch[1]+3) as usize != len { return false; }

   let len_r = vch[3] as usize;
   if 5 + len_r >= len { return false; }

   let len_s = vch[5 + len_r] as usize;
   if len_r + len_s + 7 != len { return false; }

   if vch[2] != 0x02 { return false; }
   if len_r == 0 { return false; }
   if (vch[4] & 0x80) != 0 { return false; }
   if (len_r > 1) && (vch[4] == 0x00) && ((vch[5] & 0x80) != 0) { return false; }
   
   if vch[len_r+4] != 0x02 { return false; }
   if len_s == 0 { return false; }
   if (vch[len_r+6] & 0x80) != 0 { return false; }
   if (len_s > 1) && (vch[len_r+6] == 0x00) && ((vch[len_r+7] & 0x80) != 0) { return false; }

   true
}

fn is_low_der_signature(vch:&[u8]) -> bool {
   if !is_valid_signature_encoding(vch) {
      return false;
   }
   match pubkey::check_low_s(vch) {
      Ok(_)  => true,
      Err(_) => false,
   }
}

fn is_defined_hashtype_signature(vch:&[u8]) -> bool {
   if vch.len() == 0 { return false; }

   let hash_type = vch[vch.len()-1] & !SIGHASH_ANYONECANPAY;
   if hash_type < SIGHASH_ALL || hash_type > SIGHASH_SINGLE {
      return false;
   }
   true
}

pub fn check_pubkey_encoding(vch:&[u8], flags:u32) -> Result<bool, ScriptError> {
   if (flags & SCRIPT_VERIFY_STRICTENC) != 0 {
      if !is_compressed_or_uncompressed_pubkey(vch) {
         return Err(ScriptError::new("pubkey encoding"));
      }
   }
   Ok(true)
}

pub fn check_signature_encoding(vch:&[u8], flags:u32) -> Result<bool, ScriptError> {
   if vch.len() == 0 { return Ok(true); }

   if (flags & (SCRIPT_VERIFY_DERSIG | SCRIPT_VERIFY_LOW_S | SCRIPT_VERIFY_STRICTENC)) != 0 {
      if !is_valid_signature_encoding(vch) {
         return Err(ScriptError::new("signature encoding"));
      }
   }

   if (flags & SCRIPT_VERIFY_LOW_S) != 0 {
      if !is_low_der_signature(vch) {
         return Err(ScriptError::new("not a low der signature"));
      }
   }

   if (flags & SCRIPT_VERIFY_STRICTENC) != 0 {
      if !is_defined_hashtype_signature(vch) {
         return Err(ScriptError::new("not a defined sig hashtype"));
      }
   }

   Ok(true)
}

fn hash_target(target: &[u8], tx:&Transaction, in_idx:usize, hash_type:u8) -> UInt256 {
   let one = UInt256::from_str("0000000000000000000000000000000000000000000000000000000000000001").unwrap();
   if in_idx >= tx.ins.len() {
      return one;
   }

   // Check for invalid use of SIGHASH_SINGLE
   if (hash_type & 0x1f) == SIGHASH_SINGLE {
      if in_idx >= tx.outs.len() { //??
         return one;
      }
   }

   {
      let tx_ser = TransactionSignatureSerializer::new(target, tx, in_idx, hash_type);
      let io = &mut serialize::HashWriter::new();
      let param  = SerializeParam::new_gethash_ver(0);
      Ok(0)
         .and_then(|_| tx_ser.serialize(io, &param))
         .and_then(|_| (hash_type as u32).serialize(io, &param))
         .map(|_| io.get_hash())
         .unwrap_or(one)
   }
}

struct TransactionSignatureSerializer<'a> {
   target: &'a [u8],
   tx: &'a Transaction,
   in_idx: usize, 
   anyone_can_pay: bool,
   hash_single: bool,
   hash_none: bool,
}
impl <'a> TransactionSignatureSerializer<'a> {
   fn new(target: &'a [u8], tx:&'a Transaction, in_idx:usize, hash_type:u8) -> TransactionSignatureSerializer<'a> {
      let anyone_can_pay = (hash_type & SIGHASH_ANYONECANPAY) != 0;
      let hash_single    = (hash_type & 0x1f) == SIGHASH_SINGLE;
      let hash_none      = (hash_type & 0x1f) == SIGHASH_NONE;
      TransactionSignatureSerializer {
         target: target,
         tx: tx,
         in_idx: in_idx,
         anyone_can_pay: anyone_can_pay,
         hash_single: hash_single,
         hash_none: hash_none,
      }
   }

   fn serialize_script_code(&self, io:&mut std::io::Write, param:&SerializeParam) -> serialize::Result {
      let mut r = 0usize;
      let mut parser = Parser::new(self.target);
      let mut num_sep = 0usize;
      while let Some(Parsed(_pos, code, _follow)) = parser.parse_op() {
         if code == OP_CODESEPARATOR { num_sep += 1; }
      }
      r += try!(CompactSize::Serialize((self.target.len() - num_sep) as u64, io, param));

      let mut parser = Parser::new(self.target);
      let mut pos0 = 0;
      while let Some(Parsed(pos, code, _follow)) = parser.parse_op() {
         if code == OP_CODESEPARATOR {
            r += try!(self.target[pos0..pos].serialize(io, param));
            pos0 = pos + 1;
         }
      }
      if pos0 != self.target.len() {
         r += try!(self.target[pos0..].serialize(io, param));
      }
      
      Ok(r)
   }

   fn serialize_input(&self, index:usize, io:&mut std::io::Write, param:&SerializeParam) -> serialize::Result {
      let mut r = 0usize;
      let index = if self.anyone_can_pay { self.in_idx } else { index };
      r += try!(self.tx.ins[index].prevout.serialize(io, param));

      if index != self.in_idx {
         r += try!(CompactSize::Serialize(0u64, io, param)); // empty script = empty vector
      } else {
         r += try!(self.serialize_script_code(io, param));
      }

      if (index != self.in_idx) && (self.hash_single || self.hash_none) {
         r += try!(0u32.serialize(io, param));
      } else {
         r += try!(self.tx.ins[index].sequence.serialize(io, param));
      }

      Ok(r)
   }

   fn serialize_output(&self, index:usize, io:&mut std::io::Write, param:&SerializeParam) -> serialize::Result {
      let mut r = 0usize;

      if self.hash_single && index != self.in_idx {
         let out = TxOut::new();
         r += try!(out.serialize(io, param));
      } else {
         r += try!(self.tx.outs[index].serialize(io, param));
      }
      Ok(r)
   }
}
impl <'a> Serializable for TransactionSignatureSerializer<'a> {
   fn get_serialize_size(&self, param:&SerializeParam) -> usize {
      let mut io = serialize::DummyWriter::new();
      let _ = self.serialize(&mut io, param);
      io.len()
   }
   fn serialize(&self, io: &mut std::io::Write, param:&SerializeParam) -> serialize::Result {
      let mut r = 0usize;
      r += try!(self.tx.version.serialize(io, param));

      let num_in = if self.anyone_can_pay { 1usize } else { self.tx.ins.len() };
      r += try!(CompactSize::Serialize(num_in as u64, io, param));
      for i in 0..num_in {
         r += try!(self.serialize_input(i, io, param));
      }      

      let num_out = match (self.hash_none, self.hash_single) {
         (true, _)     => 0usize,
         (false, true) => self.in_idx + 1,
         _             => self.tx.outs.len()
      };
      r += try!(CompactSize::Serialize(num_out as u64, io, param));
      for i in 0..num_in {
         r += try!(self.serialize_output(i, io, param));
      }      
      
      r += try!(self.tx.locktime.serialize(io, param));
      Ok(r)
   }
   fn deserialize(&mut self, _io: &mut std::io::Read, _param:&SerializeParam) -> serialize::Result {
      Ok(0usize)
   }
}

