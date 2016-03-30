use std;
extern crate bit_vec;
use ::serialize::{self, Serializable, UInt256};

fn reverse_u8(x:u8) -> u8 {
   let x:u8 = ((x & 0x55) << 1) | ((x & 0xAA) >> 1);
   let x:u8 = ((x & 0x33) << 2) | ((x & 0xCC) >> 2);
   let x:u8 = (x << 4) | (x >> 4);
   x
}

#[derive(Debug,Default,Clone)]
pub struct PartialMerkleTree {
   pub n_transactions: u32,
   pub bits: bit_vec::BitVec,
   pub hashes: Vec<UInt256>,
}

impl std::fmt::Display for PartialMerkleTree {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "PartialMerkleTree(n={}, bits={:?}, hash={:?})", self.n_transactions, self.bits, self.hashes)
   }
}

impl Serializable for PartialMerkleTree {
   fn get_serialize_size(&self, _stype:i32) -> usize {
      4 + (self.bits.len()+7)/8 + 32*self.hashes.len()
   }
   fn serialize(&self, io:&mut std::io::Write, stype:i32) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.n_transactions.serialize(io, stype));
      {
         let mut bytes = self.bits.to_bytes();
         for byte in &mut bytes {
            *byte = reverse_u8(*byte);
         }
         r += try!(bytes.serialize(io, stype));
      }         
      r += try!(self.hashes.serialize(io, stype));
      Ok(r)
   }
   fn unserialize(&mut self, io:&mut std::io::Read, stype:i32) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.n_transactions.unserialize(io, stype));
      {
         let mut bytes:Vec<u8> = Vec::new();
         r += try!(bytes.unserialize(io, stype));

         for byte in &mut bytes {
            *byte = reverse_u8(*byte);
         }
         self.bits = bit_vec::BitVec::from_bytes(bytes.as_slice());
      }         
      r += try!(self.hashes.unserialize(io, stype));
      Ok(r)
   }
}

