use std::collections::HashMap;
use std::fmt;
use rand::Rng;
use crate::{op, Pgm};

const STACK_SIZE: usize = 1000;

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    EndOfProgram,
    InvalidOperation(u8),
    StackUnderflow,
    StackOverflow,
    InvalidBranch,
    DivisionByZero,
    CallError,
    UnknownFunction(String),
    InvalidGlobalVariable,
}

pub struct Frame {

}

pub struct VM {
    /// The main value stack
    stack: Vec<i64>,
    /// Frame Base Register
    ///
    /// Pointer to the bottom of the stack for the current frame.
    fb: usize,
    /// Program Counter
    ///
    /// Pointer to the instruction inside program text that is to
    /// be executed next.
    pc: usize,
    /// Number of operations executed.
    op_cnt: usize,
    fstack: Vec<usize>,
    funs: HashMap<String, fn(&mut [i64]) -> Result<(), RuntimeError>>,
}

impl fmt::Debug for VM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VM {{ pc: {}, op_cnt: {}, fstack: {:?}, fb: {}, stack: {:?} }}", self.pc, self.op_cnt, &self.fstack, &self.fb, &self.stack)
    }
}


fn bogus(_: &mut [i64]) -> Result<(), RuntimeError> {
    println!("bogus");
    Ok(())
}

