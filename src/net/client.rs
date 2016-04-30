/// simple bitcoin client.
/// connect to bitcoin node and do handshake

use std;
use std::sync::Arc;
use std::io::{self,Read,Write};
use std::collections::HashMap;
extern crate net2;
use protocol;
use serialize::{self, Serializable, UInt256};
use blockchain::{BlockHeader};

#[allow(dead_code)]
struct ByteBuf<'a>(&'a [u8]);
impl<'a> std::fmt::LowerHex for ByteBuf<'a> {
    fn fmt(&self, fmtr: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for byte in self.0 {
            try!( fmtr.write_fmt(format_args!("{:02x}", byte)));
        }
        Ok(())
    }
}

enum BlockStatus {
   Init      = 0,
   GetHeader = 1,
}
struct BlockData {
   status: BlockStatus,
   hash:   UInt256,
   header: Option<BlockHeader>,
   next:   Option<UInt256>,
}
impl BlockData {
   fn new(hash: &UInt256) -> BlockData {
      BlockData {
         status: BlockStatus::Init,
         hash:   hash.clone(),
         header: None,
         next:   None,
      }
   }
}
#[derive(Default)]
pub struct BlockDB {
   map: std::collections::HashMap< UInt256, BlockData >,
}
impl BlockDB {
   fn get(&self, hash: &UInt256) -> Option<&BlockData> {
      self.map.get(hash)
   }
   fn get_mut(&mut self, hash: &UInt256) -> Option<&mut BlockData> {
      self.map.get_mut(hash)
   }
   fn insert(&mut self, hash: &UInt256) -> Result<&mut BlockData, &mut BlockData> {
      use std::collections::hash_map::Entry::{Occupied,Vacant};
      match self.map.entry(hash.clone()) {
         Vacant(v)   => Ok(v.insert(BlockData::new(hash))),
         Occupied(o) => Err(o.into_mut())
      }
   }
}

pub struct Client {
   disconn: bool,
   stream: Option<std::net::TcpStream>,
   version: i32,

   recv_buffer: super::RingBuffer,
   recv_mode: i32,
   recv_header: protocol::MessageHeader,
   recv_serialize_param: serialize::SerializeParam,

   send_buffer: Vec<u8>,
   send_queue: std::collections::LinkedList< Box<protocol::Message> >,
   send_serialize_param: serialize::SerializeParam,

   pub blocks: BlockDB,
}

impl Client {
   pub fn new() -> Client {
      Client {
         disconn: false,
         stream:  None,
         version: 0,
         recv_buffer: super::RingBuffer::new(1280),
         recv_mode: 0,
         recv_header: protocol::MessageHeader::default(),
         recv_serialize_param: serialize::SerializeParam{
            sertype: serialize::SER_NET,
            version: protocol::INIT_PROTO_VERSION,
         },

         send_buffer: vec![0u8; 1280],
         send_queue:  std::collections::LinkedList::new(),
         send_serialize_param: serialize::SerializeParam{
            sertype: serialize::SER_NET,
            version: protocol::INIT_PROTO_VERSION,
         },

         blocks: BlockDB::default(),
      }
   }
   pub fn run(&mut self, addr: String) -> Result<bool,serialize::Error> {
      try!(self.connect(addr));
      println!("connected");
      try!(self.send_version());
      try!(self.ioloop());
      println!("end");
      try!(self.close());
      Ok(true)
   }

   fn close(&mut self) -> Result< (), serialize::Error > {
      match self.stream {
         Some(ref mut s) => {
            let _ = s.shutdown(std::net::Shutdown::Both);
         }
         None => ()
      }
      self.stream = None;
      Ok(())
   }

   fn connect(&mut self, addr: String) -> Result< bool, serialize::Error > {
      match self.stream {
         Some(_) => {
            try!(Err(io::Error::new(io::ErrorKind::AlreadyExists, "already connected")))
         }
         None    => {
            let tcp = net2::TcpBuilder::new_v4().unwrap();
            // trace とれるように try!() に分解すべき
            match Ok(&tcp)
               .and_then(move|ref t| { t.reuse_address(true)})
               .and_then(move|ref t| { t.connect(&*addr)})
            {
               Ok(s)  => {
                  try!(s.set_nonblocking(true));
                  self.stream = Some(s);
                  self.recv_mode = 0;
                  self.recv_buffer.clear();
                  Ok(true)
               },
               Err(e) => try!(Err(e))
            }
         }
      }
   }

   fn push(&mut self, obj: Box<protocol::Message>) {
      self.send_queue.push_back(obj);
   }

   fn send(&mut self, obj: &protocol::Message) -> Result< (), serialize::Error >
   {
      println!("send message: {}", &obj);
      if self.stream.is_none() {
         try!(Err(io::Error::new(io::ErrorKind::NotConnected, "not connected")));
      }
      let mut hdr = protocol::MessageHeader::default();

      let hdrsize = hdr.get_serialize_size(&self.send_serialize_param);
      let objsize = obj.get_serialize_size(&self.send_serialize_param);
      if self.send_buffer.len() < hdrsize + objsize {
         self.send_buffer.resize(hdrsize + objsize, 0u8);
      }

      try!(obj.serialize(&mut &mut self.send_buffer[hdrsize..], &self.send_serialize_param));
      hdr.set_data(&obj.get_command(), &self.send_buffer[hdrsize..]);
      try!(hdr.serialize(&mut &mut self.send_buffer[0..], &self.send_serialize_param));

      //println!("sent:");
      //println!("{:x}", ByteBuf(&buf[..]));

      match self.stream.as_ref() {
         None => (),
         Some(ref mut s) => {
            try!(s.write_all(&self.send_buffer));
         }
      }
      Ok(())
   }

