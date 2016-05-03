use std;

#[derive(Debug,Default,Clone,PartialEq,Eq,Hash)]
pub struct UInt256 {
   pub data: [u8;32],
}
impl UInt256 {
   pub fn new(d: &[u8;32]) -> UInt256 {
      let mut v = UInt256 { data: [0;32] };
      v.data.clone_from_slice(d);
      v
   }
   pub fn as_slice(&self) -> &[u8] {
      &self.data[..]
   }
}
impl std::ops::Index<usize> for UInt256 {
   type Output = u8;
   fn index(&self, i:usize) -> &u8 {
      &self.data[i]
   }
}
impl std::ops::IndexMut<usize> for UInt256 {
   fn index_mut(&mut self, i:usize) -> &mut u8 {
      &mut self.data[i]
   }
}
impl std::fmt::Display for UInt256 {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      for byte in &self.data {
         try!( f.write_fmt(format_args!("{:02x}", byte)));
      }
      Ok(())
   }
}
