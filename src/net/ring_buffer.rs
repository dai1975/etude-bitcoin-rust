use std;

pub struct RingBuffer {
   buf: Vec<u8>,
   size: usize,
   rpos: usize,
   wpos: usize,
}

impl std::fmt::Display for RingBuffer {
   fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "RingBuffer(size={}, rpos={}, wpos={})", self.size, self.rpos, self.wpos)
   }
}

impl RingBuffer {
   pub fn new(size:usize) -> RingBuffer {
      RingBuffer {
         size: size,
         buf: vec![0u8; size*2],
         rpos: 0,
         wpos: 0,
      }
   }

   pub fn clear(&mut self) {
      self.rpos = 0;
      self.wpos = 0;
   }

   pub fn readable_size(&self) -> usize {
      self.wpos - self.rpos
   }
   pub fn writable_size(&self) -> usize {
      self.size - (self.wpos - self.rpos)
   }

   pub fn skip_read(&mut self, size:usize) {
      assert!(self.rpos + size <= self.wpos);
      self.rpos += size;
      if self.size <= self.rpos {
         self.rpos -= self.size;
         self.wpos -= self.size;
         let (l,r) = self.buf.split_at_mut(self.size);
         l[self.rpos .. self.wpos].clone_from_slice(&r[self.rpos .. self.wpos]);
      }
   }
   pub fn skip_write(&mut self, size:usize) {
      assert!(self.wpos + size <= self.rpos + self.size);
      self.wpos += size;
   }

   pub fn as_slice(&self) -> &[u8] {
      &self.buf[self.rpos .. self.wpos]
   }

   pub fn as_mut_slice(&mut self) -> &mut[u8] {
      &mut self.buf[self.wpos .. self.rpos+self.size]
   }
}

