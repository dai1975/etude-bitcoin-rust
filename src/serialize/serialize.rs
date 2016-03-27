use std;

#[derive(Debug)]
pub struct SerializeError {
   msg: String
}
impl SerializeError {
   pub fn new(s:&str) -> SerializeError {
      SerializeError { msg:s.to_string() }
   }
}
impl std::error::Error for SerializeError {
   fn description(&self) -> &str {
      &*self.msg
   }
}
impl std::fmt::Display for SerializeError {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
      write!(f, "{}", self.msg)
   }
}

#[derive(Debug)]
pub enum Error {
   Serialize(SerializeError),
   Io(std::io::Error),
   Utf8(std::string::FromUtf8Error),
}
impl From<SerializeError> for Error {
   fn from(err: SerializeError) -> Error {
      Error::Serialize(err)
   }
}
impl From<std::io::Error> for Error {
   fn from(err: std::io::Error) -> Error {
      Error::Io(err)
   }
}
impl From<std::string::FromUtf8Error> for Error {
   fn from(err: std::string::FromUtf8Error) -> Error {
      Error::Utf8(err)
   }
}

pub type Result = std::result::Result<usize, Error>;

pub const SER_NET:i32     = 1 << 0;
pub const SER_DISK:i32    = 1 << 1;
pub const SER_GETHASH:i32 = 1 << 2;

pub trait Serializable {
   fn get_serialize_size(&self) -> usize;
   fn serialize(&self, io:&mut std::io::Write, stype:i32) -> Result;
   fn unserialize(&mut self, io:&mut std::io::Read, stype:i32) -> Result;
}

#[macro_use]
macro_rules! ADD_SERIALIZE_METHODS {
   ( $($x:ident),*) => {
      fn get_serialize_size(&self) -> usize {
         let mut sum = 0usize;
         $(
            sum += self.$x.get_serialize_size();
         )*
         sum
      }

      fn serialize(&self, io:&mut std::io::Write, stype:i32) -> ::serialize::Result {
         let mut r = 0usize;
         $(
            r += try!( self.$x.serialize(io, stype) );
         )*
         Ok(r)
      }
      fn unserialize(&mut self, io:&mut std::io::Read, stype:i32) -> ::serialize::Result {
         let mut r = 0usize;
         $(
            r += try!( self.$x.unserialize(io, stype) );
         )*
         Ok(r)
      }
   }
}

impl Serializable for u8 {
   fn get_serialize_size(&self) -> usize {
      1
   }
   fn serialize(&self, io: &mut std::io::Write, _stype:i32) -> Result {
      let buf: [u8; 1] = [*self];
      try!(io.write_all(&buf));
      Ok(buf.len())
   }
   fn unserialize(&mut self, io: &mut std::io::Read, _stype:i32) -> Result {
      let mut buf: [u8; 1] = [0];
      try!(io.read_exact(&mut buf));
      *self = buf[0];
      Ok(buf.len())
   }
}

impl Serializable for u32 {
   fn get_serialize_size(&self) -> usize {
      4
   }
   fn serialize(&self, io: &mut std::io::Write, _stype:i32) -> Result {
      let tmp = self.to_le();
      let buf: &[u8;4] = unsafe { std::mem::transmute(&tmp) };
      try!(io.write_all(buf));
      Ok(buf.len())
   }
   fn unserialize(&mut self, io: &mut std::io::Read, _stype:i32) -> Result {
      let mut tmp:u32 = 0;
      let buf: &mut [u8;4] = unsafe { std::mem::transmute(&mut tmp) };
      try!(io.read_exact(buf));
      *self = u32::from_le(tmp);
      Ok(buf.len())
   }
}

impl Serializable for i32 {
   fn get_serialize_size(&self) -> usize {
      4
   }
   fn serialize(&self, io: &mut std::io::Write, _stype:i32) -> Result {
      let tmp = self.to_le();
      let buf: &[u8;4] = unsafe { std::mem::transmute(&tmp) };
      try!(io.write_all(buf));
      Ok(buf.len())
   }
   fn unserialize(&mut self, io: &mut std::io::Read, _stype:i32) -> Result {
      let mut tmp:i32 = 0;
      let buf: &mut [u8;4] = unsafe { std::mem::transmute(&mut tmp) };
      try!(io.read_exact(buf));
      *self = i32::from_le(tmp);
      Ok(buf.len())
   }
}

impl Serializable for u16 {
   fn get_serialize_size(&self) -> usize {
      2
   }
   fn serialize(&self, io: &mut std::io::Write, _stype:i32) -> Result {
      let tmp = self.to_le();
      let buf: &[u8;2] = unsafe { std::mem::transmute(&tmp) };
      try!(io.write_all(buf));
      Ok(buf.len())
   }
   fn unserialize(&mut self, io: &mut std::io::Read, _stype:i32) -> Result {
      let mut tmp:u16 = 0;
      let buf: &mut [u8;2] = unsafe { std::mem::transmute(&mut tmp) };
      try!(io.read_exact(buf));
      *self = u16::from_le(tmp);
      Ok(buf.len())
   }
}

