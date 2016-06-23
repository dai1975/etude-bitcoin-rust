use std;
use std::io::Write;
use primitive::{self, UInt256};

pub struct HashWriter {
   hasher: primitive::hasher::Sha256,
}
impl HashWriter {
   pub fn new() -> HashWriter {
      HashWriter {
         hasher: primitive::hasher::Sha256::new(),
      }
   }
   pub fn get_hash(&mut self) -> UInt256 {
      let first = self.hasher.result();
      self.hasher.reset();
      self.hasher.input(first.as_slice());
      self.hasher.result()
   }
   pub fn reset(&mut self) {
      self.hasher.reset()
   }
}

impl Write for HashWriter {
   fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
      self.hasher.input(bytes);
      Ok(bytes.len())
   }
   fn flush(&mut self) -> std::io::Result<()> {
      Ok(())
   }
}

