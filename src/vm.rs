use std::cmp::max;
use std::error;
use std::fmt::{Display, Formatter};
use crate::op;

/// An error that happens during execution of a program inside the VM.
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    EndOfProgram,
    UnknownOpcode(u8),
    StackUnderflow,
    StackOverflow,
    DivisionByZero,
    InvalidJump,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for RuntimeError {
}

/// The virtual machine itself.
///
/// Holds the state during execution of programs.
#[derive(Debug)]
pub struct VM {
    /// Value stack holding values during execution.
    pub stack: Vec<i64>,
    /// Program counter (PC),
    ///
    /// Points to instruction in bytecode that is to be executed next.
    pub pc: usize,
    /// Operation counter.
    ///
    /// Let's us know how "long" the execution took.
    pub op_cnt: usize,
    /// Activate verbose activity logging during execution?
    pub trace: bool,
    /// Maximal length the stack aver was, during execution.
    pub watermark: usize,
}

impl VM {
    pub fn new(stack_size: usize) -> VM{
        VM{
            stack: Vec::with_capacity(stack_size),
            pc: 0,
            op_cnt: 0,
            trace: false,
            watermark: 0,
        }
    }

    /// Tries and pops a value from value stack, respecting frame base.
    fn pop(&mut self) -> Result<i64, RuntimeError> {
        self.stack.pop().ok_or(RuntimeError::StackUnderflow)
    }

    /// Tries and pushes a value to value stack, respecting stack size.
    fn push(&mut self, v: i64) -> Result<(), RuntimeError> {
        if self.stack.len() < self.stack.capacity() {
            self.stack.push(v);
            self.watermark = max(self.watermark, self.stack.len());
            Ok(())
        } else {
            Err(RuntimeError::StackOverflow)
        }
    }

    /// Reads the next byte from the bytecode, increase programm counter, and return byte.
    fn fetch_u8(&mut self, pgm: &[u8]) -> Result<u8, RuntimeError> {
        if let Some(v) = pgm.get(self.pc) {
            self.pc += 1;
            Ok(*v)
        } else {
            Err(RuntimeError::EndOfProgram)
        }
    }

    /// Reads the next byte from the bytecode, increase program counter, and return byte.
    fn fetch_i8(&mut self, pgm: &[u8]) -> Result<i8, RuntimeError> {
        if let Some(v) = pgm.get(self.pc) {
            self.pc += 1;
            Ok(*v as i8)
        } else {
            Err(RuntimeError::EndOfProgram)
        }
    }

    /// Reads the next two bytes from the bytecode, increase program counter by two, and return as i16.
    fn fetch_i16(&mut self, pgm: &[u8]) -> Result<i16, RuntimeError> {
        let hi = self.fetch_i8(pgm)? as i16;
        let lo = self.fetch_u8(pgm)? as i16;
        Ok(hi << 8 | lo)
    }

    /// Executes a checked relative jump; Runtime error, if jump leaves program.
    fn relative_jump(&mut self, pgm: &[u8], delta: i16) -> Result<(), RuntimeError> {
        if self.trace {
            println!("  Jump from {} by {}", self.pc, delta);
        }
        if delta < 0 {
            let d = -delta as usize;
            if self.pc >= d {
                self.pc -= d;
                Ok(())
            } else {
                Err(RuntimeError::InvalidJump)
            }
        } else {
            let d = delta as usize;
            if self.pc + d < pgm.len() {
                self.pc += d;
                Ok(())
            } else {
                Err(RuntimeError::InvalidJump)
            }
        }
    }

    /// Executes a program (encoded in bytecode).
    pub fn run(&mut self, pgm: &[u8]) -> Result<(), RuntimeError> {
        // initialise the VM to be in a clean start state:
        self.stack.clear();
        self.pc = 0;
        self.op_cnt = 0;
        self.watermark = 0;

        // Loop going through the whole program, one instruction at a time.
        loop {
            // Log the vm's complete state, so we can follow what happens in console:
            if self.trace {
                println!("{:?}", self);
            }
            // Fetch next opcode from program (increases program counter):
            let opcode = self.fetch_u8(pgm)?;
            // We count the number of instructions we execute:
            self.op_cnt += 1;
            // If we are done, break loop and stop execution:
            if opcode == op::FIN {
                break;
            }
            // Execute the current instruction (with the opcode we loaded already):
            self.execute_op(pgm, opcode)?;
        }
        // Execution terminated. Output the final state of the VM:
        if self.trace {
            println!("Terminated!");
            println!("{:?}", self);
        }
        Ok(())
    }

    /// Executes an instruction, using the opcode passed.
    ///
    /// This might load more data from the program (opargs) and
    /// manipulate the stack (push, pop).
    fn execute_op(&mut self, pgm: &[u8], opcode: u8) -> Result<(), RuntimeError> {
        if self.trace {
            println!("Executing op 0x{:02x}", opcode);
        }
        match opcode {
            op::NOP => {
                // do nothing
                Ok(())
            },
            op::POP => {
                self.pop()?;
                Ok(())
            },
            op::PUSH_U8 => {
                let v = self.fetch_u8(pgm)?;
                self.push(v as i64)
            },
            op::ADD => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a + b)
            },
            op::SUB => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a - b)
            },
            op::MUL => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a * b)
            },
            op::DIV => {
                let b = self.pop()?;
                let a = self.pop()?;
                if b == 0 {
                    Err(RuntimeError::DivisionByZero)
                } else {
                    self.push(a / b)
                }
            },
            op::MOD => {
                let b = self.pop()?;
                let a = self.pop()?;
                if b == 0 {
                    Err(RuntimeError::DivisionByZero)
                } else {
                    self.push(a % b)
                }
            },
            op::GOTO => {
                let d = self.fetch_i16(pgm)?;
                self.relative_jump(pgm, d)
            }
            _ => {
                Err(RuntimeError::UnknownOpcode(opcode))
            }
        }
    }
}
