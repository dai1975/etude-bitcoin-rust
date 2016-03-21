/// simple bitcoin client.
/// connect to bitcoin node and do handshake

use std;
use std::io::{self,Read,Write};
use protocol;
use serialize;
use serialize::Serializable;

struct ByteBuf<'a>(&'a [u8]);
impl<'a> std::fmt::LowerHex for ByteBuf<'a> {
    fn fmt(&self, fmtr: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for byte in self.0 {
            try!( fmtr.write_fmt(format_args!("{:02x}", byte)));
        }
        Ok(())
    }
}

pub struct Client {
   stream: Option<std::net::TcpStream>,
   recv_buffer: super::RingBuffer,
   recv_mode: i32,
   recv_header: protocol::MessageHeader,
}

impl Client {
   pub fn new() -> Client {
      Client {
         stream:None,
         recv_buffer: super::RingBuffer::new(1280),
         recv_mode: 0,
         recv_header: protocol::MessageHeader::new(),
      }
   }
   pub fn run(&mut self, addr: String) -> Result<bool,serialize::Error> {
      try!(self.connect(addr));
      println!("connected");
      try!(self.send_version());
      println!("sent");
      try!(self.ioloop());
      Ok(true)
   }

   fn connect(&mut self, addr: String) -> Result< bool, serialize::Error > {
      match self.stream {
         Some(_) =>
            try!(Err(io::Error::new(io::ErrorKind::AlreadyExists, "already connected"))),
         None    =>
            match std::net::TcpStream::connect(&*addr) {
               Ok(s)  => {
                  self.stream = Some(s);
                  self.recv_mode = 0;
                  self.recv_buffer.clear();
                  Ok(true)
               },
               Err(e) => try!(Err(e))
            }
      }
   }

   fn send(&mut self, obj: &Serializable) -> Result< (), serialize::Error > {
      if self.stream.is_none() {
         try!(Err(io::Error::new(io::ErrorKind::NotConnected, "not connected")));
      }

      let mut hdr = protocol::MessageHeader::new();
      let hdrsize = hdr.get_serialize_size();
      let objsize = obj.get_serialize_size();
      let mut buf = vec![0u8; hdrsize + objsize];
      try!(obj.serialize(&mut &mut buf[hdrsize..])); //impl Write for &mut[u8] なので、&mut Write にするには &mut &mut[u8]
      hdr.set_data("version", &buf[hdrsize..]);
      try!(hdr.serialize(&mut &mut buf[0..]));

      println!("sent:");
      println!("{:x}", ByteBuf(&buf[..]));

      match self.stream.as_ref() {
         None => (),
         Some(ref mut s) => {
            try!(s.write_all(&buf));
         }
      }
      Ok(())
   }

   fn send_version(&mut self) -> Result< (), serialize::Error > {
      if self.stream.is_none() { try!(Err(io::Error::new(io::ErrorKind::NotConnected, "not connected"))) }

      let mut msg = protocol::VersionMessage::new();
      {
         let s = self.stream.as_ref().unwrap();
         msg.addr_me.set_services(0).set_ip(&s.local_addr().unwrap());
         msg.addr_you.set_services(0).set_ip(&s.peer_addr().unwrap());
      }
      self.send(&msg)
   }

   fn ioloop(&mut self) -> Result< (), serialize::Error > {
      if self.stream.is_none() { try!(Err(io::Error::new(io::ErrorKind::NotConnected, "not connected"))) }

      loop { 
         println!("buf: {}", &self.recv_buffer);
         let size = try!(self.stream.as_ref().unwrap().read(self.recv_buffer.as_mut_slice()));
         if 0 < size {
            println!("recv: {} {:x}", size, ByteBuf(self.recv_buffer.as_mut_slice()));
            self.recv_buffer.skip_write(size);
            println!("recv:  -> {}", &self.recv_buffer);
         }
         let size = try!(self.on_recv());
         if 0 < size {
            self.recv_buffer.skip_read(size);
            println!("consume: {}; -> {}", size, &self.recv_buffer);
         }
      }
   }

   fn on_recv(&mut self) -> Result<usize, serialize::Error> {
      if self.recv_mode == 0 { //recv header
         if self.recv_buffer.readable_size() < protocol::MessageHeader::GetSerializableSize() { return Ok(0) };
         let r = try!(self.recv_header.unserialize(&mut self.recv_buffer.as_slice()));
         self.recv_buffer.skip_read(r);
         println!("recv header: {}", &self.recv_header);
         self.recv_mode = 1;
      }
      if self.recv_mode == 1 { //recv body
         if self.recv_buffer.readable_size() < self.recv_header.size as usize { return Ok(0) };
         let result = match self.recv_header.command {
            ::protocol::message::header::COMMAND_VERSION => self.on_recv_version(),
            _ => self.on_recv_unknown()
         };
         try!(result);
         self.recv_buffer.skip_read(self.recv_header.size as usize);
         self.recv_mode = 0;
      }
      Ok(0)
   }

   fn on_recv_version(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::VersionMessage::new();
      r += try!(msg.unserialize(&mut self.recv_buffer.as_slice()));
      println!("recv version: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_unknown(&mut self) -> Result<usize, serialize::Error> {
      println!("unknown command: {:?}\n", self.recv_header.command);
      //try!(Err(serialize::SerializeError::new("unknown command")));
      Ok(0)
   }
}
