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
/// opcode: Add top two values on stack.
///
/// pop: 2, push: 1
/// oparg: 0
pub const ADD: u8 = 0x10;
/// opcode: Terminate program.
///
/// pop: 0, push: 0
/// oparg: 0
pub const FIN: u8 = 0xff;