use std;
use std::io::Write;

pub struct DummyWriter {
   len: usize,
}
impl DummyWriter {
   pub fn new() -> DummyWriter { DummyWriter { len:0usize } }
   pub fn len(&self) -> usize { self.len }
}

impl Write for DummyWriter {
   fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
      self.len += bytes.len();
      Ok(bytes.len())
   }
   fn flush(&mut self) -> std::io::Result<()> {
      Ok(())
   }
}

