extern crate secp256k1;
use primitive::{GenericError, UInt256};

impl From< secp256k1::Error > for GenericError {
   fn from(err: secp256k1::Error) -> GenericError {
      match err {
         secp256k1::Error::IncapableContext   => GenericError::new("secp256k1/IncapableContext"),
         secp256k1::Error::IncorrectSignature => GenericError::new("secp256k1/IncorrectSignature"),
         secp256k1::Error::InvalidMessage     => GenericError::new("secp256k1/InvalidMessage"),
         secp256k1::Error::InvalidPublicKey   => GenericError::new("secp256k1/InvalidPublicKey"),
         secp256k1::Error::InvalidSignature   => GenericError::new("secp256k1/InvalidSignature"),
         secp256k1::Error::InvalidSecretKey   => GenericError::new("secp256k1/InvalidSecretKey"),
         secp256k1::Error::InvalidRecoveryId  => GenericError::new("secp256k1/InvalidRecoveryId"),
      }
   }
}

pub fn check_low_s(vch:&[u8]) -> Result<(), GenericError> {
   let ctx  = secp256k1::Secp256k1::new();
   let mut sign = try!(secp256k1::Signature::from_der_lax(&ctx, vch));
   sign.normalize_s(&ctx);
   Ok(())
}
pub fn verify(pk:&[u8], hash:&UInt256, sig:&[u8]) -> Result<(), GenericError> {
   let mut pubkey = PubKey::new();
   pubkey.set(pk);
   pubkey.verify(hash, sig)
}

pub struct PubKey {
   vch:[u8;65],
}
impl PubKey {
   pub fn new() -> PubKey {
      let mut r = PubKey { vch: [0u8;65] };
      r.invalidate();
      r
   }
   pub fn set(&mut self, vch:&[u8]) {
      if vch.len() == 0 {
         self.invalidate();
      } else {
         let l = PubKey::GetLen(vch[0]);
         if l != vch.len() {
            self.invalidate();
         } else {
            self.vch[0..l].copy_from_slice(vch);
         }
      }
   }
   pub fn is_valid(&self) -> bool {
      self.get_len() > 0
   }

   fn invalidate(&mut self) {
      self.vch[0] = 0xFFu8;
   }
   #[allow(non_snake_case)]
   fn GetLen(h:u8) -> usize {
      match h {
         2 => 33,
         3 => 33,
         4 => 65,
         6 => 65,
         7 => 65,
         _ => 0,
      }
   }
   fn get_len(&self) -> usize {
      PubKey::GetLen(self.vch[0])
   }
   fn get(&self) -> &[u8] {
      let len = self.get_len();
      &self.vch[0..len]
   }

   pub fn verify(&self, hash:&UInt256, sig:&[u8]) -> Result<(), GenericError> {
      if !self.is_valid() {
         return Err(GenericError::new("invalid pubkey"));
      }
      if sig.len() == 0 {
         return Err(GenericError::new("empty sig"));
      }

      let ctx  = secp256k1::Secp256k1::new();
      let message = try!(secp256k1::Message::from_slice(hash.as_slice()));
      let pubkey = try!(secp256k1::key::PublicKey::from_slice(&ctx, self.get()));
      let mut signature = try!(secp256k1::Signature::from_der_lax(&ctx, sig));
      signature.normalize_s(&ctx);
      let _ = try!(ctx.verify(&message, &signature, &pubkey));
      Ok(())
   }
}

