pub const NOP: u8 = 0x00;
pub const CONST_0: u8 = 0x01;
pub const CONST_1: u8 = 0x02;
pub const PUSH_U8: u8 = 0x03;
pub const PUSH_U16: u8 = 0x04;
pub const PUSH_U32: u8 = 0x05;
pub const PUSH_I64: u8 = 0x06;
pub const DUP: u8 = 0x07;
pub const POP: u8 = 0x08;
pub const INV: u8 = 0x10;
pub const ADD: u8 = 0x11;
pub const SUB: u8 = 0x12;
pub const ADD_1: u8 = 0x13;
pub const SUB_1: u8 = 0x14;
pub const AND: u8 = 0x18;
pub const FIN: u8 = 0xff;
pub const IFEQ: u8 = 0x20;
pub const IFNE: u8 = 0x21;
pub const IFLT: u8 = 0x22;
pub const IFGT: u8 = 0x23;
pub const GOTO: u8 = 0x24;

pub const PUSH_RND: u8 = 0x30;

/*
PUSH_U8
PUSH_U16
PUSH_U32
PUSH_I64
INV
push_i 17
 */