   fn send_version(&mut self) -> Result< (), serialize::Error > {
      if self.stream.is_none() { try!(Err(io::Error::new(io::ErrorKind::NotConnected, "not connected"))) }

      let mut msg = Box::new(protocol::VersionMessage::default());
      msg.version = ::protocol::PROTOCOL_VERSION;
      {
         let s = self.stream.as_ref().unwrap();
         msg.addr_me.set_services(0).set_ip(&s.local_addr().unwrap());
         msg.addr_you.set_services(0).set_ip(&s.peer_addr().unwrap());
      }
      self.push(msg);
      Ok(())
   }

   fn ioloop(&mut self) -> Result< (), serialize::Error > {
      if self.stream.is_none() { try!(Err(io::Error::new(io::ErrorKind::NotConnected, "not connected"))) }

      loop { 
         //println!("buf: {}", &self.recv_buffer);
         { //read
            match self.stream.as_ref().unwrap().read(self.recv_buffer.as_mut_slice()) {
               Err(e) => {
                  match e.kind() {
                     std::io::ErrorKind::WouldBlock => (),
                     _ => try!(Err(e))
                  }
               }
               Ok(0) => { () }
               Ok(size) => {
                  //println!("recv: {} {:x}", size, ByteBuf(self.recv_buffer.as_mut_slice()));
                  self.recv_buffer.skip_write(size);
                  //println!("recv:  -> {}", &self.recv_buffer);
                  if self.disconn {
                     self.recv_buffer.skip_read(size);
                  } else {
                     loop {
                        match try!(self.on_recv()) {
                           0 => { break; }
                           s => {
                              //println!("on_recv consume: {}", s);
                              self.recv_buffer.skip_read(s);
                           }
                        }
                     }
                  }
               }
            }
         }

         {
            while !self.send_queue.is_empty() {
               let b = self.send_queue.pop_front().unwrap();
               try!(self.send(&*b));
            }
         }
         if self.disconn && self.send_queue.is_empty() {
            break;
         }
         std::thread::sleep(std::time::Duration::from_millis(10));
      }
      Ok(())
   }

   fn on_recv(&mut self) -> Result<usize, serialize::Error> {
      let mut r:usize = 0;
      if self.recv_mode == 0 { //recv header
         if self.recv_buffer.readable_size() < protocol::MessageHeader::GetSerializableSize() {
            return Ok(0)
         };
         r = try!(self.recv_header.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
         self.recv_mode = 1;

      } else if self.recv_mode == 1 { //recv body
         if self.recv_buffer.readable_size() < self.recv_header.size as usize {
            return Ok(0)
         };
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
         r = self.recv_header.size as usize;
         self.recv_mode = 0;
      }
      Ok(r)
   }

   fn on_recv_version(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::VersionMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {:?}", &msg);
      use protocol::msg_reject::*;
      if 0 < self.version {
         self.push(Box::new(protocol::RejectMessage::new(&msg, REJECT_DUPLICATE, &"duplicate version".to_string())));
         self.disconn = true;
      } else if msg.version < protocol::GETHEADERS_VERSION {
         self.push(Box::new(protocol::RejectMessage::new(&msg, REJECT_OBSOLETE, &"too old version".to_string())));
         self.disconn = true;
      } else {
         self.version = msg.version;
         self.push(Box::new(protocol::VerAckMessage::default()));
         self.send_serialize_param.version = std::cmp::min(msg.version, protocol::PROTOCOL_VERSION);
      }
      Ok(r)
   }
   fn on_recv_verack(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::VerAckMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {:?}", &msg);
      self.recv_serialize_param.version = std::cmp::min(self.version, protocol::PROTOCOL_VERSION);
      Ok(r)
   }
   fn on_recv_addr(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::AddrMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_inv(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::InvMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {:?}", &msg);
      for inv in msg.invs.into_iter() {
         match inv.blocktype {
            protocol::msg_inv::MessageBlockType::Block => {
               // To release self borrowing, returs cloned value.
               match self.blocks.insert(&inv.hash).ok().map(|rBlock| rBlock.hash.clone()) {
                  Some(hash) => {
                     // succeed in insert means the block is unknown. Request block header.
                     self.push(Box::new(protocol::GetHeadersMessage::new(&hash)));
                  }
                  None => ()
               };
            }
            _ => {}
         }

      }
      Ok(r)
   }
   fn on_recv_getdata(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::GetDataMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_merkleblock(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::MerkleBlockMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_getblocks(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::GetBlocksMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_getheaders(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::GetHeadersMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_tx(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::TxMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_headers(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::HeadersMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {:?}", &msg);
      for h in msg.headers.iter() {
      }
      Ok(r)
   }
   fn on_recv_block(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::BlockMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_getaddr(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::GetAddrMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_mempool(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::MemPoolMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_ping(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::PingMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {:?}", &msg);
      self.push(Box::new(protocol::PongMessage::new(&msg)));
      Ok(r)
   }
   fn on_recv_pong(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::PongMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_alert(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::AlertMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_notfound(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::NotFoundMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_filterload(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::FilterLoadMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_filteradd(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::FilterAddMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_filterclear(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::FilterClearMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_reject(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::RejectMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_sendheaders(&mut self) -> Result<usize, serialize::Error> {
      let mut r = 0usize;
      let mut msg = protocol::SendHeadersMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {:?}", &msg);
      Ok(r)
   }
   fn on_recv_unknown(&mut self) -> Result<usize, serialize::Error> {
      println!("unknown command: {:?}\n", self.recv_header.command);
      //try!(Err(serialize::SerializeError::new("unknown command")));
      Ok(0)
   }
}
