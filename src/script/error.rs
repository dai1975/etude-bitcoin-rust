use std;

#[derive(Debug,PartialEq)]
pub struct ScriptError {
   msg: String
}

impl ScriptError {
   pub fn new(s:String) -> ScriptError {
      ScriptError { msg:s }
   }
}

impl std::error::Error for ScriptError {
   fn description(&self) -> &str {
      &*self.msg
   }
}
impl std::fmt::Display for ScriptError {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
      write!(f, "{}", self.msg)
   }
}
