use std;
use ::serialize::{self, Serializable};

#[derive(Debug,Clone)]
pub struct ScriptNum {
   pub value: i64,
}

impl std::fmt::Display for ScriptNum {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "scriptnum({})", self.value) //ByteBuf(&self.bytecode[..]))
   }
}

impl ScriptNum {
   pub fn new(v:i64) -> ScriptNum {
      ScriptNum { value:v }
   }
}

fn serialize(out:&mut [u8;9], val:i64) -> usize {
   if val == 0 {
      return 0usize
   }

   let (neg, mut v) = if val < 0 { (true, -val) } else { (false, val) };

   let mut i:usize = 0;
   while 0 < v {
      out[i] = (v & 0xFF) as u8;
      v >>= 8;
      i += 1;
   }

   if (out[i-1] & 0x80) != 0 {
      out[i] = if neg { 0x80 } else { 0 };
      i += 1;
   } else if neg {
      out[i-1] |= 0x80;
   }
   i
}

impl Serializable for ScriptNum {
   fn get_serialize_size(&self, _ser:&serialize::SerializeParam) -> usize {
      let mut tmp = [0u8;9];
      serialize(&mut tmp, self.value)
   }
   fn serialize(&self, io:&mut std::io::Write, _ser:&serialize::SerializeParam) -> serialize::Result {
      let mut tmp = [0u8;9];
      let len = serialize(&mut tmp, self.value);
      try!(io.write_all(&tmp[0..len]));
      println!("scriptnum#serialize: {} -> {:x}", self.value, ::serialize::ByteBuf(&tmp[0..len]));
      Ok(len)
   }
   fn deserialize(&mut self, io:&mut std::io::Read, _ser:&serialize::SerializeParam) -> serialize::Result {
      let mut tmp = [0u8;9];
      let len = try!(io.read(&mut tmp)); //caller garantees to return full data by one read

      let v:i64 = (&tmp[0..len]).iter().enumerate().fold( 0i64, |acc:i64, (i,pv)| {
         let v = *pv as i64;
         if (i == len-1) && (v & 0x80 != 0) {
            acc | (v & 0x7F) << (i*8)
         } else {
            acc | v << (i*8)
         }
      }); 
      self.value = v;
      Ok(len)
   }
}



#[test]
fn test_0() {
   let sp = serialize::SerializeParam::new_net();
   let mut buf = [0u8;9];

   let mut n = ScriptNum::new(0);
   assert_eq!(0, n.serialize(&mut &mut buf[0..], &sp).unwrap());

   n.value = 0x3939;
   assert_eq!(0, n.deserialize(&mut &buf[0..0], &sp).unwrap());
   assert_eq!(0, n.value);
}

#[test]
fn test_0x1234() {
   let sp = serialize::SerializeParam::new_net();
   let mut buf = [0u8;9];

   let mut n = ScriptNum::new(0x1234);
   assert_eq!(2, n.serialize(&mut &mut buf[0..], &sp).unwrap());
   assert_eq!([0x34, 0x12], &buf[0..2]);

   n.value = 0x3939;
   assert_eq!(2, n.deserialize(&mut &buf[0..2], &sp).unwrap());
   assert_eq!(0x1234, n.value);
}

#[test]
fn test_0x80() {
   let sp = serialize::SerializeParam::new_net();
   let mut buf = [0u8;9];

   let mut n = ScriptNum::new(0x80);
   assert_eq!(2, n.serialize(&mut &mut buf[0..], &sp).unwrap());
   assert_eq!([0x80, 0x00], &buf[0..2]);

   n.value = 0x3939;
   assert_eq!(2, n.deserialize(&mut &buf[0..2], &sp).unwrap());
   assert_eq!(0x80, n.value);
}

#[test]
fn test_0x48() {
   let sp = serialize::SerializeParam::new_net();
   let mut buf = Vec::<u8>::new();

   let n = ScriptNum::new(0x48);
   assert_eq!(1, n.serialize(&mut buf, &sp).unwrap());
   assert_eq!([0x48], &buf[0..1]);
}
