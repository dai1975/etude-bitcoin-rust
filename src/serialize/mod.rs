pub use self::error::SerializeError;
pub use self::serialize::Result;
pub use self::serialize::SerializeParam;
pub use self::serialize::Serializable;
pub use self::serialize::CompactSize;
pub use self::serialize::LimitedString;
pub use self::serialize::{SER_NET, SER_DISK, SER_GETHASH};

pub mod error;
#[macro_use]
pub mod serialize;

#[allow(dead_code)]
pub struct ByteBuf<'a>(pub &'a [u8]);
impl<'a> ::std::fmt::LowerHex for ByteBuf<'a> {
    fn fmt(&self, fmtr: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        for byte in self.0 {
            try!( fmtr.write_fmt(format_args!("{:02x}", byte)));
        }
        Ok(())
    }
}
