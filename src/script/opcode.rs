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

const CONTEXT_SOURCE:u32  = 0x01;
const CONTEXT_EXECUTE:u32 = 0x02;
const CONTEXT_NONE:u32    = 0x00;
const CONTEXT_ALL:u32     = CONTEXT_SOURCE | CONTEXT_EXECUTE;

#[derive(Debug,Clone)]
pub struct OpCodeInfo {
   pub code: u8,
   pub name: &'static str,
   pub validity: u32,
}

pub const OPCODE_INFO:[OpCodeInfo; 256] = [
   // push value
   OpCodeInfo{ code:0x00, name:"OP_0",    validity:CONTEXT_ALL },

   OpCodeInfo{ code:0x01, name:"OP_0x01", validity:CONTEXT_ALL, },
   OpCodeInfo{ code:0x02, name:"OP_0x02", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x03, name:"OP_0x03", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x04, name:"OP_0x04", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x05, name:"OP_0x05", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x06, name:"OP_0x06", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x07, name:"OP_0x07", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x08, name:"OP_0x08", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x09, name:"OP_0x09", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x0a, name:"OP_0x0a", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x0b, name:"OP_0x0b", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x0c, name:"OP_0x0c", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x0d, name:"OP_0x0d", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x0e, name:"OP_0x0e", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x0f, name:"OP_0x0f", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x10, name:"OP_0x10", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x11, name:"OP_0x11", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x12, name:"OP_0x12", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x13, name:"OP_0x13", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x14, name:"OP_0x14", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x15, name:"OP_0x15", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x16, name:"OP_0x16", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x17, name:"OP_0x17", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x18, name:"OP_0x18", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x19, name:"OP_0x19", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x1a, name:"OP_0x1a", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x1b, name:"OP_0x1b", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x1c, name:"OP_0x1c", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x1d, name:"OP_0x1d", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x1e, name:"OP_0x1e", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x1f, name:"OP_0x1f", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x20, name:"OP_0x20", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x21, name:"OP_0x21", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x22, name:"OP_0x22", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x23, name:"OP_0x23", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x24, name:"OP_0x24", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x25, name:"OP_0x25", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x26, name:"OP_0x26", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x27, name:"OP_0x27", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x28, name:"OP_0x28", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x29, name:"OP_0x29", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x2a, name:"OP_0x2a", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x2b, name:"OP_0x2b", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x2c, name:"OP_0x2c", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x2d, name:"OP_0x2d", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x2e, name:"OP_0x2e", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x2f, name:"OP_0x2f", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x30, name:"OP_0x30", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x31, name:"OP_0x31", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x32, name:"OP_0x32", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x33, name:"OP_0x33", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x34, name:"OP_0x34", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x35, name:"OP_0x35", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x36, name:"OP_0x36", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x37, name:"OP_0x37", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x38, name:"OP_0x38", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x39, name:"OP_0x39", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x3a, name:"OP_0x3a", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x3b, name:"OP_0x3b", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x3c, name:"OP_0x3c", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x3d, name:"OP_0x3d", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x3e, name:"OP_0x3e", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x3f, name:"OP_0x3f", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x40, name:"OP_0x40", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x41, name:"OP_0x41", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x42, name:"OP_0x42", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x43, name:"OP_0x43", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x44, name:"OP_0x44", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x45, name:"OP_0x45", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x46, name:"OP_0x46", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x47, name:"OP_0x47", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x48, name:"OP_0x48", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x49, name:"OP_0x49", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x4a, name:"OP_0x4a", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x4b, name:"OP_0x4b", validity:CONTEXT_ALL,  },
   
   OpCodeInfo{ code:0x4c, name:"OP_PUSHDATA1", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x4d, name:"OP_PUSHDATA2", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x4e, name:"OP_PUSHDATA4", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x4f, name:"OP_1NEGATE", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x50, name:"OP_RESERVED", validity:CONTEXT_EXECUTE, },
   OpCodeInfo{ code:0x51, name:"OP_1", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x52, name:"OP_2", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x53, name:"OP_3", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x54, name:"OP_4", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x55, name:"OP_5", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x56, name:"OP_6", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x57, name:"OP_7", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x58, name:"OP_8", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x59, name:"OP_9", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x5a, name:"OP_10", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x5b, name:"OP_11", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x5c, name:"OP_12", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x5d, name:"OP_13", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x5e, name:"OP_14", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x5f, name:"OP_15", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x60, name:"OP_16", validity:CONTEXT_ALL,  },

   // control
   OpCodeInfo{ code:0x61, name:"OP_NOP", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x62, name:"OP_VER", validity:CONTEXT_EXECUTE, },
   OpCodeInfo{ code:0x63, name:"OP_IF", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x64, name:"OP_NOTIF", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x65, name:"OP_VERIF", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0x66, name:"OP_VERNOTIF", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0x67, name:"OP_ELSE", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x68, name:"OP_ENDIF", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x69, name:"OP_VERIFY", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x6a, name:"OP_RETURN", validity:CONTEXT_ALL,  },
   // stack ops
   OpCodeInfo{ code:0x6b, name:"OP_TOALTSTACK", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x6c, name:"OP_FROMALTSTACK", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x6d, name:"OP_2DROP", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x6e, name:"OP_2DUP", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x6f, name:"OP_3DUP", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x70, name:"OP_2OVER", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x71, name:"OP_2ROT", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x72, name:"OP_2SWAP", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x73, name:"OP_IFDUP", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x74, name:"OP_DEPTH", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x75, name:"OP_DROP", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x76, name:"OP_DUP", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x77, name:"OP_NIP", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x78, name:"OP_OVER", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x79, name:"OP_PICK", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x7a, name:"OP_ROLL", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x7b, name:"OP_ROT", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x7c, name:"OP_SWAP", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x7d, name:"OP_TUCK", validity:CONTEXT_ALL,  },

   // splice ops
   OpCodeInfo{ code:0x7e, name:"OP_CAT", validity:CONTEXT_NONE,  },
   OpCodeInfo{ code:0x7f, name:"OP_SUBSTR", validity:CONTEXT_NONE,  },
   OpCodeInfo{ code:0x80, name:"OP_LEFT", validity:CONTEXT_NONE,  },
   OpCodeInfo{ code:0x81, name:"OP_RIGHT", validity:CONTEXT_NONE,  },
   OpCodeInfo{ code:0x82, name:"OP_SIZE", validity:CONTEXT_ALL,  },
   // bit logic
   OpCodeInfo{ code:0x83, name:"OP_INVERT", validity:CONTEXT_NONE,  },
   OpCodeInfo{ code:0x84, name:"OP_AND", validity:CONTEXT_NONE,  },
   OpCodeInfo{ code:0x85, name:"OP_OR", validity:CONTEXT_NONE,  },
   OpCodeInfo{ code:0x86, name:"OP_XOR", validity:CONTEXT_NONE,  },
   OpCodeInfo{ code:0x87, name:"OP_EQUAL", validity:CONTEXT_ALL, },
   OpCodeInfo{ code:0x88, name:"OP_EQUALVERIFY", validity:CONTEXT_ALL, },
   OpCodeInfo{ code:0x89, name:"OP_RESERVED1", validity:CONTEXT_EXECUTE, },
   OpCodeInfo{ code:0x8a, name:"OP_RESERVED2", validity:CONTEXT_EXECUTE, },

   // numeric
   OpCodeInfo{ code:0x8b, name:"OP_1ADD", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x8c, name:"OP_1SUB", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x8d, name:"OP_2MUL", validity:CONTEXT_NONE,  },
   OpCodeInfo{ code:0x8e, name:"OP_2DIV", validity:CONTEXT_NONE,  },
   OpCodeInfo{ code:0x8f, name:"OP_NEGATE", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x90, name:"OP_ABS", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x91, name:"OP_NOT", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x92, name:"OP_0NOTEQUAL", validity:CONTEXT_ALL,  },

   OpCodeInfo{ code:0x93, name:"OP_ADD", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x94, name:"OP_SUB", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x95, name:"OP_MUL", validity:CONTEXT_NONE,  },
   OpCodeInfo{ code:0x96, name:"OP_DIV", validity:CONTEXT_NONE,  },
   OpCodeInfo{ code:0x97, name:"OP_MOD", validity:CONTEXT_NONE,  },
   OpCodeInfo{ code:0x98, name:"OP_LSHIFT", validity:CONTEXT_NONE,  },
   OpCodeInfo{ code:0x99, name:"OP_RSHIFT", validity:CONTEXT_NONE,  },

   OpCodeInfo{ code:0x9a, name:"OP_BOOLAND", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x9b, name:"OP_BOOLOR", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x9c, name:"OP_NUMEQUAL", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x9d, name:"OP_NUMEQUALVERIFY", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x9e, name:"OP_NUMNOTEQUAL", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x9f, name:"OP_LESSTHAN", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xa0, name:"OP_GREATERTHAN", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xa1, name:"OP_LESSTHANOREQUAL", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xa2, name:"OP_GREATERTHANOREQUAL", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xa3, name:"OP_MIN", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xa4, name:"OP_MAX", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xa5, name:"OP_WITHIN", validity:CONTEXT_ALL,  },
   // crypto
   OpCodeInfo{ code:0xa6, name:"OP_RIPEMD160", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xa7, name:"OP_SHA1", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xa8, name:"OP_SHA256", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xa9, name:"OP_HASH160", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xaa, name:"OP_HASH256", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xab, name:"OP_CODESEPARATOR", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xac, name:"OP_CHECKSIG", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xad, name:"OP_CHECKSIGVERIFY", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xae, name:"OP_CHECKMULTISIG", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xaf, name:"OP_CHECKMULTISIGVERIFY", validity:CONTEXT_ALL,  },

   // expansion
   OpCodeInfo{ code:0xb0, name:"OP_NOP1", validity:CONTEXT_ALL, },
   OpCodeInfo{ code:0xb1, name:"OP_CHECKLOCKTIMEVERIFY", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xb2, name:"OP_CHECKSEQUENCEVERIFY", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xb3, name:"OP_NOP4", validity:CONTEXT_ALL, },
   OpCodeInfo{ code:0xb4, name:"OP_NOP5", validity:CONTEXT_ALL, },
   OpCodeInfo{ code:0xb5, name:"OP_NOP6", validity:CONTEXT_ALL, },
   OpCodeInfo{ code:0xb6, name:"OP_NOP7", validity:CONTEXT_ALL, },
   OpCodeInfo{ code:0xb7, name:"OP_NOP8", validity:CONTEXT_ALL, },
   OpCodeInfo{ code:0xb8, name:"OP_NOP9", validity:CONTEXT_ALL, },
   OpCodeInfo{ code:0xb9, name:"OP_NOP10", validity:CONTEXT_ALL, },

   OpCodeInfo{ code:0xba, name:"OP_0xca", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xbb, name:"OP_0xcb", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xbc, name:"OP_0xcc", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xbd, name:"OP_0xcd", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xbe, name:"OP_0xce", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xbf, name:"OP_0xcf", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xc0, name:"OP_0xc0", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xc1, name:"OP_0xc1", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xc2, name:"OP_0xc2", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xc3, name:"OP_0xc3", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xc4, name:"OP_0xc4", validity:CONTEXT_NONE, },

   OpCodeInfo{ code:0xc5, name:"OP_0xc5", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xc6, name:"OP_0xc6", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xc7, name:"OP_0xc7", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xc8, name:"OP_0xc8", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xc9, name:"OP_0xc9", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xca, name:"OP_0xca", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xcb, name:"OP_0xcb", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xcc, name:"OP_0xcc", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xcd, name:"OP_0xcd", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xce, name:"OP_0xce", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xcf, name:"OP_0xcf", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xd0, name:"OP_0xd0", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xd1, name:"OP_0xd1", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xd2, name:"OP_0xd2", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xd3, name:"OP_0xd3", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xd4, name:"OP_0xd4", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xd5, name:"OP_0xd5", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xd6, name:"OP_0xd6", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xd7, name:"OP_0xd7", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xd8, name:"OP_0xd8", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xd9, name:"OP_0xd9", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xda, name:"OP_0xda", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xdb, name:"OP_0xdb", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xdc, name:"OP_0xdc", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xdd, name:"OP_0xdd", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xde, name:"OP_0xde", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xdf, name:"OP_0xdf", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xe0, name:"OP_0xe0", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xe1, name:"OP_0xe1", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xe2, name:"OP_0xe2", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xe3, name:"OP_0xe3", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xe4, name:"OP_0xe4", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xe5, name:"OP_0xe5", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xe6, name:"OP_0xe6", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xe7, name:"OP_0xe7", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xe8, name:"OP_0xe8", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xe9, name:"OP_0xe9", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xea, name:"OP_0xea", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xeb, name:"OP_0xeb", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xec, name:"OP_0xec", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xed, name:"OP_0xed", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xee, name:"OP_0xee", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xef, name:"OP_0xef", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xf0, name:"OP_0xf0", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xf1, name:"OP_0xf1", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xf2, name:"OP_0xf2", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xf3, name:"OP_0xf3", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xf4, name:"OP_0xf4", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xf5, name:"OP_0xf5", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xf6, name:"OP_0xf6", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xf7, name:"OP_0xf7", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xf8, name:"OP_0xf8", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xf9, name:"OP_0xf9", validity:CONTEXT_NONE, },
   
   // template matching params
   OpCodeInfo{ code:0xfa, name:"OP_SMALLINTEGER", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xfb, name:"OP_PUBKEYS", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xfc, name:"OP_0xfc", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xfd, name:"OP_PUBKEYHASH", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xfe, name:"OP_PUBKEY", validity:CONTEXT_ALL,  },
   
   OpCodeInfo{ code:0xff, name:"OP_INVALIDOPCODE", validity:CONTEXT_ALL,  },
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
