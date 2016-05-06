use std;
use super::{Error,UInt256,pow,ConsensusParams};
use ::serialize::{self, Serializable};

#[derive(Debug,Default,Clone)]
pub struct BlockHeader {
   pub version: i32,
   pub hash_prev_block: UInt256,
   pub hash_merkle_root: UInt256,
   pub time: u32,
   pub bits: u32,
   pub nonce: u32,
}

impl std::fmt::Display for BlockHeader {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "BlockHeader(version={}, prev={}, merkle={}, time={}, bits={}, nonce={})",
             self.version, self.hash_prev_block, self.hash_merkle_root, self.time, self.bits, self.nonce)
   }
}

impl Serializable for BlockHeader {
   fn get_serialize_size(&self, _ser:&serialize::SerializeParam) -> usize {
      4 + 32 + 32 + 4 + 4 + 4
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.version.serialize(io, ser));
      r += try!(self.hash_prev_block.serialize(io, ser));
      r += try!(self.hash_merkle_root.serialize(io, ser));
      r += try!(self.time.serialize(io, ser));
      r += try!(self.bits.serialize(io, ser));
      r += try!(self.nonce.serialize(io, ser));
      Ok(r)
   }
   fn deserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.version.deserialize(io, ser));
      r += try!(self.hash_prev_block.deserialize(io, ser));
      r += try!(self.hash_merkle_root.deserialize(io, ser));
      r += try!(self.time.deserialize(io, ser));
      r += try!(self.bits.deserialize(io, ser));
      r += try!(self.nonce.deserialize(io, ser));
      Ok(r)
   }
}

impl BlockHeader {
   pub fn calc_hash(&self) -> UInt256 {
      let o:&Serializable = self as &Serializable;
      o.serialize_hash256d(&serialize::SerializeParam::new_net()).unwrap()
   }
   
   pub fn check(&self, params:&ConsensusParams) -> Result<(), Error> {
      try!(pow::check_proof_of_work(&self.calc_hash(), self.bits, params));

      // TODO: timestamp check
      Ok(())
   }
}

#[test]
fn test_hash() {
   use primitive::UInt256;
   use serialize::SerializeParam;

   //block 125552 https://blockexplorer.com/block/00000000000000001e8d6829a8a21adc5d38d0a473b144b6765798e61f98bd1d
   let h = BlockHeader {
      version: 1,
      hash_prev_block:  UInt256::from_str("00000000000008a3a41b85b8b29ad444def299fee21793cd8b9e567eab02cd81").unwrap(),
      hash_merkle_root: UInt256::from_str("2b12fcf1b09288fcaff797d71e950e71ae42b91e8bdb2304758dfcffc2b620e3").unwrap(),
      time:  0x4dd7f5c7,
      bits:  0x1a44b9f2,
      nonce: 0x9546a142,
   };
   let ser = &SerializeParam::new_net();
   
   {
      assert_eq!(80usize, h.get_serialize_size(ser));
      let mut mem:Vec<u8> = Vec::with_capacity(h.get_serialize_size(ser));
      assert_eq!(80usize, h.serialize(&mut mem, ser).unwrap());
      let expect:[u8;80] = [
         0x01, 0x00, 0x00, 0x00,
         0x81, 0xcd, 0x02, 0xab, 0x7e, 0x56, 0x9e, 0x8b, 0xcd, 0x93, 0x17, 0xe2, 0xfe, 0x99, 0xf2, 0xde,
         0x44, 0xd4, 0x9a, 0xb2, 0xb8, 0x85, 0x1b, 0xa4, 0xa3, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0xe3, 0x20, 0xb6, 0xc2, 0xff, 0xfc, 0x8d, 0x75, 0x04, 0x23, 0xdb, 0x8b, 0x1e, 0xb9, 0x42, 0xae,
         0x71, 0x0e, 0x95, 0x1e, 0xd7, 0x97, 0xf7, 0xaf, 0xfc, 0x88, 0x92, 0xb0, 0xf1, 0xfc, 0x12, 0x2b,
         0xc7, 0xf5, 0xd7, 0x4d,
         0xf2, 0xb9, 0x44, 0x1a,
         0x42, 0xa1, 0x46, 0x95,
      ];
      assert_eq!(&expect[..], mem.as_slice());
   }

   let hash = h.calc_hash();
   let hashstr = format!("{}", hash);
   assert_eq!("00000000000000001e8d6829a8a21adc5d38d0a473b144b6765798e61f98bd1d", hashstr);
}
