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

   pub fn eval(&self, script:&Script) -> Result<(), ScriptError> {
      println!("eval: script={}", script.bytecode.len());
      let mut parser = Parser::new(&script.bytecode);
      while parser.has_next() {
         match parser.next() {
            Some(Parsed(code,follow)) => {
//      parser.map(|Parsed(code, follow)| {
               let info = &OPCODE_INFO[code as usize];
               println!("{:x}={}[{}]", code, info.name, follow.len());
            },
            None => { println!("end before fin"); }
         }
//      });
      }
      Ok(())
   }
}
