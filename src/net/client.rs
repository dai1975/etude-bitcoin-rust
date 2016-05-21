/// simple bitcoin client.
/// connect to bitcoin node and do handshake

use std;
use std::io::{self,Read,Write};
extern crate net2;
use protocol;
use serialize::{self, SerializeError, Serializable};
use primitive::{Error,ChainParams,BlockHeader,Block,UInt256,Transaction,GenericError};
use chain::{BlockMap,TransactionMap};
use script::{Script,Interpreter};

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

pub struct Client {
   chain_params: &'static ChainParams,
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

   blocks: BlockMap,
   transactions: TransactionMap,
}

impl Client {
   pub fn new() -> Client {
      Client {
         chain_params: ChainParams::get("test"),
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

         blocks: BlockMap::default(),
         transactions: TransactionMap::default(),
      }
   }
   pub fn run(&mut self, addr: String) -> Result<bool,Error> {
      try!(self.connect(addr));
      println!("connected");
      try!(self.send_version());
      try!(self.ioloop());
      println!("end");
      try!(self.close());
      Ok(true)
   }

   fn close(&mut self) -> Result< (), Error > {
      match self.stream {
         Some(ref mut s) => {
            let _ = s.shutdown(std::net::Shutdown::Both);
         },
         None => ()
      }
      self.stream = None;
      Ok(())
   }

