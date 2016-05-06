use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied,Vacant};
use primitive::{UInt256,Transaction};

#[derive(PartialEq)]
pub enum TransactionIndexStatus {
   Init = 0,
   Get  = 1,
}

pub struct TransactionIndex {
   status: TransactionIndexStatus,
   hash:   UInt256,
   transaction: Option<Transaction>,
   waiters: Vec<UInt256>,
}

impl TransactionIndex {
   pub fn new(hash: &UInt256) -> TransactionIndex {
      TransactionIndex {
         status:      TransactionIndexStatus::Init,
         hash:        hash.clone(),
         transaction: None,
         waiters:     Vec::new(),
      }
   }

   pub fn is_init(&self) -> bool { self.status == TransactionIndexStatus::Init }

   pub fn get_hash(&self)        -> &UInt256 { &self.hash }
   pub fn get_transaction(&self) -> &Option<Transaction> { &self.transaction }

   pub fn set_transaction(&mut self, transaction: Transaction) {
      self.transaction = Some(transaction);
      self.status      = TransactionIndexStatus::Get;
   }
   pub fn add_waiter(&mut self, next: UInt256) {
      self.waiters.push(next);
   }
   pub fn move_waiters(&mut self, v:&mut Vec<UInt256>) {
      v.append(&mut self.waiters);
   }
}


#[derive(Default)]
pub struct TransactionMap {
   map: HashMap< UInt256, TransactionIndex >,
}
impl TransactionMap {
   pub fn get(&self, hash: &UInt256) -> Option<&TransactionIndex> {
      self.map.get(hash)
   }
   pub fn get_mut(&mut self, hash: &UInt256) -> Option<&mut TransactionIndex> {
      self.map.get_mut(hash)
   }
   pub fn insert(&mut self, hash: &UInt256) -> Result<&mut TransactionIndex, &mut TransactionIndex> {
      match self.map.entry(hash.clone()) {
         Vacant(v)   => Ok(v.insert(TransactionIndex::new(hash))),
         Occupied(o) => Err(o.into_mut())
      }
   }
}

