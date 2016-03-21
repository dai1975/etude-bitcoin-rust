pub enum Chain {
   MAIN,
   TESTNET,
   REGTEST,
}

pub struct ChainParams {
   id : Chain,
   message_start : [u8;4],
}

let chain_main = ChainParams {
   id : Chain::MAIN,
   message_start : [ 0xf9, 0xbe, 0xb4, 0xd9 ],
};
let chain_test3 = ChainParams {
   id : Chain::TESTNET,
   message_start : [ 0x0b, 0x11, 0x09, 0x07 ],
};
let chain_reg = ChainParams {
   id : Chain::REGTEST,
   message_start : [ 0xfa, 0xbf, 0xb5, 0xda ],
};

impl ChainParams {
   fn new(id : &str) {
      match str {
         "main"    => chain_main,
         "test"    => chain_test3,
         "regtest" => chain_reg,
      }
   }
}