   fn connect(&mut self, addr: String) -> Result< bool, Error > {
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

   fn send(&mut self, obj: &protocol::Message) -> Result< (), Error >
   {
      println!("send message: {}", &obj);
      if self.stream.is_none() {
         try!(Err(io::Error::new(io::ErrorKind::NotConnected, "not connected")));
      }
      let mut hdr = protocol::MessageHeader::default();

      let hdrsize = hdr.get_serialize_size(&self.send_serialize_param);
      let objsize = obj.get_serialize_size(&self.send_serialize_param);
      let msgsize = hdrsize + objsize;
      if self.send_buffer.len() < msgsize {
         self.send_buffer.resize(msgsize, 0u8);
      }

      try!(obj.serialize(&mut &mut self.send_buffer[hdrsize..], &self.send_serialize_param));
      hdr.set_data(&obj.get_command(), &self.send_buffer[hdrsize..msgsize]);
      try!(hdr.serialize(&mut &mut self.send_buffer[0..], &self.send_serialize_param));

//      println!("sent:");
//      println!("{:x}", ByteBuf(&self.send_buffer[0..msgsize]));

      match self.stream.as_ref() {
         None => (),
         Some(ref mut s) => {
            try!(s.write_all(&self.send_buffer[0..msgsize]));
         }
      }
      Ok(())
   }

   fn send_version(&mut self) -> Result< (), Error > {
      if self.stream.is_none() { try!(Err(io::Error::new(io::ErrorKind::NotConnected, "not connected"))) }

      let mut msg = Box::new(protocol::VersionMessage::default());
      msg.version = ::protocol::PROTOCOL_VERSION;
      {
         let s = self.stream.as_ref().unwrap();
         msg.addr_from.set_services(0).set_ip(&s.local_addr().unwrap());
         msg.addr_recv.set_services(0).set_ip(&s.peer_addr().unwrap());
      }
      self.push(msg);
      Ok(())
   }
   fn accept_header(&mut self, header:&BlockHeader) -> Result<(), Error> {
      try!(header.check(&self.chain_params.consensus));

      let key = header.calc_hash();

      // check status of this block in local db
      {
         let mut data = match self.blocks.insert(&key) {
            Ok(data) => {
               data
            },
            Err(data) => {
               if !data.is_init() {
                  //return SerializeError::result::<()>(format!("header is already received: {}", key));
                  return Ok(());
               }
               if key != *data.get_hash() {
                  return SerializeError::result::<()>(format!("hash mismatch: id={}, calc={}", data.get_hash(), key));
               }
               data
            },
         };
         //set data
         println!("accept header: {}", key);
         data.set_header(header.clone());
      }

      // link from prev block
      let prev_hash = &header.hash_prev_block.clone();
      if let Some(prev_data) = self.blocks.get_mut(prev_hash) {
         if let Some(ref prev_next) = *prev_data.get_next() {
            if *prev_next != key {
               println!("Detect a fork. Drop later received one: {} -> {} | {}", prev_hash, prev_next, key);
            }
         }
         if prev_data.get_next().is_none() {
            prev_data.set_next(key.clone());
         }
      }
      Ok(())
   }
   fn accept_block(&mut self, block:&Block) -> Result<(), Error> {
      try!(self.accept_header(&block.header));

      try!(block.check(&self.chain_params.consensus));

      for ptx in block.transactions.iter() {
         let _ = self.accept_transaction(ptx, true);
      }
      Ok(())
   }

   fn accept_transaction(&mut self, tx:&Transaction, request_missing:bool) -> Result<(), Error> {
      let txhash = tx.calc_hash();
      println!("  accept_tx...: hash={}", txhash);
      let mut checks:Vec<UInt256> = Vec::new();
      match self.transactions.insert(&txhash) {
         Ok(e)  => {
            e.set_transaction(tx.clone());
            e
         }
         Err(e) => {
            if e.is_init() {
               e.set_transaction(tx.clone());
               e.move_waiters(&mut checks); //move
            }
            e
         },
      };

      let requests:Vec<UInt256> = tx.ins.iter().filter_map(|pin| {
         match self.transactions.insert(&pin.prevout.hash) {
            Ok(e) => {
               println!("    tx.in[]: not found. hash={}", pin.prevout.hash);
               e.add_waiter(txhash.clone());
               Some(pin.prevout.hash.clone())
            }
            Err(e) => {
               if e.is_init() {
                  println!("    tx.in[]: found but not received. hash={}", pin.prevout.hash);
                  e.add_waiter(txhash.clone());
                  Some(pin.prevout.hash.clone())
               } else {
                  println!("    tx.in[]: found. hash={}", pin.prevout.hash);
                  None
               }
            }
         }
      }).collect();

      if requests.is_empty() {
         println!("tx({}): all prevout are found. check signature.", tx);
         checks.push(txhash);
      } else {
         println!("tx({}): some prevout are not found. request={}", tx, request_missing);
         if request_missing {
            for h in requests.into_iter() {
               self.push(Box::new(protocol::GetDataMessage::new_tx(h)));
            }
         }
      }

      for h in checks.iter() {
         match self.check_signature(h) {
            Err(e) => println!("  sig fail: {:?}, tx={}", e, h),
            Ok(_) => println!("  sig ok: tx={}", h),
         }
      }
      Ok(())
   }

   fn check_signature(&mut self, txhash: &UInt256) -> Result<(), Error> {
      println!("check_signature(tx={})...", txhash);
      let ptxidx = try!(self.transactions.get(txhash).ok_or(GenericError::new("no transaction index found")));
      let ptx = match *ptxidx.get_transaction() {
         None => try!(Err(GenericError::new("no transaction found"))),
         Some(ref ptx) => ptx
      };

      let scripts:Vec<(&Script,&Script)> = ptx.ins.iter().filter_map(|pin| {
         match self.transactions.get(&pin.prevout.hash) {
            None => None,
            Some(ptxinidx) => {
               match *ptxinidx.get_transaction() {
                  None => None,
                  Some(ref ptxin) => {
                     let pubkey = &ptxin.outs[pin.prevout.n as usize].script_pubkey;
                     Some((pubkey, &pin.script_sig))
                  }
               }
            }
         }
      }).collect();
      if scripts.len() != ptx.ins.len() {
         println!("check_signature(tx={})...some tx are not found", txhash);
         return Ok(());
      }

      for (i, (pubkey, sig)) in scripts.into_iter().enumerate() {
         println!("check_signature(tx={}) for in[{}]...", txhash, i);
         let mut ip = Interpreter::new();
         try!(ip.eval(sig));
         try!(ip.eval(pubkey));
      }
      Ok(())
   }

   fn on_establish(&mut self) {
      let hashstrs = [
         "0000000000003a309aa6be96d21a5e31d221a55a86f7bfeb48ed9d5b26d4f55b",
      ];
      for hashstr in hashstrs.iter() {
         let h:UInt256 = UInt256::from_str(hashstr).unwrap();
         let pmsg = Box::new(protocol::GetDataMessage::new_block(h));
         self.push(pmsg);
      }
   }

   fn ioloop(&mut self) -> Result< (), Error > {
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
                  //println!("recv: {} {:x}", size, ByteBuf(&self.recv_buffer.as_mut_slice()[0..size]));
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

   fn on_recv(&mut self) -> Result<usize, Error> {
      let mut r:usize = 0;
      if self.recv_mode == 0 { //recv header
         if self.recv_buffer.readable_size() < protocol::MessageHeader::GetSerializableSize() {
            return Ok(0)
         };
         r = try!(self.recv_header.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
         /* {
            let h = &self.recv_header;
            let buf:&[u8] = self.recv_buffer.as_slice();
            println!("  {} {:x}", h, ByteBuf(buf));
         } */
         self.recv_mode = 1;

      } else if self.recv_mode == 1 { //recv body
         if self.recv_buffer.readable_size() < self.recv_header.size as usize {
            self.recv_buffer.ensure(self.recv_header.size as usize);
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

   fn on_recv_version(&mut self) -> Result<usize, Error> {
      let mut r = 0usize;
      let mut msg = protocol::VersionMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {}", &msg);
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
   fn on_recv_verack(&mut self) -> Result<usize, Error> {
      let mut r = 0usize;
      let mut msg = protocol::VerAckMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {}", &msg);
      self.recv_serialize_param.version = std::cmp::min(self.version, protocol::PROTOCOL_VERSION);
      self.on_establish();
      Ok(r)
   }
   fn on_recv_addr(&mut self) -> Result<usize, Error> {
      let mut r = 0usize;
      let mut msg = protocol::AddrMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {}", &msg);
      Ok(r)
   }
   fn on_recv_inv(&mut self) -> Result<usize, Error> {
      let mut r = 0usize;
      let mut msg = protocol::InvMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {}", &msg);
      for inv in msg.invs.into_iter() {
         match inv.invtype {
            protocol::InvType::Block => {
               // To release self borrowing, returs cloned value.
               if self.blocks.insert(&inv.hash).is_ok() {
                  // succeed in insert means the block is unknown. Request block.
                  self.push(Box::new(protocol::GetDataMessage::new_block(inv.hash.clone())));
               };
            }
            protocol::InvType::Tx => {
               self.push(Box::new(protocol::GetDataMessage::new_tx(inv.hash.clone())));
            }
            _ => {}
         }

      }
      Ok(r)
   }
   fn on_recv_getdata(&mut self) -> Result<usize, Error> {
      let mut r = 0usize;
      let mut msg = protocol::GetDataMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {}", &msg);
      Ok(r)
   }
   fn on_recv_merkleblock(&mut self) -> Result<usize, Error> {
      let mut r = 0usize;
      let mut msg = protocol::MerkleBlockMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {}", &msg);
      Ok(r)
   }
   fn on_recv_getblocks(&mut self) -> Result<usize, Error> {
      let mut r = 0usize;
      let mut msg = protocol::GetBlocksMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {}", &msg);
      Ok(r)
   }
   fn on_recv_getheaders(&mut self) -> Result<usize, Error> {
      let mut r = 0usize;
      let mut msg = protocol::GetHeadersMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {}", &msg);
      Ok(r)
   }
   fn on_recv_tx(&mut self) -> Result<usize, Error> {
      let mut r = 0usize;
      let mut msg = protocol::TxMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {}", &msg);
      try!(self.accept_transaction(&msg.tx, false));
      Ok(r)
   }
   fn on_recv_headers(&mut self) -> Result<usize, Error> {
      let mut r = 0usize;
      let mut msg = protocol::HeadersMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {}", &msg);
      for e in msg.headers.iter() {
         try!(self.accept_header(&e.header));
      }
      Ok(r)
   }
   fn on_recv_block(&mut self) -> Result<usize, Error> {
      let mut r = 0usize;
      let mut msg = protocol::BlockMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {}", &msg);
      try!(self.accept_block(&msg.block));
      Ok(r)
   }
   fn on_recv_getaddr(&mut self) -> Result<usize, Error> {
      let mut r = 0usize;
      let mut msg = protocol::GetAddrMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {}", &msg);
      Ok(r)
   }
   fn on_recv_mempool(&mut self) -> Result<usize, Error> {
      let mut r = 0usize;
      let mut msg = protocol::MemPoolMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {}", &msg);
      Ok(r)
   }
   fn on_recv_ping(&mut self) -> Result<usize, Error> {
      let mut r = 0usize;
      let mut msg = protocol::PingMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {}", &msg);
      self.push(Box::new(protocol::PongMessage::new(&msg)));
      Ok(r)
   }
   fn on_recv_pong(&mut self) -> Result<usize, Error> {
      let mut r = 0usize;
      let mut msg = protocol::PongMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {}", &msg);
      Ok(r)
   }
   fn on_recv_alert(&mut self) -> Result<usize, Error> {
      let mut r = 0usize;
      let mut msg = protocol::AlertMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {}", &msg);
      Ok(r)
   }
   fn on_recv_notfound(&mut self) -> Result<usize, Error> {
      let mut r = 0usize;
      let mut msg = protocol::NotFoundMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {}", &msg);
      Ok(r)
   }
   fn on_recv_filterload(&mut self) -> Result<usize, Error> {
      let mut r = 0usize;
      let mut msg = protocol::FilterLoadMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {}", &msg);
      Ok(r)
   }
   fn on_recv_filteradd(&mut self) -> Result<usize, Error> {
      let mut r = 0usize;
      let mut msg = protocol::FilterAddMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {}", &msg);
      Ok(r)
   }
   fn on_recv_filterclear(&mut self) -> Result<usize, Error> {
      let mut r = 0usize;
      let mut msg = protocol::FilterClearMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {}", &msg);
      Ok(r)
   }
   fn on_recv_reject(&mut self) -> Result<usize, Error> {
      let mut r = 0usize;
      let mut msg = protocol::RejectMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {}", &msg);
      Ok(r)
   }
   fn on_recv_sendheaders(&mut self) -> Result<usize, Error> {
      let mut r = 0usize;
      let mut msg = protocol::SendHeadersMessage::default();
      r += try!(msg.deserialize(&mut self.recv_buffer.as_slice(), &self.recv_serialize_param));
      println!("recv message: {}", &msg);
      Ok(r)
   }
   fn on_recv_unknown(&mut self) -> Result<usize, Error> {
      println!("unknown command: {:?}\n", self.recv_header.command);
      //try!(Err(serialize::SerializeError::new("unknown command")));
      Ok(0)
   }
}
