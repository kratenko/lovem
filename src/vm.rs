use rand::Rng;
use crate::{op, Pgm};

const STACK_SIZE: usize = 1000;

#[derive(Debug)]
pub enum RuntimeError {
    EndOfProgram,
    InvalidOperation,
    StackUnderflow,
    StackOverflow,
    InvalidBranch,
}

#[derive(Debug)]
pub struct VM {
    stack: Vec<i64>,
    pc: usize,
    op_cnt: usize,
}

impl VM {
    pub fn new() -> VM {
        VM{
            stack: Vec::with_capacity(STACK_SIZE),
            pc: 0,
            op_cnt: 0
        }
    }

    fn pop(&mut self) -> Result<i64, RuntimeError> {
        self.stack.pop().ok_or(RuntimeError::StackUnderflow)
    }

    fn push(&mut self, v: i64) -> Result<(), RuntimeError> {
        if self.stack.len() < self.stack.capacity() {
            self.stack.push(v);
            Ok(())
        } else {
            Err(RuntimeError::StackOverflow)
        }
    }

    fn load_u8(&mut self, pgm: &Pgm) -> Result<u8, RuntimeError> {
        if let Some(v) = pgm.text.get(self.pc) {
            self.pc += 1;
            Ok(*v)
        } else {
            Err(RuntimeError::EndOfProgram)
        }
    }

    fn load_u16(&mut self, pgm: &Pgm) -> Result<u16, RuntimeError> {
        let hi = self.load_u8(pgm)? as u16;
        let lo = self.load_u8(pgm)? as u16;
        Ok(hi << 8 | lo)
    }

    fn load_u32(&mut self, pgm: &Pgm) -> Result<u32, RuntimeError> {
        let mut v = 0u32;
        for _ in 0..4 {
            v = v << 8 | self.load_u8(pgm)? as u32;
        }
        Ok(v)
    }

    fn load_u64(&mut self, pgm: &Pgm) -> Result<u64, RuntimeError> {
        let mut v = 0u64;
        for _ in 0..8 {
            v = v << 8 | self.load_u8(pgm)? as u64;
        }
        Ok(v)
    }

    fn load_i8(&mut self, pgm: &Pgm) -> Result<i8, RuntimeError> {
        if let Some(v) = pgm.text.get(self.pc) {
            self.pc += 1;
            Ok(*v as i8)
        } else {
            Err(RuntimeError::EndOfProgram)
        }
    }

    fn load_i16(&mut self, pgm: &Pgm) -> Result<i16, RuntimeError> {
        let hi = self.load_i8(pgm)? as i16;
        let lo = self.load_u8(pgm)? as i16;
        return Ok(hi << 8 | lo);
    }

    fn load_i32(&mut self, pgm: &Pgm) -> Result<i32, RuntimeError> {
        let mut v = self.load_i8(pgm)? as i32;
        for _ in 1..4 {
            v = v << 8 | self.load_u8(pgm)? as i32;
        }
        Ok(v)
    }

    fn load_i64(&mut self, pgm: &Pgm) -> Result<i64, RuntimeError> {
        let mut v = self.load_i8(pgm)? as i64;
        for _ in 1..8 {
            v = v << 8 | self.load_u8(pgm)? as i64;
        }
        Ok(v)
    }

    pub fn run(&mut self, pgm: &Pgm) -> Result<(), RuntimeError> {
        self.stack.clear();
        self.pc = 0;
        self.op_cnt = 0;

        loop {
            println!("{:?}", self);
            let opcode = self.load_u8(pgm)?;
            self.op_cnt += 1;
            if opcode == op::FIN {
                return Ok(());
            }
            self.execute_op(pgm, opcode)?;
        }
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

    fn execute_op(&mut self, pgm: &Pgm, opcode: u8) -> Result<(), RuntimeError> {
        println!("Executing op 0x{:02x}", opcode);
        match opcode {
            op::NOP => {
                // Not doing anything...
            }
            op::CONST_0 => self.push(0)?,
            op::CONST_1 => self.push(1)?,
            op::PUSH_U8 => {
                let v = self.load_u8(pgm)?;
                self.push(v as i64)?;
            }
            op::PUSH_U16 => {
                let v = self.load_u16(pgm)?;
                self.push(v as i64)?;
            }
            op::PUSH_U32 => {
                let v = self.load_u32(pgm)?;
                self.push(v as i64)?;
            }
            op::PUSH_I64 => {
                let v = self.load_i64(pgm)?;
                self.push(v)?;
            }
            op::ADD => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a + b)?;
            }
            op::SUB => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a - b)?;
            }
            op::ADD_1 => {
                let a = self.pop()?;
                self.push(a + 1)?;
            }
            op::SUB_1 => {
                let a = self.pop()?;
                self.push(a - 1)?;
            }
            op::INV => {
                let a = -self.pop()?;
                self.push(a)?;
            }
            op::AND => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a & b)?;
            }
            op::IFEQ => {
                let offset = self.load_i16(pgm)?;
                let v = self.pop()?;
                if v == 0 {
                    self.branch(pgm, offset)?;
                }
            }
            op::IFNE => {
                let offset = self.load_i16(pgm)?;
                let v = self.pop()?;
                if v != 0 {
                    self.branch(pgm, offset)?;
                }
            }
            op::IFLT => {
                let offset = self.load_i16(pgm)?;
                let v = self.pop()?;
                if v < 0 {
                    self.branch(pgm, offset)?;
                }
            }
            op::IFGT => {
                let offset = self.load_i16(pgm)?;
                let v = self.pop()?;
                if v > 0 {
                    self.branch(pgm, offset)?;
                }
            }
            op::GOTO => {
                let offset = self.load_i16(pgm)?;
                self.branch(pgm, offset)?;
            }
            op::PUSH_RND => {
                self.push(rand::thread_rng().gen())?;
            }
            op::DUP => {
                let v = self.pop()?;
                self.push(v)?;
                self.push(v)?;
            }
            op::POP => {
                self.pop()?;
            }
            _ => {
                return Err(RuntimeError::InvalidOperation);
            }
        }
        Ok(())
    }
}