impl VM {
    pub fn new() -> VM {
        let mut vm = VM{
            stack: Vec::with_capacity(STACK_SIZE),
            fb: 0,
            pc: 0,
            op_cnt: 0,
            fstack: vec![],
            funs: Default::default()
        };
        vm.funs.insert(String::from("bogus"), bogus);
        vm.funs.insert(String::from("sayf"), |v: &mut [i64]| -> Result<(), RuntimeError> {
            let i = v.get(0).ok_or(RuntimeError::CallError)?;
            let f = f64::from_be_bytes(i.to_be_bytes());
            println!("f: {}", f);
            Ok(())
        });
        vm.funs.insert(String::from("sin"), |v: &mut [i64]| -> Result<(), RuntimeError> {
            let i = v.get(0).ok_or(RuntimeError::CallError)?;
            let f = f64::from_be_bytes(i.to_be_bytes()).sin();
            v[0] = i64::from_be_bytes(f.to_be_bytes());
            Ok(())
        });
        vm.funs.insert(String::from("pi"), |v: &mut [i64]| -> Result<(), RuntimeError> {
            if v.len() == 1 {
                v[0] = i64::from_be_bytes(std::f64::consts::PI.to_be_bytes());
                Ok(())
            } else {
                Err(RuntimeError::CallError)
            }
        });
        vm
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
            Ok(())
        } else {
            Err(RuntimeError::StackOverflow)
        }
    }

    /// Pops a value from value stack and returns it as a float64.
    fn pop_f64(&mut self) -> Result<f64, RuntimeError> {
        let i = self.pop()?;
        Ok(f64::from_be_bytes(i.to_be_bytes()))
    }

    /// Push a float64, converted to a value, to value stack.
    fn push_f64(&mut self, v: f64) -> Result<(), RuntimeError> {
        self.push(i64::from_be_bytes(v.to_be_bytes()))
    }

    /// Load a single byte from program text as u8.
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

    fn load_f32(&mut self, pgm: &Pgm) -> Result<f32, RuntimeError> {
        let mut bb: [u8;4] = [0; 4];
        for n in 0..4 {
            bb[n] = self.load_u8(pgm)?;
        }
        Ok(f32::from_be_bytes(bb))
    }

    pub fn run(&mut self, pgm: &Pgm, label: &str) -> Result<(), RuntimeError> {
        self.stack.clear();
        for _ in 0..pgm.vars {
            self.stack.push(0);
            self.fb += 1
        }
        self.op_cnt = 0;

        self.pc = if label == "" {
            0
        } else {
            *pgm.labels.get(label).ok_or(RuntimeError::CallError)?
        };

        for e in &pgm.ext {
            if !self.funs.contains_key(e) {
                return Err(RuntimeError::UnknownFunction(String::from(e)));
            }
        }

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
            op::FCONST_0 => self.push_f64(0f64)?,
            op::FCONST_1 => self.push_f64(1f64)?,
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
            op::MUL => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a * b)?;
            }
            op::DIV => {
                let b = self.pop()?;
                if b == 0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                let a = self.pop()?;
                self.push(a / b)?;
            }
            op::MOD => {
                let b = self.pop()?;
                if b == 0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                let a = self.pop()?;
                self.push(a % b)?;
            }
            op::FADD => {
                let b = self.pop_f64()?;
                let a = self.pop_f64()?;
                self.push_f64(a + b)?;
            }
            op::FSUB => {
                let b = self.pop_f64()?;
                let a = self.pop_f64()?;
                self.push_f64(a - b)?;
            }
            op::FMUL => {
                let b = self.pop_f64()?;
                let a = self.pop_f64()?;
                self.push_f64(a * b)?;
            }
            op::FDIV => {
                let b = self.pop_f64()?;
                let a = self.pop_f64()?;
                self.push_f64(a / b)?;
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
            op::PUSH_F32 => {
                let f = self.load_f32(pgm)? as f64;
                self.push_f64(f)?;
            }
            op::CALL => {
                let offset = self.load_i16(pgm)?;
                self.fstack.push(self.pc);
                self.branch(pgm, offset)?;
            }
            op::RET => {
                let d = self.fstack.pop().ok_or(RuntimeError::StackUnderflow)?;
                self.pc = d;
            }
            op::STORE_G => {
                let vn = self.load_u8(pgm)?;
                if vn >= pgm.vars {
                    return Err(RuntimeError::InvalidGlobalVariable);
                }
                let v = self.pop()?;
                self.stack[vn as usize] = v;
            }
            op::LOAD_G => {
                let vn = self.load_u8(pgm)?;
                if vn >= pgm.vars {
                    return Err(RuntimeError::InvalidGlobalVariable);
                }
                let v = self.stack[vn as usize];
                self.push(v)?;
            }
            op::ECALL => {
                let n = self.load_u8(pgm)?;
                let ename = pgm.ext.get(n as usize).ok_or(RuntimeError::CallError)?;
                let fu = self.funs.get(ename).ok_or(RuntimeError::CallError)?.clone();
                let n = self.pop()?;
                if n < 0 || n > 255 {
                    return Err(RuntimeError::CallError);
                }
                let n = n as usize;
                if self.stack.len() < n {
                    return Err(RuntimeError::StackUnderflow);
                }
                let start = self.stack.len() - n;
                let a = &mut self.stack[start..];
                fu(a)?;
            }
            op::DEV => {
                let fu = self.funs.get("sayf").unwrap().clone();
                let n = self.pop()?;
                if n < 0 || n > 255 {
                    return Err(RuntimeError::CallError);
                }
                let n = n as usize;
                if self.stack.len() < n {
                    return Err(RuntimeError::StackUnderflow);
                }
                let start = self.stack.len() - n;
                let a = &mut self.stack[start..];
                fu(a)?;
            }
            op::DEV2 => {
                let fu = self.funs.get("sin").unwrap().clone();
                let n = self.pop()?;
                if n < 0 || n > 255 {
                    return Err(RuntimeError::CallError);
                }
                let n = n as usize;
                if self.stack.len() < n {
                    return Err(RuntimeError::StackUnderflow);
                }
                let start = self.stack.len() - n;
                let a = &mut self.stack[start..];
                fu(a)?;
            }
            _ => {
                return Err(RuntimeError::InvalidOperation(opcode));
            }
        }
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use crate::{Pgm, VM};
    use crate::vm::RuntimeError;

    fn vm_with_stack(stack: Vec<i64>) -> VM {
        VM{
            stack,
            fb: 0,
            pc: 0,
            op_cnt: 0,
            fstack: vec![],
            funs: Default::default()
        }
    }

    fn pgm_with_text(text: Vec<u8>) -> Pgm {
        Pgm{
            ext: vec![],
            text,
            labels: Default::default(),
            vars: 0
        }
    }

    #[test]
    fn pop_stack_empty() {
        let mut vm = vm_with_stack(vec![]);
        assert_eq!(vm.stack.len(), 0);
        assert_eq!(vm.pop(), Err(RuntimeError::StackUnderflow));
        assert_eq!(vm.stack.len(), 0);
    }

    #[test]
    fn pop_stack_full() {
        let mut vm = vm_with_stack(vec![1, 2, -1, 0xffffffffff, -99999999999999999, 0, 123456]);
        assert_eq!(vm.stack.len(), 7);
        assert_eq!(vm.pop(), Ok(123456));
        assert_eq!(vm.stack.len(), 6);
        assert_eq!(vm.pop(), Ok(0));
        assert_eq!(vm.stack.len(), 5);
        assert_eq!(vm.pop(), Ok(-99999999999999999));
        assert_eq!(vm.stack.len(), 4);
        assert_eq!(vm.pop(), Ok(0xffffffffff));
        assert_eq!(vm.stack.len(), 3);
        assert_eq!(vm.pop(), Ok(-1));
        assert_eq!(vm.stack.len(), 2);
        assert_eq!(vm.pop(), Ok(2));
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.pop(), Ok(1));
        assert_eq!(vm.stack.len(), 0);
        assert_eq!(vm.pop(), Err(RuntimeError::StackUnderflow));
        assert_eq!(vm.stack.len(), 0);
        assert_eq!(vm.pop(), Err(RuntimeError::StackUnderflow));
        assert_eq!(vm.stack.len(), 0);
    }

    #[test]
    fn pop_stack_fb() {
        let mut vm = vm_with_stack(vec![1, 2, 3]);
        vm.fb = 2;
        assert_eq!(vm.pop(), Ok(3));
        assert_eq!(vm.stack.len(), 2);
        assert_eq!(vm.pop(), Err(RuntimeError::StackUnderflow));
        assert_eq!(vm.stack.len(), 2);
        assert_eq!(vm.pop(), Err(RuntimeError::StackUnderflow));
        assert_eq!(vm.stack.len(), 2);
    }

    #[test]
    fn pop_stack_f64_empty() {
        let mut vm = vm_with_stack(vec![]);
        assert_eq!(vm.pop_f64(), Err(RuntimeError::StackUnderflow));
        assert_eq!(vm.stack.len(), 0);
    }

    #[test]
    fn pop_stack_f64_full() -> Result<(), RuntimeError> {
        // make sure all kinda edge cases get through:
        let mut vm = vm_with_stack(vec![
                        i64::from_be_bytes((-1f64).to_be_bytes()),
                        i64::from_be_bytes(0f64.to_be_bytes()),
                        i64::from_be_bytes(f64::NAN.to_be_bytes()),
                        i64::from_be_bytes(f64::NEG_INFINITY.to_be_bytes()),
                        i64::from_be_bytes(f64::INFINITY.to_be_bytes()),
                        i64::from_be_bytes(f64::MIN.to_be_bytes()),
                        i64::from_be_bytes(f64::MIN_POSITIVE.to_be_bytes()),
                        i64::from_be_bytes(f64::MAX.to_be_bytes()),
                        i64::from_be_bytes(1337f64.to_be_bytes()),
        ]);
        assert_eq!(vm.pop_f64(), Ok(1337f64));
        assert_eq!(vm.pop_f64(), Ok(f64::MAX));
        assert_eq!(vm.pop_f64(), Ok(f64::MIN_POSITIVE));
        assert_eq!(vm.pop_f64(), Ok(f64::MIN));
        assert_eq!(vm.pop_f64(), Ok(f64::INFINITY));
        assert_eq!(vm.pop_f64(), Ok(f64::NEG_INFINITY));
        // cannot compare Ok(NaN) to Ok(NaN) directly...
        assert!(vm.pop_f64()?.is_nan());
        assert_eq!(vm.pop_f64(), Ok(0f64));
        assert_eq!(vm.pop_f64(), Ok(-1f64));
        assert_eq!(vm.pop_f64(), Err(RuntimeError::StackUnderflow));
        Ok(())
    }

    #[test]
    fn pop_stack_f64_fb() {
        let mut vm = vm_with_stack(vec![1, 2, 3, 4,
                                        i64::from_be_bytes(13.5f64.to_be_bytes())]);
        vm.fb = 4;
        assert_eq!(vm.pop_f64(), Ok(13.5));
        assert_eq!(vm.stack.len(), 4);
        assert_eq!(vm.pop_f64(), Err(RuntimeError::StackUnderflow));
        assert_eq!(vm.stack.len(), 4);
    }

    #[test]
    fn push_pop_stack() -> Result<(), RuntimeError> {
        // not strickly a unit test, but might catch some nasty problems
        let mut vm = vm_with_stack(Vec::with_capacity(10));
        vm.push(1)?;
        vm.push(-1)?;
        vm.push_f64(123.456f64)?;
        vm.push(0)?;
        assert_eq!(vm.pop(), Ok(0));
        assert_eq!(vm.pop_f64(), Ok(123.456));
        assert_eq!(vm.pop(), Ok(-1));
        assert_eq!(vm.pop(), Ok(1));
        assert_eq!(vm.pop_f64(), Err(RuntimeError::StackUnderflow));
        Ok(())
    }

    #[test]
    fn cross_push_pop_stack() -> Result<(), RuntimeError> {
        let mut vm = vm_with_stack(Vec::with_capacity(10));
        vm.push(100)?;
        vm.push_f64(0.123f64)?;
        vm.push(200)?;
        assert_eq!(vm.pop(), Ok(200));
        vm.pop()?;
        assert_eq!(vm.pop(), Ok(100));
        Ok(())
    }

    #[test]
    fn load_u8_empty_program() {
        let mut vm = vm_with_stack(vec![]);
        let pgm = pgm_with_text(vec![]);
        assert_eq!(vm.pc, 0);
        assert_eq!(vm.load_u8(&pgm), Err(RuntimeError::EndOfProgram));
        assert_eq!(vm.pc, 0);
        assert_eq!(vm.load_u8(&pgm), Err(RuntimeError::EndOfProgram));
        assert_eq!(vm.pc, 0);
    }

    #[test]
    fn load_u8_whole_program() {
        let mut vm = vm_with_stack(vec![]);
        let pgm = pgm_with_text(vec![0xf3, 0x00, 0x01, 0x02]);
        assert_eq!(vm.pc, 0);
        assert_eq!(vm.load_u8(&pgm), Ok(0xf3));
        assert_eq!(vm.pc, 1);
        assert_eq!(vm.load_u8(&pgm), Ok(0x00));
        assert_eq!(vm.pc, 2);
        assert_eq!(vm.load_u8(&pgm), Ok(0x01));
        assert_eq!(vm.pc, 3);
        assert_eq!(vm.load_u8(&pgm), Ok(0x02));
        assert_eq!(vm.pc, 4);
        assert_eq!(vm.load_u8(&pgm), Err(RuntimeError::EndOfProgram));
        assert_eq!(vm.pc, 4);
        assert_eq!(vm.load_u8(&pgm), Err(RuntimeError::EndOfProgram));
        assert_eq!(vm.pc, 4);
    }
}
