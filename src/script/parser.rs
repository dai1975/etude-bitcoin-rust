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
   pub fn is_end(&self) -> bool { self.cursor >= self.bytecode.len() }
}

#[derive(Debug,Clone)]
pub struct Parsed(pub usize, pub u8, pub Vec<u8>);

impl <'a> Parser<'a> {
   pub fn parse_bytes(&mut self, size:usize) -> Option<Vec<u8>> {
      let b = self.bytecode;
      let i = self.cursor;
      if i + size >= b.len() { return None; }

      self.cursor += size;
      Some(b[i..(self.cursor)].to_vec())
   }

   pub fn parse_op(&mut self) -> Option<Parsed> {
      let b = self.bytecode;
      let mut i = self.cursor;
      if i+1 > b.len() { return None; }

      let cursor0 = self.cursor;
      let code = b[i];
      //println!("    next. len={}, code[{}]={:x}={}...", self.bytecode.len(), i, code, OPCODE_INFO[code as usize].name);
      i += 1;
      let size:usize = match code {
         _x if OP_PUSHDATA1 > code => {
            code as usize
         },
         OP_PUSHDATA1 => {
            if i+1 > b.len() { return None; }
            let v = b[i];
            i += 1;
            v as usize
         },
         OP_PUSHDATA2 => {
            if i+2 > b.len() { return None; }
            let v:u16 = (b[i] as u16) | (b[i+1] as u16) << 8;
            i += 2;
            v as usize
         },
         OP_PUSHDATA4 => {
            if i+4 > b.len() { return None; }
            let v:u32 = (b[i] as u32) | (b[i+1] as u32) << 8 | (b[i+2] as u32) << 16 | (b[i+3] as u32) << 24;
            i += 4;
            v as usize
         },
         _ => 0
      };
      if i+size > b.len() { return None; }
      self.cursor = i + size;
      Some(Parsed(cursor0, code, b[i..(self.cursor)].to_vec()))
   }
}

