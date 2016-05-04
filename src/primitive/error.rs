use std;
use serialize::SerializeError;
use super::ParseUInt256Error;

#[derive(Debug)]
pub enum Error {
   Io(std::io::Error),
   Utf8(std::string::FromUtf8Error),
   ParseInt(std::num::ParseIntError),

   ParseUInt256(ParseUInt256Error),
   Serialize(SerializeError),
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

impl From<std::num::ParseIntError> for Error {
   fn from(err: std::num::ParseIntError) -> Error {
      Error::ParseInt(err)
   }
}

impl From<ParseUInt256Error> for Error {
   fn from(err: ParseUInt256Error) -> Error {
      Error::ParseUInt256(err)
   }
}

impl From<SerializeError> for Error {
   fn from(err: SerializeError) -> Error {
      Error::Serialize(err)
   }
}

