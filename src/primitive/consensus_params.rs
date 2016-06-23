use super::UInt256;

#[derive(Debug)]
pub struct ConsensusParams {
   pub hash_genesis_block: UInt256,
   pub subsidy_halving_interval: i32,

   /** Used to check majorities for block version upgrade */
   pub majority_enforce_block_upgrade: i32,
   pub majority_reject_block_outdated: i32,
   pub majority_window: i32,

   /** Block height and hash at which BIP34 becomes active */
   pub bip34_height: i32,
   pub bip34_hash  : UInt256,

   /** Proof of work parameters */
   pub pow_limit: UInt256,
   pub pow_allow_min_difficulty_blocks: bool,
   pub pow_no_retargeting: bool,
   pub pow_target_spacing: i64,
   pub pow_target_timespan: i64,
}

impl ConsensusParams {
//   pub fn getDifficultyAdjustmentInterval(&self) -> i64 {
//      self.pow_target_timespan / self.pow_target_spacing
//   }
}
