use std;
use super::{ScriptError, Script, Parser, Parsed, ScriptNum, signature};
use super::opcode::*;
use ::serialize::{self, Serializable};
use primitive::{hasher, Transaction};

#[allow(dead_code)]
struct ByteBuf<'a>(&'a [u8]);
impl<'a> std::fmt::LowerHex for ByteBuf<'a> {
    fn fmt(&self, fmtr: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for byte in self.0 {
            try!( fmtr.write_fmt(format_args!("{:02x}", byte)));
        }
        Ok(())
    }
}

#[derive(Debug,Clone)]
pub struct Interpreter
{
   stack: Vec<Vec<u8>>,
   codesep_pos: usize,
}

impl Interpreter {
   pub fn new() -> Interpreter {
      Interpreter {
         stack:  Vec::new(),
         codesep_pos: 0usize,
      }
   }

   pub fn result(&self) -> bool {
      match self.stack.last() {
         None => false,
         Some(vch) => vch.len() != 0
      }
   }

   pub fn eval(&mut self, script:&Script, tx:&Transaction, in_idx:usize, flags:u32) -> Result<(), ScriptError> {
      //println!("eval: {}", script);
      let checker = signature::Checker::new(tx, in_idx);
      let mut parser = Parser::new(&script.bytecode);
      self.codesep_pos = 0usize;
      while let Some(Parsed(pos, code, ref follow)) = parser.parse_op() {
         let info = &OPCODE_INFO[code as usize];
         println!("{:x}={}[{}]", code, info.name, follow.len());

         try!(self.operate(script, pos, code, follow, flags, &checker));

         for (i,v) in self.stack.iter().enumerate() {
            println!("  [{}] {:x}", i, ByteBuf(&v[..]));
         }

      }
      if !parser.is_end() {
         return Err(ScriptError::new("end before fin"));
      }
      Ok(())
   }

   fn push_true(&mut self) { self.stack.push(vec![1u8]); }
   fn push_false(&mut self) { self.stack.push(vec![]); }

   fn operate(&mut self, script:&Script, pos:usize, code:u8, follow:&Vec<u8>, flags:u32, checker:&signature::Checker) -> Result<(), ScriptError> {
      let sp = serialize::SerializeParam::new_net();
      match code {
         _x if code <= OP_PUSHDATA4 => {
            self.stack.push(follow.clone());
            Ok(())
         },
         _x if code <= OP_16 => {
            let n = ScriptNum::new((code - (OP_1 - 1)) as i64);
            self.stack.push(Vec::<u8>::new());
            n.serialize(self.stack.last_mut().unwrap(), &sp).unwrap();
            Ok(())
         },
         OP_DUP => {
            if self.stack.len() < 1 { return Err(ScriptError::new("few stacks")); }
            let vch = self.stack.last().unwrap().clone();
            self.stack.push(vch);
            Ok(())
         },
         OP_HASH160 => {
            if self.stack.len() < 1 { return Err(ScriptError::new("few stacks")); }
            let rvch = self.stack.last_mut().unwrap();
            let hash = hasher::hash160(&rvch[..]);
            rvch.resize(hash.len(), 0u8);
            rvch.clone_from_slice(&hash[..]);
            Ok(())
         },
         _x if (code == OP_EQUAL || code == OP_EQUALVERIFY) => {
            if self.stack.len() < 2 { return Err(ScriptError::new("few stacks")); }
            let vch1 = self.stack.pop().unwrap();
            let vch2 = self.stack.pop().unwrap();
            match (code, vch1 == vch2) {
               (OP_EQUAL,true)  => { self.push_true(); Ok(()) },
               (OP_EQUAL,false) => { self.push_false(); Ok(()) },
               (OP_EQUALVERIFY,true)  => Ok(()),
               (OP_EQUALVERIFY,false) => Err(ScriptError::new("equalverify")),
               _ => Err(ScriptError::new("not reach")),
            }
         }
         OP_CODESEPARATOR => {
            self.codesep_pos = pos;
            Ok(())
         },
         _x if (code == OP_CHECKSIG || code == OP_CHECKSIGVERIFY) => {
            if self.stack.len() < 2 { return Err(ScriptError::new("few stacks")); }
            let pubkey = self.stack.pop().unwrap();
            let sig    = self.stack.pop().unwrap();
            let target = &script.bytecode[self.codesep_pos .. ];
            let r = checker.verify(target, &pubkey[..], &sig[..], flags);
            match code {
               OP_CHECKSIG => {
                  match r {
                     Ok(_)  => self.push_true(),
                     Err(_) => self.push_false(),
                  };
                  Ok(())
               },
               OP_CHECKSIGVERIFY => {
                  r
               },
               _ => Err(ScriptError::new("not reach")),
            }
         },
         _ => {
            let info = &OPCODE_INFO[code as usize];
            println!("  unimplemented {}(0x{:x})", info.name, code);
            Ok(())
         },
      }
   }
}
