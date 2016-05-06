extern crate crypto;
use self::crypto::digest::Digest;
use primitive::UInt256;

pub fn hash256d(bytes:&[u8]) -> UInt256 {
   let mut hasher = crypto::sha2::Sha256::new();
   let out = &mut [0u8; 32];
   hasher.input(bytes);
   hasher.result(out);
   hasher.reset();
   hasher.input(out);
   hasher.result(out);
   UInt256::new(out)
}

pub fn hash160(bytes:&[u8]) -> [u8;20] {
   let mut hasher1 = crypto::sha2::Sha256::new();

   let tmp = &mut [0u8; 32];
   hasher1.input(bytes);
   hasher1.result(tmp);

   let mut hasher2 = crypto::ripemd160::Ripemd160::new();
   hasher2.input(tmp);
   hasher2.result(tmp);

   let mut r:[u8;20] = [0u8;20];
   r.clone_from_slice(&tmp[0..20]);
   r
}
