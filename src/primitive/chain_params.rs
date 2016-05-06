use super::{UInt256,ConsensusParams};

pub enum Chain {
   MAIN,
   TESTNET,
   REGTEST,
}

pub struct ChainParams {
   pub id : Chain,
   pub message_start : [u8;4],
   pub consensus: ConsensusParams,
}

lazy_static! {
   #[allow(dead_code)]
   static ref CHAIN_MAIN:ChainParams = ChainParams{
      id : Chain::MAIN,
      message_start : [ 0xf9, 0xbe, 0xb4, 0xd9 ],
      consensus: ConsensusParams {
         hash_genesis_block: UInt256::from_str("000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f").unwrap(),
         subsidy_halving_interval: 210000,
         majority_enforce_block_upgrade: 750,
         majority_reject_block_outdated: 950,
         majority_window: 1000,
         bip34_height: 227931,
         bip34_hash: UInt256::from_str("000000000000024b89b42a942fe0d9fea3bb44ab7bd1b19115dd6a759c0808b8").unwrap(),
         pow_limit:  UInt256::from_str("00000000ffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap(),
         pow_target_timespan: 14 * 24 * 60 * 60, // two weeks
         pow_target_spacing:  10 * 60,
         pow_allow_min_difficulty_blocks: false,
         pow_no_retargeting: false,
      },
   };

   #[allow(dead_code)]
   static ref CHAIN_TEST3:ChainParams = ChainParams {
      id : Chain::TESTNET,
      message_start : [ 0x0b, 0x11, 0x09, 0x07 ],
      consensus: ConsensusParams {
         hash_genesis_block: UInt256::from_str("000000000933ea01ad0ee984209779baaec3ced90fa3f408719526f8d77f4943").unwrap(),
         subsidy_halving_interval: 210000,
         majority_enforce_block_upgrade: 51,
         majority_reject_block_outdated: 75,
         majority_window: 100,
         bip34_height: 21111,
         bip34_hash: UInt256::from_str("0000000023b3a96d3484e5abb3755c413e7d41500f8e2a5c3f0dd01299cd8ef8").unwrap(),
         pow_limit:  UInt256::from_str("00000000ffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap(),
         pow_target_timespan: 14 * 24 * 60 * 60, // two weeks
         pow_target_spacing:  10 * 60,
         pow_allow_min_difficulty_blocks: true,
         pow_no_retargeting: false,
      },
   };

   #[allow(dead_code)]
   static ref CHAIN_REG:ChainParams = ChainParams {
      id : Chain::REGTEST,
      message_start : [ 0xfa, 0xbf, 0xb5, 0xda ],
      consensus: ConsensusParams {
         hash_genesis_block: UInt256::from_str("0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206").unwrap(),
         subsidy_halving_interval: 150,
         majority_enforce_block_upgrade: 750,
         majority_reject_block_outdated: 950,
         majority_window: 1000,
         bip34_height: -1,
         bip34_hash: UInt256::default(),
         pow_limit:  UInt256::from_str("7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap(),
         pow_target_timespan: 14 * 24 * 60 * 60, // two weeks
         pow_target_spacing:  10 * 60,
         pow_allow_min_difficulty_blocks: true,
         pow_no_retargeting: true,
      },
   };
}

impl ChainParams {
   pub fn get(id : &str) -> &'static ChainParams {
      match id {
         "main"    => &*CHAIN_MAIN,
         "test"    => &*CHAIN_TEST3,
         "regtest" => &*CHAIN_REG,
         _         => &*CHAIN_REG,
      }
   }
}

