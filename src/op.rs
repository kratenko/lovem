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
pub const MUL: u8 = 0x15;
pub const DIV: u8 = 0x16;
pub const MOD: u8 = 0x17;
pub const AND: u8 = 0x18;
pub const FIN: u8 = 0xff;
pub const IFEQ: u8 = 0x20;
pub const IFNE: u8 = 0x21;
pub const IFLT: u8 = 0x22;
pub const IFGT: u8 = 0x23;
pub const GOTO: u8 = 0x24;
pub const CALL: u8 = 0x25;
pub const RET: u8 = 0x26;

pub const DEV: u8 = 0x27;
pub const DEV2: u8 = 0x28;
pub const ECALL: u8 = 0x29;

pub const PUSH_RND: u8 = 0x30;
pub const PUSH_F32: u8 = 0x31;
pub const PUSH_F64: u8 = 0x32;
pub const FADD: u8 = 0x33;
pub const FSUB: u8 = 0x34;
pub const FMUL: u8 = 0x35;
pub const FDIV: u8 = 0x36;
pub const FCONST_0: u8 = 0x37;
pub const FCONST_1: u8 = 0x38;

pub const FPUSH: u8 = 0x39;
pub const FPOP: u8 = 0x3a;


/*
u8
i8
u16be

 */