impl Serializable for u64 {
   fn get_serialize_size(&self) -> usize {
      8
   }
   fn serialize(&self, io: &mut std::io::Write, _stype:i32) -> Result {
      let tmp = self.to_le();
      let buf: &[u8;8] = unsafe { std::mem::transmute(&tmp) };
      try!(io.write_all(buf));
      Ok(buf.len())
   }
   fn unserialize(&mut self, io: &mut std::io::Read, _stype:i32) -> Result {
      let mut tmp:u64 = 0;
      let buf: &mut [u8;8] = unsafe { std::mem::transmute(&mut tmp) };
      try!(io.read_exact(buf));
      *self = u64::from_le(tmp);
      Ok(buf.len())
   }
}
impl Serializable for i64 {
   fn get_serialize_size(&self) -> usize {
      8
   }
   fn serialize(&self, io: &mut std::io::Write, _stype:i32) -> Result {
      let tmp = self.to_le();
      let buf: &[u8;8] = unsafe { std::mem::transmute(&tmp) };
      try!(io.write_all(buf));
      Ok(buf.len())
   }
   fn unserialize(&mut self, io: &mut std::io::Read, _stype:i32) -> Result {
      let mut tmp:i64 = 0;
      let buf: &mut [u8;8] = unsafe { std::mem::transmute(&mut tmp) };
      try!(io.read_exact(buf));
      *self = i64::from_le(tmp);
      Ok(buf.len())
   }
}

#[derive(Debug,Default,Clone,PartialEq)]
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
impl Serializable for UInt256 {
   fn get_serialize_size(&self) -> usize {
      32
   }
   fn serialize(&self, io: &mut std::io::Write, _stype:i32) -> Result {
      try!(io.write_all(&self.data));
      Ok(32)
   }
   fn unserialize(&mut self, io: &mut std::io::Read, _stype:i32) -> Result {
      try!(io.read_exact(&mut self.data));
      Ok(32)
   }
}

pub struct CompactSize {
   pub value:u64,
}
impl CompactSize {
   fn new(v:u64) -> CompactSize {
      CompactSize { value:v }
   }
   // I beleave that the coding style which explicitely differs static method with instance method is good style.
   #[allow(non_snake_case)]
   fn GetSerializeSize(v:u64) -> usize {
      if v < 253 {
         1
      } else if v <= 0xFFFF {
         3
      } else if v <= 0xFFFFFFFF {
         5
      } else {
         9
      }
   }

   #[allow(non_snake_case)]
   fn Serialize(value:u64, io: &mut std::io::Write, stype:i32) -> Result {
      let mut r = 0usize;
      if value < 253 {
         let v = value as u8;
         r += try!(v.serialize(io, stype));
      } else if value <= 0xFFFF {
         let v = value as u16;
         r += try!(253u8.serialize(io, stype));
         r += try!(v.serialize(io, stype));
      } else if value <= 0xFFFFFFFF {
         let v = value as u32;
         r += try!(254u8.serialize(io, stype));
         r += try!(v.serialize(io, stype));
      } else {
         r += try!(255u8.serialize(io, stype));
         r += try!(value.serialize(io, stype));
      }
      Ok(r)
   }

   #[allow(non_snake_case)]
   fn Unserialize(value:&mut u64, io: &mut std::io::Read, stype:i32) -> Result {
      let mut r = 0usize;
      let mut h:u8 = 0;
      r += try!(h.unserialize(io, stype));
      if h < 253 {
         *value = h as u64;
      } else if h == 253 {
         let mut v:u16 = 0;
         r += try!(v.unserialize(io, stype));
         *value = v as u64;
      } else if h == 254 {
         let mut v:u32 = 0;
         r += try!(v.unserialize(io, stype));
         *value = v as u64;
      } else if h == 255 {
         let mut v:u64 = 0;
         r += try!(v.unserialize(io, stype));
         *value = v;
      }
      Ok(r)
   }
}
impl Serializable for CompactSize {
   fn get_serialize_size(&self) -> usize {
      CompactSize::GetSerializeSize(self.value)
   }
   fn serialize(&self, io: &mut std::io::Write, stype:i32) -> Result {
      CompactSize::Serialize(self.value, io, stype)
   }
   fn unserialize(&mut self, io: &mut std::io::Read, stype:i32) -> Result {
      CompactSize::Unserialize(&mut self.value, io, stype)
   }
}

