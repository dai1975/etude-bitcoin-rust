
#[allow(dead_code)] pub const SIGHASH_ALL:u8          = 1u8;
#[allow(dead_code)] pub const SIGHASH_NONE:u8         = 2u8;
#[allow(dead_code)] pub const SIGHASH_SINGLE:u8       = 3u8;
#[allow(dead_code)] pub const SIGHASH_ANYONECANPAY:u8 = 0x80u8;

#[allow(dead_code)] pub const SCRIPT_VERIFY_NONE:u32                       = 0u32;
#[allow(dead_code)] pub const SCRIPT_VERIFY_P2SH:u32                       = (1u32 << 0);
#[allow(dead_code)] pub const SCRIPT_VERIFY_STRICTENC:u32                  = (1u32 << 1);
#[allow(dead_code)] pub const SCRIPT_VERIFY_DERSIG:u32                     = (1u32 << 2);
#[allow(dead_code)] pub const SCRIPT_VERIFY_LOW_S:u32                      = (1u32 << 3);
#[allow(dead_code)] pub const SCRIPT_VERIFY_NULLDUMMY:u32                  = (1u32 << 4);
#[allow(dead_code)] pub const SCRIPT_VERIFY_SIGPUSHONLY:u32                = (1u32 << 5);
#[allow(dead_code)] pub const SCRIPT_VERIFY_MINIMALDATA:u32                = (1u32 << 6);
#[allow(dead_code)] pub const SCRIPT_VERIFY_DISCOURAGE_UPGRADABLE_NOPS:u32 = (1u32 << 7);
#[allow(dead_code)] pub const SCRIPT_VERIFY_CLEANSTACK:u32                 = (1u32 << 8);
#[allow(dead_code)] pub const SCRIPT_VERIFY_CHECKLOCKTIMEVERIFY:u32        = (1u32 << 9);
#[allow(dead_code)] pub const SCRIPT_VERIFY_CHECKSEQUENCEVERIFY:u32        = (1u32 << 10);


#[allow(dead_code)] pub const MANDATORY_SCRIPT_VERIFY_FLAGS:u32
   = SCRIPT_VERIFY_P2SH;

#[allow(dead_code)] pub const STANDARD_SCRIPT_VERIFY_FLAGS:u32
   = MANDATORY_SCRIPT_VERIFY_FLAGS
   | SCRIPT_VERIFY_DERSIG
   | SCRIPT_VERIFY_STRICTENC
   | SCRIPT_VERIFY_MINIMALDATA
   | SCRIPT_VERIFY_NULLDUMMY
   | SCRIPT_VERIFY_DISCOURAGE_UPGRADABLE_NOPS
   | SCRIPT_VERIFY_CLEANSTACK
   | SCRIPT_VERIFY_CHECKLOCKTIMEVERIFY
   | SCRIPT_VERIFY_CHECKSEQUENCEVERIFY
   | SCRIPT_VERIFY_LOW_S
   ;

#[allow(dead_code)] pub const STANDARD_NOT_MANDATORY_VERIFY_FLAGS:u32
   = STANDARD_SCRIPT_VERIFY_FLAGS & !MANDATORY_SCRIPT_VERIFY_FLAGS;


