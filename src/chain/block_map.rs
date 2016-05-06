use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied,Vacant};
use primitive::{UInt256,BlockHeader};

#[derive(PartialEq)]
pub enum BlockIndexStatus {
   Init      = 0,
   GetHeader = 1,
}

pub struct BlockIndex {
   status: BlockIndexStatus,
   hash:   UInt256,
   header: Option<BlockHeader>,
   next:   Option<UInt256>,
}

impl BlockIndex {
   pub fn new(hash: &UInt256) -> BlockIndex {
      BlockIndex {
         status: BlockIndexStatus::Init,
         hash:   hash.clone(),
         header: None,
         next:   None,
      }
   }

   pub fn is_init(&self) -> bool { self.status == BlockIndexStatus::Init }

   pub fn get_hash(&self)   -> &UInt256 { &self.hash }
   pub fn get_header(&self) -> &Option<BlockHeader> { &self.header }
   pub fn get_next(&self)   -> &Option<UInt256> { &self.next }

   pub fn set_header(&mut self, header: BlockHeader) {
      self.header = Some(header);
      self.status = BlockIndexStatus::GetHeader;
   }
   pub fn set_next(&mut self, next: UInt256) {
      self.next = Some(next);
   }
}

#[derive(Default)]
pub struct BlockMap {
   map: HashMap< UInt256, BlockIndex >,
}
impl BlockMap {
   pub fn get(&self, hash: &UInt256) -> Option<&BlockIndex> {
      self.map.get(hash)
   }
   pub fn get_mut(&mut self, hash: &UInt256) -> Option<&mut BlockIndex> {
      self.map.get_mut(hash)
   }
   pub fn insert(&mut self, hash: &UInt256) -> Result<&mut BlockIndex, &mut BlockIndex> {
      match self.map.entry(hash.clone()) {
         Vacant(v)   => Ok(v.insert(BlockIndex::new(hash))),
         Occupied(o) => Err(o.into_mut())
      }
   }
}

