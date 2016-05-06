#[allow(dead_code)]

// push value

macro_rules! defop( ($k:ident, $v:expr) => (#[allow(dead_code)] pub const $k:u8 = $v;) );

defop!(OP_0, 0x00);
defop!(OP_FALSE, OP_0);

defop!(OP_PUSHDATA1, 0x4c);
defop!(OP_PUSHDATA2, 0x4d);
defop!(OP_PUSHDATA4, 0x4e);
defop!(OP_1NEGATE, 0x4f);
defop!(OP_RESERVED, 0x50);
defop!(OP_1, 0x51);
defop!(OP_TRUE, OP_1);
defop!(OP_2, 0x52);
defop!(OP_3, 0x53);
defop!(OP_4, 0x54);
defop!(OP_5, 0x55);
defop!(OP_6, 0x56);
defop!(OP_7, 0x57);
defop!(OP_8, 0x58);
defop!(OP_9, 0x59);
defop!(OP_10, 0x5a);
defop!(OP_11, 0x5b);
defop!(OP_12, 0x5c);
defop!(OP_13, 0x5d);
defop!(OP_14, 0x5e);
defop!(OP_15, 0x5f);
defop!(OP_16, 0x60);

// control
defop!(OP_NOP, 0x61);
defop!(OP_VER, 0x62);
defop!(OP_IF, 0x63);
defop!(OP_NOTIF, 0x64);
defop!(OP_VERIF, 0x65);
defop!(OP_VERNOTIF, 0x66);
defop!(OP_ELSE, 0x67);
defop!(OP_ENDIF, 0x68);
defop!(OP_VERIFY, 0x69);
defop!(OP_RETURN, 0x6a);
// stack ops
defop!(OP_TOALTSTACK, 0x6b);
defop!(OP_FROMALTSTACK, 0x6c);
defop!(OP_2DROP, 0x6d);
defop!(OP_2DUP, 0x6e);
defop!(OP_3DUP, 0x6f);
defop!(OP_2OVER, 0x70);
defop!(OP_2ROT, 0x71);
defop!(OP_2SWAP, 0x72);
defop!(OP_IFDUP, 0x73);
defop!(OP_DEPTH, 0x74);
defop!(OP_DROP, 0x75);
defop!(OP_DUP, 0x76);
defop!(OP_NIP, 0x77);
defop!(OP_OVER, 0x78);
defop!(OP_PICK, 0x79);
defop!(OP_ROLL, 0x7a);
defop!(OP_ROT, 0x7b);
defop!(OP_SWAP, 0x7c);
defop!(OP_TUCK, 0x7d);

// splice ops
defop!(OP_CAT, 0x7e);
defop!(OP_SUBSTR, 0x7f);
defop!(OP_LEFT, 0x80);
defop!(OP_RIGHT, 0x81);
defop!(OP_SIZE, 0x82);
// bit logic
defop!(OP_INVERT, 0x83);
defop!(OP_AND, 0x84);
defop!(OP_OR, 0x85);
defop!(OP_XOR, 0x86);
defop!(OP_EQUAL, 0x87);
defop!(OP_EQUALVERIFY, 0x88);
defop!(OP_RESERVED1, 0x89);
defop!(OP_RESERVED2, 0x8a);

// numeric
defop!(OP_1ADD, 0x8b);
defop!(OP_1SUB, 0x8c);
defop!(OP_2MUL, 0x8d);
defop!(OP_2DIV, 0x8e);
defop!(OP_NEGATE, 0x8f);
defop!(OP_ABS, 0x90);
defop!(OP_NOT, 0x91);
defop!(OP_0NOTEQUAL, 0x92);

defop!(OP_ADD, 0x93);
defop!(OP_SUB, 0x94);
defop!(OP_MUL, 0x95);
defop!(OP_DIV, 0x96);
defop!(OP_MOD, 0x97);
defop!(OP_LSHIFT, 0x98);
defop!(OP_RSHIFT, 0x99);

defop!(OP_BOOLAND, 0x9a);
defop!(OP_BOOLOR, 0x9b);
defop!(OP_NUMEQUAL, 0x9c);
defop!(OP_NUMEQUALVERIFY, 0x9d);
defop!(OP_NUMNOTEQUAL, 0x9e);
defop!(OP_LESSTHAN, 0x9f);
defop!(OP_GREATERTHAN, 0xa0);
defop!(OP_LESSTHANOREQUAL, 0xa1);
defop!(OP_GREATERTHANOREQUAL, 0xa2);
defop!(OP_MIN, 0xa3);
defop!(OP_MAX, 0xa4);

defop!(OP_WITHIN, 0xa5);
// crypto
defop!(OP_RIPEMD160, 0xa6);
defop!(OP_SHA1, 0xa7);
defop!(OP_SHA256, 0xa8);
defop!(OP_HASH160, 0xa9);
defop!(OP_HASH256, 0xaa);
defop!(OP_CODESEPARATOR, 0xab);
defop!(OP_CHECKSIG, 0xac);
defop!(OP_CHECKSIGVERIFY, 0xad);
defop!(OP_CHECKMULTISIG, 0xae);
defop!(OP_CHECKMULTISIGVERIFY, 0xaf);

// expansion
defop!(OP_NOP1, 0xb0);
defop!(OP_CHECKLOCKTIMEVERIFY, 0xb1);
defop!(OP_NOP2, OP_CHECKLOCKTIMEVERIFY);
defop!(OP_NOP3, 0xb2);
defop!(OP_CHECKSEQUENCEVERIFY, OP_NOP3);
defop!(OP_NOP4, 0xb3);
defop!(OP_NOP5, 0xb4);
defop!(OP_NOP6, 0xb5);
defop!(OP_NOP7, 0xb6);
defop!(OP_NOP8, 0xb7);
defop!(OP_NOP9, 0xb8);
defop!(OP_NOP10, 0xb9);


// template matching params
defop!(OP_SMALLINTEGER, 0xfa);
defop!(OP_PUBKEYS, 0xfb);
defop!(OP_PUBKEYHASH, 0xfd);
defop!(OP_PUBKEY, 0xfe);

defop!(OP_INVALIDOPCODE, 0xff);

#[derive(Debug,Clone)]
pub struct OpCodeInfo {
   pub code: u8,
   pub name: &'static str,
}

pub const OPCODE_INFO:[OpCodeInfo; 256] = [
   // push value
   OpCodeInfo{ code:0x00, name:"OP_0" },

   OpCodeInfo{ code:0x01, name:"OP_0x01" },
   OpCodeInfo{ code:0x02, name:"OP_0x02" },
   OpCodeInfo{ code:0x03, name:"OP_0x03" },
   OpCodeInfo{ code:0x04, name:"OP_0x04" },
   OpCodeInfo{ code:0x05, name:"OP_0x05" },
   OpCodeInfo{ code:0x06, name:"OP_0x06" },
   OpCodeInfo{ code:0x07, name:"OP_0x07" },
   OpCodeInfo{ code:0x08, name:"OP_0x08" },
   OpCodeInfo{ code:0x09, name:"OP_0x09" },
   OpCodeInfo{ code:0x0a, name:"OP_0x0a" },
   OpCodeInfo{ code:0x0b, name:"OP_0x0b" },
   OpCodeInfo{ code:0x0c, name:"OP_0x0c" },
   OpCodeInfo{ code:0x0d, name:"OP_0x0d" },
   OpCodeInfo{ code:0x0e, name:"OP_0x0e" },
   OpCodeInfo{ code:0x0f, name:"OP_0x0f" },
   OpCodeInfo{ code:0x10, name:"OP_0x10" },
   OpCodeInfo{ code:0x11, name:"OP_0x11" },
   OpCodeInfo{ code:0x12, name:"OP_0x12" },
   OpCodeInfo{ code:0x13, name:"OP_0x13" },
   OpCodeInfo{ code:0x14, name:"OP_0x14" },
   OpCodeInfo{ code:0x15, name:"OP_0x15" },
   OpCodeInfo{ code:0x16, name:"OP_0x16" },
   OpCodeInfo{ code:0x17, name:"OP_0x17" },
   OpCodeInfo{ code:0x18, name:"OP_0x18" },
   OpCodeInfo{ code:0x19, name:"OP_0x19" },
   OpCodeInfo{ code:0x1a, name:"OP_0x1a" },
   OpCodeInfo{ code:0x1b, name:"OP_0x1b" },
   OpCodeInfo{ code:0x1c, name:"OP_0x1c" },
   OpCodeInfo{ code:0x1d, name:"OP_0x1d" },
   OpCodeInfo{ code:0x1e, name:"OP_0x1e" },
   OpCodeInfo{ code:0x1f, name:"OP_0x1f" },
   OpCodeInfo{ code:0x20, name:"OP_0x20" },
   OpCodeInfo{ code:0x21, name:"OP_0x21" },
   OpCodeInfo{ code:0x22, name:"OP_0x22" },
   OpCodeInfo{ code:0x23, name:"OP_0x23" },
   OpCodeInfo{ code:0x24, name:"OP_0x24" },
   OpCodeInfo{ code:0x25, name:"OP_0x25" },
   OpCodeInfo{ code:0x26, name:"OP_0x26" },
   OpCodeInfo{ code:0x27, name:"OP_0x27" },
   OpCodeInfo{ code:0x28, name:"OP_0x28" },
   OpCodeInfo{ code:0x29, name:"OP_0x29" },
   OpCodeInfo{ code:0x2a, name:"OP_0x2a" },
   OpCodeInfo{ code:0x2b, name:"OP_0x2b" },
   OpCodeInfo{ code:0x2c, name:"OP_0x2c" },
   OpCodeInfo{ code:0x2d, name:"OP_0x2d" },
   OpCodeInfo{ code:0x2e, name:"OP_0x2e" },
   OpCodeInfo{ code:0x2f, name:"OP_0x2f" },
   OpCodeInfo{ code:0x30, name:"OP_0x30" },
   OpCodeInfo{ code:0x31, name:"OP_0x31" },
   OpCodeInfo{ code:0x32, name:"OP_0x32" },
   OpCodeInfo{ code:0x33, name:"OP_0x33" },
   OpCodeInfo{ code:0x34, name:"OP_0x34" },
   OpCodeInfo{ code:0x35, name:"OP_0x35" },
   OpCodeInfo{ code:0x36, name:"OP_0x36" },
   OpCodeInfo{ code:0x37, name:"OP_0x37" },
   OpCodeInfo{ code:0x38, name:"OP_0x38" },
   OpCodeInfo{ code:0x39, name:"OP_0x39" },
   OpCodeInfo{ code:0x3a, name:"OP_0x3a" },
   OpCodeInfo{ code:0x3b, name:"OP_0x3b" },
   OpCodeInfo{ code:0x3c, name:"OP_0x3c" },
   OpCodeInfo{ code:0x3d, name:"OP_0x3d" },
   OpCodeInfo{ code:0x3e, name:"OP_0x3e" },
   OpCodeInfo{ code:0x3f, name:"OP_0x3f" },
   OpCodeInfo{ code:0x40, name:"OP_0x40" },
   OpCodeInfo{ code:0x41, name:"OP_0x41" },
   OpCodeInfo{ code:0x42, name:"OP_0x42" },
   OpCodeInfo{ code:0x43, name:"OP_0x43" },
   OpCodeInfo{ code:0x44, name:"OP_0x44" },
   OpCodeInfo{ code:0x45, name:"OP_0x45" },
   OpCodeInfo{ code:0x46, name:"OP_0x46" },
   OpCodeInfo{ code:0x47, name:"OP_0x47" },
   OpCodeInfo{ code:0x48, name:"OP_0x48" },
   OpCodeInfo{ code:0x49, name:"OP_0x49" },
   OpCodeInfo{ code:0x4a, name:"OP_0x4a" },
   OpCodeInfo{ code:0x4b, name:"OP_0x4b" },
   
   OpCodeInfo{ code:0x4c, name:"OP_PUSHDATA1" },
   OpCodeInfo{ code:0x4d, name:"OP_PUSHDATA2" },
   OpCodeInfo{ code:0x4e, name:"OP_PUSHDATA4" },
   OpCodeInfo{ code:0x4f, name:"OP_1NEGATE" },
   OpCodeInfo{ code:0x50, name:"OP_RESERVED" },
   OpCodeInfo{ code:0x51, name:"OP_1" },
   OpCodeInfo{ code:0x52, name:"OP_2" },
   OpCodeInfo{ code:0x53, name:"OP_3" },
   OpCodeInfo{ code:0x54, name:"OP_4" },
   OpCodeInfo{ code:0x55, name:"OP_5" },
   OpCodeInfo{ code:0x56, name:"OP_6" },
   OpCodeInfo{ code:0x57, name:"OP_7" },
   OpCodeInfo{ code:0x58, name:"OP_8" },
   OpCodeInfo{ code:0x59, name:"OP_9" },
   OpCodeInfo{ code:0x5a, name:"OP_10" },
   OpCodeInfo{ code:0x5b, name:"OP_11" },
   OpCodeInfo{ code:0x5c, name:"OP_12" },
   OpCodeInfo{ code:0x5d, name:"OP_13" },
   OpCodeInfo{ code:0x5e, name:"OP_14" },
   OpCodeInfo{ code:0x5f, name:"OP_15" },
   OpCodeInfo{ code:0x60, name:"OP_16" },

   // control
   OpCodeInfo{ code:0x61, name:"OP_NOP" },
   OpCodeInfo{ code:0x62, name:"OP_VER" },
   OpCodeInfo{ code:0x63, name:"OP_IF" },
   OpCodeInfo{ code:0x64, name:"OP_NOTIF" },
   OpCodeInfo{ code:0x65, name:"OP_VERIF" },
   OpCodeInfo{ code:0x66, name:"OP_VERNOTIF" },
   OpCodeInfo{ code:0x67, name:"OP_ELSE" },
   OpCodeInfo{ code:0x68, name:"OP_ENDIF" },
   OpCodeInfo{ code:0x69, name:"OP_VERIFY" },
   OpCodeInfo{ code:0x6a, name:"OP_RETURN" },
   // stack ops
   OpCodeInfo{ code:0x6b, name:"OP_TOALTSTACK" },
   OpCodeInfo{ code:0x6c, name:"OP_FROMALTSTACK" },
   OpCodeInfo{ code:0x6d, name:"OP_2DROP" },
   OpCodeInfo{ code:0x6e, name:"OP_2DUP" },
   OpCodeInfo{ code:0x6f, name:"OP_3DUP" },
   OpCodeInfo{ code:0x70, name:"OP_2OVER" },
   OpCodeInfo{ code:0x71, name:"OP_2ROT" },
   OpCodeInfo{ code:0x72, name:"OP_2SWAP" },
   OpCodeInfo{ code:0x73, name:"OP_IFDUP" },
   OpCodeInfo{ code:0x74, name:"OP_DEPTH" },
   OpCodeInfo{ code:0x75, name:"OP_DROP" },
   OpCodeInfo{ code:0x76, name:"OP_DUP" },
   OpCodeInfo{ code:0x77, name:"OP_NIP" },
   OpCodeInfo{ code:0x78, name:"OP_OVER" },
   OpCodeInfo{ code:0x79, name:"OP_PICK" },
   OpCodeInfo{ code:0x7a, name:"OP_ROLL" },
   OpCodeInfo{ code:0x7b, name:"OP_ROT" },
   OpCodeInfo{ code:0x7c, name:"OP_SWAP" },
   OpCodeInfo{ code:0x7d, name:"OP_TUCK" },

   // splice ops
   OpCodeInfo{ code:0x7e, name:"OP_CAT" },
   OpCodeInfo{ code:0x7f, name:"OP_SUBSTR" },
   OpCodeInfo{ code:0x80, name:"OP_LEFT" },
   OpCodeInfo{ code:0x81, name:"OP_RIGHT" },
   OpCodeInfo{ code:0x82, name:"OP_SIZE" },
   // bit logic
   OpCodeInfo{ code:0x83, name:"OP_INVERT" },
   OpCodeInfo{ code:0x84, name:"OP_AND" },
   OpCodeInfo{ code:0x85, name:"OP_OR" },
   OpCodeInfo{ code:0x86, name:"OP_XOR" },
   OpCodeInfo{ code:0x87, name:"OP_EQUAL" },
   OpCodeInfo{ code:0x88, name:"OP_EQUALVERIFY" },
   OpCodeInfo{ code:0x89, name:"OP_RESERVED1" },
   OpCodeInfo{ code:0x8a, name:"OP_RESERVED2" },

   // numeric
   OpCodeInfo{ code:0x8b, name:"OP_1ADD" },
   OpCodeInfo{ code:0x8c, name:"OP_1SUB" },
   OpCodeInfo{ code:0x8d, name:"OP_2MUL" },
   OpCodeInfo{ code:0x8e, name:"OP_2DIV" },
   OpCodeInfo{ code:0x8f, name:"OP_NEGATE" },
   OpCodeInfo{ code:0x90, name:"OP_ABS" },
   OpCodeInfo{ code:0x91, name:"OP_NOT" },
   OpCodeInfo{ code:0x92, name:"OP_0NOTEQUAL" },

   OpCodeInfo{ code:0x93, name:"OP_ADD" },
   OpCodeInfo{ code:0x94, name:"OP_SUB" },
   OpCodeInfo{ code:0x95, name:"OP_MUL" },
   OpCodeInfo{ code:0x96, name:"OP_DIV" },
   OpCodeInfo{ code:0x97, name:"OP_MOD" },
   OpCodeInfo{ code:0x98, name:"OP_LSHIFT" },
   OpCodeInfo{ code:0x99, name:"OP_RSHIFT" },

   OpCodeInfo{ code:0x9a, name:"OP_BOOLAND" },
   OpCodeInfo{ code:0x9b, name:"OP_BOOLOR" },
   OpCodeInfo{ code:0x9c, name:"OP_NUMEQUAL" },
   OpCodeInfo{ code:0x9d, name:"OP_NUMEQUALVERIFY" },
   OpCodeInfo{ code:0x9e, name:"OP_NUMNOTEQUAL" },
   OpCodeInfo{ code:0x9f, name:"OP_LESSTHAN" },
   OpCodeInfo{ code:0xa0, name:"OP_GREATERTHAN" },
   OpCodeInfo{ code:0xa1, name:"OP_LESSTHANOREQUAL" },
   OpCodeInfo{ code:0xa2, name:"OP_GREATERTHANOREQUAL" },
   OpCodeInfo{ code:0xa3, name:"OP_MIN" },
   OpCodeInfo{ code:0xa4, name:"OP_MAX" },

   OpCodeInfo{ code:0xa5, name:"OP_WITHIN" },
   // crypto
   OpCodeInfo{ code:0xa6, name:"OP_RIPEMD160" },
   OpCodeInfo{ code:0xa7, name:"OP_SHA1" },
   OpCodeInfo{ code:0xa8, name:"OP_SHA256" },
   OpCodeInfo{ code:0xa9, name:"OP_HASH160" },
   OpCodeInfo{ code:0xaa, name:"OP_HASH256" },
   OpCodeInfo{ code:0xab, name:"OP_CODESEPARATOR" },
   OpCodeInfo{ code:0xac, name:"OP_CHECKSIG" },
   OpCodeInfo{ code:0xad, name:"OP_CHECKSIGVERIFY" },
   OpCodeInfo{ code:0xae, name:"OP_CHECKMULTISIG" },
   OpCodeInfo{ code:0xaf, name:"OP_CHECKMULTISIGVERIFY" },

   // expansion
   OpCodeInfo{ code:0xb0, name:"OP_NOP1" },
   OpCodeInfo{ code:0xb1, name:"OP_CHECKLOCKTIMEVERIFY" },
   OpCodeInfo{ code:0xb2, name:"OP_CHECKSEQUENCEVERIFY" },
   OpCodeInfo{ code:0xb3, name:"OP_NOP4" },
   OpCodeInfo{ code:0xb4, name:"OP_NOP5" },
   OpCodeInfo{ code:0xb5, name:"OP_NOP6" },
   OpCodeInfo{ code:0xb6, name:"OP_NOP7" },
   OpCodeInfo{ code:0xb7, name:"OP_NOP8" },
   OpCodeInfo{ code:0xb8, name:"OP_NOP9" },
   OpCodeInfo{ code:0xb9, name:"OP_NOP10" },

   OpCodeInfo{ code:0xba, name:"OP_0xca" },
   OpCodeInfo{ code:0xbb, name:"OP_0xcb" },
   OpCodeInfo{ code:0xbc, name:"OP_0xcc" },
   OpCodeInfo{ code:0xbd, name:"OP_0xcd" },
   OpCodeInfo{ code:0xbe, name:"OP_0xce" },
   OpCodeInfo{ code:0xbf, name:"OP_0xcf" },
   OpCodeInfo{ code:0xc0, name:"OP_0xc0" },
   OpCodeInfo{ code:0xc1, name:"OP_0xc1" },
   OpCodeInfo{ code:0xc2, name:"OP_0xc2" },
   OpCodeInfo{ code:0xc3, name:"OP_0xc3" },
   OpCodeInfo{ code:0xc4, name:"OP_0xc4" },
   OpCodeInfo{ code:0xc5, name:"OP_0xc5" },
   OpCodeInfo{ code:0xc6, name:"OP_0xc6" },
   OpCodeInfo{ code:0xc7, name:"OP_0xc7" },
   OpCodeInfo{ code:0xc8, name:"OP_0xc8" },
   OpCodeInfo{ code:0xc9, name:"OP_0xc9" },
   OpCodeInfo{ code:0xca, name:"OP_0xca" },
   OpCodeInfo{ code:0xcb, name:"OP_0xcb" },
   OpCodeInfo{ code:0xcc, name:"OP_0xcc" },
   OpCodeInfo{ code:0xcd, name:"OP_0xcd" },
   OpCodeInfo{ code:0xce, name:"OP_0xce" },
   OpCodeInfo{ code:0xcf, name:"OP_0xcf" },
   OpCodeInfo{ code:0xd0, name:"OP_0xd0" },
   OpCodeInfo{ code:0xd1, name:"OP_0xd1" },
   OpCodeInfo{ code:0xd2, name:"OP_0xd2" },
   OpCodeInfo{ code:0xd3, name:"OP_0xd3" },
   OpCodeInfo{ code:0xd4, name:"OP_0xd4" },
   OpCodeInfo{ code:0xd5, name:"OP_0xd5" },
   OpCodeInfo{ code:0xd6, name:"OP_0xd6" },
   OpCodeInfo{ code:0xd7, name:"OP_0xd7" },
   OpCodeInfo{ code:0xd8, name:"OP_0xd8" },
   OpCodeInfo{ code:0xd9, name:"OP_0xd9" },
   OpCodeInfo{ code:0xda, name:"OP_0xda" },
   OpCodeInfo{ code:0xdb, name:"OP_0xdb" },
   OpCodeInfo{ code:0xdc, name:"OP_0xdc" },
   OpCodeInfo{ code:0xdd, name:"OP_0xdd" },
   OpCodeInfo{ code:0xde, name:"OP_0xde" },
   OpCodeInfo{ code:0xdf, name:"OP_0xdf" },
   OpCodeInfo{ code:0xe0, name:"OP_0xe0" },
   OpCodeInfo{ code:0xe1, name:"OP_0xe1" },
   OpCodeInfo{ code:0xe2, name:"OP_0xe2" },
   OpCodeInfo{ code:0xe3, name:"OP_0xe3" },
   OpCodeInfo{ code:0xe4, name:"OP_0xe4" },
   OpCodeInfo{ code:0xe5, name:"OP_0xe5" },
   OpCodeInfo{ code:0xe6, name:"OP_0xe6" },
   OpCodeInfo{ code:0xe7, name:"OP_0xe7" },
   OpCodeInfo{ code:0xe8, name:"OP_0xe8" },
   OpCodeInfo{ code:0xe9, name:"OP_0xe9" },
   OpCodeInfo{ code:0xea, name:"OP_0xea" },
   OpCodeInfo{ code:0xeb, name:"OP_0xeb" },
   OpCodeInfo{ code:0xec, name:"OP_0xec" },
   OpCodeInfo{ code:0xed, name:"OP_0xed" },
   OpCodeInfo{ code:0xee, name:"OP_0xee" },
   OpCodeInfo{ code:0xef, name:"OP_0xef" },
   OpCodeInfo{ code:0xf0, name:"OP_0xf0" },
   OpCodeInfo{ code:0xf1, name:"OP_0xf1" },
   OpCodeInfo{ code:0xf2, name:"OP_0xf2" },
   OpCodeInfo{ code:0xf3, name:"OP_0xf3" },
   OpCodeInfo{ code:0xf4, name:"OP_0xf4" },
   OpCodeInfo{ code:0xf5, name:"OP_0xf5" },
   OpCodeInfo{ code:0xf6, name:"OP_0xf6" },
   OpCodeInfo{ code:0xf7, name:"OP_0xf7" },
   OpCodeInfo{ code:0xf8, name:"OP_0xf8" },
   OpCodeInfo{ code:0xf9, name:"OP_0xf9" },
   
   // template matching params
   OpCodeInfo{ code:0xfa, name:"OP_SMALLINTEGER" },
   OpCodeInfo{ code:0xfb, name:"OP_PUBKEYS" },
   OpCodeInfo{ code:0xfc, name:"OP_0xfc" },
   OpCodeInfo{ code:0xfd, name:"OP_PUBKEYHASH" },
   OpCodeInfo{ code:0xfe, name:"OP_PUBKEY" },
   
   OpCodeInfo{ code:0xff, name:"OP_INVALIDOPCODE" },
];

/*
use std::ops::Index);
impl Index<u8> for OpCodeInfo {
   type Output = OpCodeInfo);
   fn index(&self, index: u8) -> &Self::Output { self.index(index as usize) }
}
*/

#[test]
fn test_infoarray() {
   assert_eq!(256, OPCODE_INFO.len());
}
