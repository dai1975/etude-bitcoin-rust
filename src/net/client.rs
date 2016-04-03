/// simple bitcoin client.
/// connect to bitcoin node and do handshake

use std;
use std::io::{self,Read,Write};
use protocol;
use serialize::{self, Serializable};

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
   serialize_param: serialize::SerializeParam,
}

impl Client {
   pub fn new() -> Client {
      Client {
         stream:None,
         recv_buffer: super::RingBuffer::new(1280),
         recv_mode: 0,
         recv_header: protocol::MessageHeader::default(),
         serialize_param: serialize::SerializeParam{
            sertype: serialize::SER_NET,
            version: 0,
         }
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
      self.serialize_param.version = protocol::PROTOCOL_VERSION;
      let mut hdr = protocol::MessageHeader::default();

      let hdrsize = hdr.get_serialize_size(&self.serialize_param);
      let objsize = obj.get_serialize_size(&self.serialize_param);
      let mut buf = vec![0u8; hdrsize + objsize];

      try!(obj.serialize(&mut &mut buf[hdrsize..], &self.serialize_param));
      hdr.set_data("version", &buf[hdrsize..]);
      try!(hdr.serialize(&mut &mut buf[0..], &self.serialize_param));

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

      let mut msg = protocol::VersionMessage::default();
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
         let r = try!(self.recv_header.unserialize(&mut self.recv_buffer.as_slice(), &self.serialize_param));
         self.recv_buffer.skip_read(r);
         println!("recv header: {}", &self.recv_header);
         self.recv_mode = 1;
      }
      if self.recv_mode == 1 { //recv body
         if self.recv_buffer.readable_size() < self.recv_header.size as usize { return Ok(0) };
         let result = match self.recv_header.command {
            ::protocol::message_header::COMMAND_VERSION     => self.on_recv_version(),
            ::protocol::message_header::COMMAND_VERACK      => self.on_recv_verack(),
            ::protocol::message_header::COMMAND_ADDR        => self.on_recv_addr(),
            ::protocol::message_header::COMMAND_INV         => self.on_recv_inv(),
            ::protocol::message_header::COMMAND_GETDATA     => self.on_recv_getdata(),
            ::protocol::message_header::COMMAND_MERKLEBLOCK => self.on_recv_merkleblock(),
            ::protocol::message_header::COMMAND_GETBLOCKS   => self.on_recv_getblocks(),
            ::protocol::message_header::COMMAND_GETHEADERS  => self.on_recv_getheaders(),
            ::protocol::message_header::COMMAND_TX          => self.on_recv_tx(),
            ::protocol::message_header::COMMAND_HEADERS     => self.on_recv_headers(),
            ::protocol::message_header::COMMAND_BLOCK       => self.on_recv_block(),
            ::protocol::message_header::COMMAND_GETADDR     => self.on_recv_getaddr(),
            ::protocol::message_header::COMMAND_MEMPOOL     => self.on_recv_mempool(),
            ::protocol::message_header::COMMAND_PING        => self.on_recv_ping(),
            ::protocol::message_header::COMMAND_PONG        => self.on_recv_pong(),
            ::protocol::message_header::COMMAND_ALERT       => self.on_recv_alert(),
            ::protocol::message_header::COMMAND_NOTFOUND    => self.on_recv_notfound(),
            ::protocol::message_header::COMMAND_FILTERLOAD  => self.on_recv_filterload(),
            ::protocol::message_header::COMMAND_FILTERADD   => self.on_recv_filteradd(),
            ::protocol::message_header::COMMAND_FILTERCLEAR => self.on_recv_filterclear(),
            ::protocol::message_header::COMMAND_REJECT      => self.on_recv_reject(),
            ::protocol::message_header::COMMAND_SENDHEADERS => self.on_recv_sendheaders(),
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
      let mut msg = protocol::VersionMessage::default();
      r += try!(msg.unserialize(&mut self.recv_buffer.as_slice(), &self.serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_verack(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::VerAckMessage::default();
      r += try!(msg.unserialize(&mut self.recv_buffer.as_slice(), &self.serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_addr(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::AddrMessage::default();
      r += try!(msg.unserialize(&mut self.recv_buffer.as_slice(), &self.serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_inv(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::InvMessage::default();
      r += try!(msg.unserialize(&mut self.recv_buffer.as_slice(), &self.serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_getdata(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::GetDataMessage::default();
      r += try!(msg.unserialize(&mut self.recv_buffer.as_slice(), &self.serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_merkleblock(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::MerkleBlockMessage::default();
      r += try!(msg.unserialize(&mut self.recv_buffer.as_slice(), &self.serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_getblocks(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::GetBlocksMessage::default();
      r += try!(msg.unserialize(&mut self.recv_buffer.as_slice(), &self.serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_getheaders(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::GetHeadersMessage::default();
      r += try!(msg.unserialize(&mut self.recv_buffer.as_slice(), &self.serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_tx(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::TxMessage::default();
      r += try!(msg.unserialize(&mut self.recv_buffer.as_slice(), &self.serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_headers(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::HeadersMessage::default();
      r += try!(msg.unserialize(&mut self.recv_buffer.as_slice(), &self.serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_block(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::BlockMessage::default();
      r += try!(msg.unserialize(&mut self.recv_buffer.as_slice(), &self.serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_getaddr(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::GetAddrMessage::default();
      r += try!(msg.unserialize(&mut self.recv_buffer.as_slice(), &self.serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_mempool(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::MemPoolMessage::default();
      r += try!(msg.unserialize(&mut self.recv_buffer.as_slice(), &self.serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_ping(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::PingMessage::default();
      r += try!(msg.unserialize(&mut self.recv_buffer.as_slice(), &self.serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_pong(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::PongMessage::default();
      r += try!(msg.unserialize(&mut self.recv_buffer.as_slice(), &self.serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_alert(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::VerAckMessage::default();
      r += try!(msg.unserialize(&mut self.recv_buffer.as_slice(), &self.serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_notfound(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::VerAckMessage::default();
      r += try!(msg.unserialize(&mut self.recv_buffer.as_slice(), &self.serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_filterload(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::VerAckMessage::default();
      r += try!(msg.unserialize(&mut self.recv_buffer.as_slice(), &self.serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_filteradd(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::VerAckMessage::default();
      r += try!(msg.unserialize(&mut self.recv_buffer.as_slice(), &self.serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_filterclear(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::VerAckMessage::default();
      r += try!(msg.unserialize(&mut self.recv_buffer.as_slice(), &self.serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_reject(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::VerAckMessage::default();
      r += try!(msg.unserialize(&mut self.recv_buffer.as_slice(), &self.serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_sendheaders(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::VerAckMessage::default();
      r += try!(msg.unserialize(&mut self.recv_buffer.as_slice(), &self.serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_unknown(&mut self) -> Result<usize, serialize::Error> {
      println!("unknown command: {:?}\n", self.recv_header.command);
      //try!(Err(serialize::SerializeError::new("unknown command")));
      Ok(0)
   }
}
