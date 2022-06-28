use crate::{op, Pgm};

/// An error that happens during execution of a program inside the VM.
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    EndOfProgram,
    UnknownOpcode(u8),
    StackUnderflow,
    StackOverflow,
    DivisionByZero,
    InvalidBranch,
}

/// The virtual machine itself.
///
/// Holds the state during execution of programs.
#[derive(Debug)]
pub struct VM {
    /// Value stack holding values during execution.
    stack: Vec<i64>,
    /// Program counter (PC),
    ///
    /// Points to instruction in bytecode that is to be executed next.
    pc: usize,
    /// Operation counter.
    ///
    /// Let's us know how "long" the execution took.
    op_cnt: usize,
}

impl VM {
    pub fn new(stack_size: usize) -> VM{
        VM{
            stack: Vec::with_capacity(stack_size),
            pc: 0,
            op_cnt: 0
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

    fn fetch_i8(&mut self, pgm: &Pgm) -> Result<i8, RuntimeError> {
        Ok(self.fetch_u8(pgm)? as i8)
    }

    fn fetch_i16(&mut self, pgm: &Pgm) -> Result<i16, RuntimeError> {
        let hi = self.fetch_i8(pgm)? as i16;
        let lo = self.fetch_u8(pgm)? as i16;
        Ok(hi << 8 | lo)
    }

    /// Executes a program (encoded in bytecode).
    pub fn run(&mut self, pgm: &Pgm) -> Result<(), RuntimeError> {
        // initialise the VM to be in a clean start state:
        self.stack.clear();
        self.pc = 0;
        self.op_cnt = 0;

        // Loop going through the whole program, one instruction at a time.
        loop {
            // Log the vm's complete state, so we can follow what happens in console:
            println!("{:?}", self);
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
        println!("Terminated!");
        println!("{:?}", self);
        Ok(())
    }

    fn branch(&mut self, pgm: &Pgm, offset: i16) -> Result<(), RuntimeError> {
        if offset < 0 {
            let off = -offset as usize;
            if off > self.pc {
                return Err(RuntimeError::InvalidBranch);
            }
            self.pc -= off;
        }
        if offset > 0 {
            let off = offset as usize;
            if pgm.text.len() - self.pc <= off {
                return Err(RuntimeError::InvalidBranch);
            }
            self.pc += off;
        }
        Ok(())
    }

    /// Executes an instruction, using the opcode passed.
    ///
    /// This might load more data from the program (opargs) and
    /// manipulate the stack (push, pop).
    fn execute_op(&mut self, pgm: &Pgm, opcode: u8) -> Result<(), RuntimeError> {
        println!("Executing op 0x{:02x}", opcode);
        match opcode {
            op::NOP => {
                println!("  NOP");
                // do nothing
                Ok(())
            },
            op::POP => {
                println!("  POP");
                let v = self.pop()?;
                println!("  dropping value {}", v);
                Ok(())
            },
            op::DUP => {
                println!("  DUP");
                let v = self.pop()?;
                self.push(v)?;
                self.push(v)?;
                Ok(())
            },
            op::PUSH_U8 => {
                println!("  PUSH_U8");
                let v = self.fetch_u8(pgm)?;
                println!("  value: {}", v);
                self.push(v as i64)
            },
            op::ADD => {
                println!("  ADD");
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a + b)
            },
            op::SUB => {
                println!("  SUB");
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a - b)
            },
            op::MUL => {
                println!("  MUL");
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a * b)
            },
            op::DIV => {
                println!("  DIV");
                let b = self.pop()?;
                let a = self.pop()?;
                if b == 0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                self.push(a / b)
            },
            op::MOD => {
                println!("  MOD");
                let b = self.pop()?;
                let a = self.pop()?;
                if b == 0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                self.push(a % b)
            },
            op::NEG => {
                println!("  NEG");
                let a = self.pop()?;
                self.push(-a)
            },
            op::IFLT => {
                let offset = self.fetch_i16(pgm)?;
                let v = self.pop()?;
                if v < 0 {
                    self.branch(pgm, offset)
                } else {
                    Ok(())
                }
            },
            _ => {
                Err(RuntimeError::UnknownOpcode(opcode))
            }
        }
    }
}
