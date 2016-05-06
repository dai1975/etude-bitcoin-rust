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
      if s.len() != 64 { try!(Err(ParseUInt256Error::new(&format!("string is too short: {}", s)))); }
      let mut r = UInt256::default();
      for (i,v) in r.data.iter_mut().enumerate() {
         let j = 31 - i;
         let hex = &s[(j*2)..(j*2+2)];
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
      for i in 0..32 {
         try!( f.write_fmt(format_args!("{:02x}", self.data[31-i])));
      }
      Ok(())
   }
}

#[test]
fn test_str() {
   let s = "00000000000008a3a41b85b8b29ad444def299fee21793cd8b9e567eab02cd81";
   let uint256 = UInt256::from_str(s).unwrap();

   let expect:[u8;32] = [
      0x81, 0xcd, 0x02, 0xab, 0x7e, 0x56, 0x9e, 0x8b, 0xcd, 0x93, 0x17, 0xe2, 0xfe, 0x99, 0xf2, 0xde,
      0x44, 0xd4, 0x9a, 0xb2, 0xb8, 0x85, 0x1b, 0xa4, 0xa3, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
   ];
   assert_eq!(expect, uint256.data);

   let t = format!("{}", uint256);
   assert_eq!(s, t);
}

