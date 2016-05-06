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
      println!("eval: script={}", script.bytecode.len());
      let mut parser = Parser::new(&script.bytecode);
      while !parser.is_end() {
         if let Some(Parsed(code,follow)) = parser.parse_op() {
            try!(self.operate(code, follow));
         } else {
            println!("end before fin");
         }
      }
      Ok(())
   }

   fn operate(&mut self, code:u8, follow:Vec<u8>) -> Result<(), ScriptError> {
      let info = &OPCODE_INFO[code as usize];
      println!("{:x}={}[{}]", code, info.name, follow.len());
      Ok(())
   }
}
