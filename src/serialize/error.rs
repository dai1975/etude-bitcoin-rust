use std;

#[derive(Debug,PartialEq)]
pub struct SerializeError {
   msg: String
}

impl SerializeError {
   pub fn new(s:String) -> SerializeError {
      SerializeError { msg:s }
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
