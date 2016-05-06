use super::opcode::*;

#[derive(Debug,Clone)]
pub struct Parser<'a>
{
   bytecode: &'a Vec<u8>,
   cursor: usize,
}

impl <'x> Parser<'x> {
   pub fn new<'a>(bytecode:&'a Vec<u8>) -> Parser<'a> {
      Parser {
         bytecode: bytecode,
         cursor: 0,
      }
   }
   // If has_next() returns true but next() returns None it means an error.
   pub fn has_next(&self) -> bool { self.cursor < self.bytecode.len() }
}

#[derive(Debug,Clone)]
pub struct Parsed(pub u8, pub Vec<u8>);

impl <'a> Iterator for Parser<'a> {
   type Item = Parsed;
   fn next(&mut self) -> Option<Self::Item> {
      println!("    next. cur={}, len={}", self.cursor, self.bytecode.len());
      let b = self.bytecode;
      let mut i = self.cursor;
      if b.len() <= i { return None; }

      let code = b[i];
      println!("    code[{}]={}", i, code);
      i += 1;
      let size:usize = match code {
         _ if code < OP_PUSHDATA1 => {
            code as usize
         },
         OP_PUSHDATA1 => {
            if b.len() - i < 1 { return None; }
            let v = b[i];
            i += 1;
            v as usize
         },
         OP_PUSHDATA2 => {
            if b.len() - i < 2 { return None; }
            let v:u16 = (b[i] as u16) | (b[i+1] as u16) << 8;
            i += 2;
            v as usize
         },
         OP_PUSHDATA4 => {
            if b.len() - i < 4 { return None; }
            let v:u32 = (b[i] as u32) | (b[i+1] as u32) << 8 | (b[i+2] as u32) << 16 | (b[i+3] as u32) << 24;
            i += 4;
            v as usize
         },
         _ => 0
      };
      if b.len() - i < size { return None; }
      self.cursor = i + size;
      Some(Parsed(code, b[i..(i+size)].to_vec()))
   }
}

