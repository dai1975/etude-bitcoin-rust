use std;
use primitive::Error;

#[derive(Debug,PartialEq)]
pub struct ParseUInt256Error {
   msg: String
}

impl ParseUInt256Error {
   pub fn new(s:&str) -> Self {
      ParseUInt256Error { msg:s.to_string() }
   }
}

impl std::error::Error for ParseUInt256Error {
   fn description(&self) -> &str {
      &*self.msg
   }
}
impl std::fmt::Display for ParseUInt256Error {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
      write!(f, "{}", self.msg)
   }
}


#[derive(Debug,Default,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct UInt256 {
   pub data: [u8;32],
}
impl std::hash::Hash for UInt256 {
   fn hash<H:std::hash::Hasher>(&self, state:&mut H) {
      state.write(&self.data[..]);
   }
}

const UINT256_NULL:UInt256 = UInt256 { data: [0u8;32] };

impl UInt256 {
   pub fn new(d: &[u8;32]) -> UInt256 {
      let mut v = UInt256 { data: [0;32] };
      v.data.clone_from_slice(d);
      v
   }
   pub fn from_str(s:&str) -> Result<UInt256, Error> {
      if s.len() != 64 { try!(Err(ParseUInt256Error::new("string is too short"))); }
      let mut r = UInt256::default();
      for (i,v) in r.data.iter_mut().enumerate() {
         let hex = &s[(i*2)..(i*2+2)];
         *v = try!(u8::from_str_radix(hex,16));
      };
      Ok(r)
   }
   pub fn as_slice(&self) -> &[u8] {
      &self.data[..]
   }

   pub fn set_null(&mut self)    { self.data.clone_from_slice(&UINT256_NULL.data) }
   pub fn is_null(&self) -> bool { self.data == UINT256_NULL.data }
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
