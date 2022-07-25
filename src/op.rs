//! Module holding the constants defining the opcodes for the VM.

/// opcode: Do nothing. No oparg.
///
/// pop: 0, push: 0
/// oparg: 0
pub const NOP: u8 = 0x00;

/// opcode: Pop value from stack and discard it.
///
/// pop: 1, push: 0
/// oparg: 0
pub const POP: u8 = 0x01;

/// opcode: Push immediate value to stack.
///
/// pop: 0, push: 1
/// oparg: 1B, u8 value to push
pub const PUSH_U8: u8 = 0x02;

/// opcode: Pop value from stack and push it back, twice.
///
/// pop: 1, push: 2
/// oparg: 0
pub const DUP: u8 = 0x03;

/// opcode: Pop value from stack and put in global variable.
///
/// pop: 1, push: 0
/// oparg: 1B, u8 index of variable
pub const STORE: u8 = 0x04;

/// opcode: Read value from global variable and push to stack.
///
/// pop: 0, push: 1
/// oparg: 1B, u8 index of variable
pub const LOAD: u8 = 0x05;

/// opcode: Debug. Output pop value and print it.
///
/// pop: 1, push: 0
/// oparg: 0
pub const OUT: u8 = 0x06;

/// opcode: Add top two values on stack.
///
/// pop: 2, push: 1
/// oparg: 0
pub const ADD: u8 = 0x10;

/// opcode: Subtract top two values on stack.
///
/// pop: 2, push: 1
/// oparg: 0
pub const SUB: u8 = 0x11;

/// opcode: Multiply top two values on stack.
///
/// pop: 2, push: 1
/// oparg: 0
pub const MUL: u8 = 0x12;

/// opcode: Divide top two values on stack.
///
/// pop: 2, push: 1
/// oparg: 0
pub const DIV: u8 = 0x13;

/// opcode: Calculate modulo of top two values on stack.
///
/// pop: 2, push: 1
/// oparg: 0
pub const MOD: u8 = 0x14;

/// opcode: Relative jump.
///
/// pop: 0, push: 0
/// oparg: 2B, i16 relative jump
pub const GOTO: u8 = 0x20;

/// opcode: Conditional relative jump (branch) on pop == zero.
///
/// pop: 1, push: 0
/// oparg: 2B, i16 relative jump
pub const IFEQ: u8 = 0x21;

/// opcode: Conditional relative jump (branch) on pop != zero.
///
/// pop: 1, push: 0
/// oparg: 2B, i16 relative jump
pub const IFNE: u8 = 0x22;

/// opcode: Conditional relative jump (branch) on pop < zero.
///
/// pop: 1, push: 0
/// oparg: 2B, i16 relative jump
pub const IFLT: u8 = 0x23;

/// opcode: Conditional relative jump (branch) on pop <= zero.
///
/// pop: 1, push: 0
/// oparg: 2B, i16 relative jump
pub const IFLE: u8 = 0x24;

/// opcode: Conditional relative jump (branch) on pop > zero.
///
/// pop: 1, push: 0
/// oparg: 2B, i16 relative jump
pub const IFGT: u8 = 0x25;

/// opcode: Conditional relative jump (branch) on pop >= zero.
///
/// pop: 1, push: 0
/// oparg: 2B, i16 relative jump
pub const IFGE: u8 = 0x26;

/// opcode: Save return position and jump.
///
/// pop: 0, push: 0
/// oparg: 2B, i16 relative jump
pub const CALL: u8 = 0x27;

/// opcode: Return from `CALL`.
///
/// pop: 0, push: 0
/// oparg: 0B
pub const RET: u8 = 0x28;

/// opcode: Terminate program.
///
/// pop: 0, push: 0
/// oparg: 0
pub const FIN: u8 = 0xff;
