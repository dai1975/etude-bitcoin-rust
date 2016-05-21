use super::{ScriptError, Script, Parser, Parsed};
use super::opcode::*;

#[derive(Debug,Clone)]
pub struct Interpreter
{
   stack: Vec<Vec<u8>>,
}

impl Interpreter {
   pub fn new() -> Interpreter {
      Interpreter {
         stack:  Vec::new(),
      }
   }

   pub fn eval(&mut self, script:&Script) -> Result<(), ScriptError> {
      println!("eval: {}", script);
      let mut parser = Parser::new(&script.bytecode);
      while let Some(Parsed(code,follow)) = parser.parse_op() {
         try!(self.operate(code, follow));
      }
      if !parser.is_end() {
         return Err(ScriptError::new("end before fin"));
      }
      Ok(())
   }

   fn operate(&mut self, code:u8, follow:Vec<u8>) -> Result<(), ScriptError> {
      //let info = &OPCODE_INFO[code as usize];
      //println!("{:x}={}[{}]", code, info.name, follow.len());
      match code {
         _x if code <= OP_PUSHDATA4 => {
            self.stack.push(follow);
         },
         _x if code <= OP_16 => {
            //let n = SriptNum::new(code - (OP_1 - 1));
            //self.stack.serialize_push(n);
            ()
         }
         _ => ()
      }
      Ok(())
   }
}