impl <T> Serializable for Vec<T> where T:Clone + Default + Serializable {
   fn get_serialize_size(&self) -> usize {
      let mut r:usize = 0;
      r += CompactSize::GetSerializeSize(self.len() as u64);
      for v in self {
         r += v.get_serialize_size();
      }
      r
   }
   fn serialize(&self, io:&mut std::io::Write, stype:i32) -> Result {
      let mut r:usize = 0;
      r += try!(CompactSize::Serialize(self.len() as u64, io, stype));
      for v in self {
         r += try!(v.serialize(io, stype));
      }
      Ok(r)
   }
   fn unserialize(&mut self, io:&mut std::io::Read, stype:i32) -> Result
   {
      let mut r:usize = 0;
      let mut len:u64 = 0;
      {
         r += try!(CompactSize::Unserialize(&mut len, io, stype));
         self.resize(len as usize, T::default());
      }
      for v in self {
         r += try!(v.unserialize(io, stype));
      }
      Ok(r)
   }
}

pub struct LimitedString {
   pub string: String,
   pub limit:  usize,
}
impl LimitedString {
   pub fn new(s: &str, l:u64) -> LimitedString {
      let lim = std::cmp::min(l, std::u32::MAX as u64) as usize;
      let mut r = LimitedString{ string:String::with_capacity(lim as usize), limit:lim };
      if s.len() <= lim {
         r.string.push_str(s);
      } else {
         r.string.push_str(&s[..lim]);
      }
      r
   }
   #[allow(non_snake_case)]
   pub fn GetSerializeSize(s:&str, l:u64) -> usize {
      let lim = std::cmp::min(l, std::u32::MAX as u64) as usize;
      let len = std::cmp::min(s.len(), lim);
      CompactSize::GetSerializeSize(len as u64) + len
   }
   #[allow(non_snake_case)]
   pub fn Serialize(s:&str, l:u64, io: &mut std::io::Write, stype:i32) -> Result {
      let mut r = 0usize;
      let lim = std::cmp::min(l, std::u32::MAX as u64) as usize;
      let len = std::cmp::min(s.len(), lim);
      r += try!( CompactSize::new(len as u64).serialize(io, stype) );
      r += try!( s.as_bytes()[..len].serialize(io, stype) );
      Ok(r)
   }
}
impl Serializable for LimitedString {
   fn get_serialize_size(&self) -> usize {
      LimitedString::GetSerializeSize(&*self.string, self.limit as u64)
   }
   fn serialize(&self, io: &mut std::io::Write, stype:i32) -> Result {
      LimitedString::Serialize(&*self.string, self.limit as u64, io, stype)
   }
   fn unserialize(&mut self, io: &mut std::io::Read, stype:i32) -> Result {
      let mut r = 0usize;
      let mut total = CompactSize{value:0};
      r += try!(total.unserialize(io, stype));

      let total = total.value as usize;
      let mut buf:Vec<u8> = Vec::new();
      if self.limit < total {
         buf.reserve(self.limit);
         try!(io.read_exact(&mut buf));
         let tmp = &mut vec![0u8; total - self.limit];
         try!(io.read_exact(tmp)); //Can I read without buffer?
      } else {
         buf.reserve(total);
         try!(io.read_exact(&mut buf));
      }
      r += total;
      self.string = try!(String::from_utf8(buf));
      Ok(r)
   }
}

impl Serializable for [u8] {
   fn get_serialize_size(&self) -> usize {
      self.len()
   }
   fn serialize(&self, io:&mut std::io::Write, _stype:i32) -> Result {
      try!(io.write_all(self));
      Ok(self.len())
   }
   fn unserialize(&mut self, io:&mut std::io::Read, _stype:i32) -> Result {
      try!(io.read_exact(self));
      Ok(self.len())
   }
}

macro_rules! IMPL_ARRAY {
   ($n:expr) => {
      impl Serializable for [u8;$n] {
         fn get_serialize_size(&self) -> usize {
            $n
         }
         fn serialize(&self, io:&mut std::io::Write, _stype:i32) -> Result {
            try!(io.write_all(self));
            Ok(self.len())
         }
         fn unserialize(&mut self, io:&mut std::io::Read, _stype:i32) -> Result {
            try!(io.read_exact(self));
            Ok(self.len())
         }
      }
   }
}

IMPL_ARRAY!(0);
IMPL_ARRAY!(1);
IMPL_ARRAY!(2);
IMPL_ARRAY!(3);
IMPL_ARRAY!(4);
IMPL_ARRAY!(5);
IMPL_ARRAY!(6);
IMPL_ARRAY!(7);
IMPL_ARRAY!(8);
IMPL_ARRAY!(9);
IMPL_ARRAY!(10);
IMPL_ARRAY!(11);
IMPL_ARRAY!(12);
IMPL_ARRAY!(13);
IMPL_ARRAY!(14);
IMPL_ARRAY!(15);
IMPL_ARRAY!(16);
IMPL_ARRAY!(17);
IMPL_ARRAY!(18);
IMPL_ARRAY!(19);

