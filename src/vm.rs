use std::cmp::max;
use std::error;
use std::fmt::{Display, Formatter};
use crate::{op, Pgm};

/// An error that happens during execution of a program inside the VM.
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    EndOfProgram,
    UnknownOpcode(u8),
    StackUnderflow,
    StackOverflow,
    DivisionByZero,
    InvalidJump,
    InstructionLimitExceeded,
    InvalidVariable,
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
    /// Frame base register
    ///
    /// Pointer to the bottom of the stack for the current frame.
    pub fb: usize,
    /// Operation counter.
    ///
    /// Let's us know how "long" the execution took.
    pub op_cnt: usize,
    /// Activate verbose activity logging during execution?
    pub trace: bool,
    /// Maximal length the stack aver was, during execution.
    pub watermark: usize,
    /// Maximal number of instructions that are allowed for execution (0 for unlimited).
    pub instruction_limit: usize,
}

impl VM {
    pub fn new(stack_size: usize) -> VM{
        VM{
            stack: Vec::with_capacity(stack_size),
            pc: 0,
            fb: 0,
            op_cnt: 0,
            trace: false,
            watermark: 0,
            instruction_limit: 0,
        }
    }

    /// Tries and pops a value from value stack, respecting frame base.
    fn pop(&mut self) -> Result<i64, RuntimeError> {
        if self.stack.len() > self.fb {
            Ok(self.stack.pop().unwrap())
        } else {
            Err(RuntimeError::StackUnderflow)
        }
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
    fn fetch_u8(&mut self, pgm: &Pgm) -> Result<u8, RuntimeError> {
        if let Some(v) = pgm.text.get(self.pc) {
            self.pc += 1;
            Ok(*v)
        } else {
            Err(RuntimeError::EndOfProgram)
        }
    }

    /// Reads the next byte from the bytecode, increase program counter, and return byte.
    fn fetch_i8(&mut self, pgm: &Pgm) -> Result<i8, RuntimeError> {
        if let Some(v) = pgm.text.get(self.pc) {
            self.pc += 1;
            Ok(*v as i8)
        } else {
            Err(RuntimeError::EndOfProgram)
        }
    }

    /// Reads the next two bytes from the bytecode, increase program counter by two, and return as i16.
    fn fetch_i16(&mut self, pgm: &Pgm) -> Result<i16, RuntimeError> {
        let hi = self.fetch_i8(pgm)? as i16;
        let lo = self.fetch_u8(pgm)? as i16;
        Ok(hi << 8 | lo)
    }

    /// Executes a checked relative jump; Runtime error, if jump leaves program.
    fn relative_jump(&mut self, pgm: &Pgm, delta: i16) -> Result<(), RuntimeError> {
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
            if self.pc + d < pgm.text.len() {
                self.pc += d;
                Ok(())
            } else {
                Err(RuntimeError::InvalidJump)
            }
        }
    }

    /// Executes a program (encoded in bytecode).
    pub fn run(&mut self, pgm: &Pgm) -> Result<(), RuntimeError> {
        // initialise the VM to be in a clean start state:
        self.stack.clear();
        self.pc = 0;
        self.op_cnt = 0;
        self.watermark = 0;
        // create global variables in stack:
        for _ in 0..pgm.vars {
            self.push(0)?;
        }
        self.fb = pgm.vars as usize;

        // Loop going through the whole program, one instruction at a time.
        loop {
            // Log the vm's complete state, so we can follow what happens in console:
            if self.trace {
                println!("{:?}", self);
            }
            // Fetch next opcode from program (increases program counter):
            let opcode = self.fetch_u8(&pgm)?;
            // Limit execution by number of instructions that will be executed:
            if self.instruction_limit != 0 && self.op_cnt >= self.instruction_limit {
                return Err(RuntimeError::InstructionLimitExceeded);
            }
            // We count the number of instructions we execute:
            self.op_cnt += 1;
            // If we are done, break loop and stop execution:
            if opcode == op::FIN {
                break;
            }
            // Execute the current instruction (with the opcode we loaded already):
            self.execute_op(&pgm, opcode)?;
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
    fn execute_op(&mut self, pgm: &Pgm, opcode: u8) -> Result<(), RuntimeError> {
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
            op::DUP => {
                let v = self.pop()?;
                self.push(v)?;
                self.push(v)?;
                Ok(())
            },
            op::OUT => {
                let v = self.pop()?;
                println!("Out: {} (@{})", v, self.op_cnt);
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
            },
            op::IFEQ => {
                let d = self.fetch_i16(pgm)?;
                let v = self.pop()?;
                if v == 0 {
                    self.relative_jump(pgm, d)
                } else {
                    Ok(())
                }
            },
            op::IFNE => {
                let d = self.fetch_i16(pgm)?;
                let v = self.pop()?;
                if v != 0 {
                    self.relative_jump(pgm, d)
                } else {
                    Ok(())
                }
            },
            op::IFLT => {
                let d = self.fetch_i16(pgm)?;
                let v = self.pop()?;
                if v < 0 {
                    self.relative_jump(pgm, d)
                } else {
                    Ok(())
                }
            },
            op::IFLE => {
                let d = self.fetch_i16(pgm)?;
                let v = self.pop()?;
                if v <= 0 {
                    self.relative_jump(pgm, d)
                } else {
                    Ok(())
                }
            },
            op::IFGT => {
                let d = self.fetch_i16(pgm)?;
                let v = self.pop()?;
                if v > 0 {
                    self.relative_jump(pgm, d)
                } else {
                    Ok(())
                }
            },
            op::IFGE => {
                let d = self.fetch_i16(pgm)?;
                let v = self.pop()?;
                if v >= 0 {
                    self.relative_jump(pgm, d)
                } else {
                    Ok(())
                }
            },
            op::STORE => {
                let idx = self.fetch_u8(pgm)?;
                if idx >= pgm.vars {
                    Err(RuntimeError::InvalidVariable)
                } else {
                    let v = self.pop()?;
                    self.stack[idx as usize] = v;
                    Ok(())
                }
            },
            op::LOAD => {
                let idx = self.fetch_u8(pgm)?;
                if idx >= pgm.vars {
                    Err(RuntimeError::InvalidVariable)
                } else {
                    self.push(self.stack[idx as usize])?;
                    Ok(())
                }
            },
            _ => {
                Err(RuntimeError::UnknownOpcode(opcode))
            }
        }
    }
}
