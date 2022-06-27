/// Module holding the constants defining the opcodes for the VM.
pub mod op {
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
    /// Reads the next byte from the bytecode, increase programm counter, and return byte.
    fn fetch_u8(&mut self, pgm: &[u8]) -> u8 {
        if self.pc >= pgm.len() {
            panic!("End of program exceeded");
        }
        let v = pgm[self.pc];
        self.pc += 1;
        v
    }

    /// Executes a program (encoded in bytecode).
    pub fn run(&mut self, pgm: &[u8]) {
        // initialise the VM to be in a clean start state:
        self.stack.clear();
        self.pc = 0;
        self.op_cnt = 0;

        // Loop going through the whole program, one instruction at a time.
        loop {
            // Log the vm's complete state, so we can follow what happens in console:
            println!("{:?}", self);
            // Fetch next opcode from program (increases program counter):
            let opcode = self.fetch_u8(pgm);
            // We count the number of instructions we execute:
            self.op_cnt += 1;
            // If we are done, break loop and stop execution:
            if opcode == op::FIN {
                break;
            }
            // Execute the current instruction (with the opcode we loaded already):
            self.execute_op(pgm, opcode);
        }
        // Execution terminated. Output the final state of the VM:
        println!("Terminated!");
        println!("{:?}", self);
    }

    /// Executes an instruction, using the opcode passed.
    ///
    /// This might load more data from the program (opargs) and
    /// manipulate the stack (push, pop).
    fn execute_op(&mut self, pgm: &[u8], opcode: u8) {
        println!("Executing op 0x{:02x}", opcode);
        match opcode {
            op::NOP => {
                println!("  NOP");
                // do nothing
            },
            op::POP => {
                println!("  POP");
                let v = self.stack.pop().unwrap();
                println!("  dropping value {}", v);
            },
            op::PUSH_U8 => {
                println!("  PUSH_U8");
                let v = self.fetch_u8(pgm);
                println!("  value: {}", v);
                self.stack.push(v as i64);
            },
            op::ADD => {
                println!("  ADD");
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                self.stack.push(a + b);
            },
            _ => {
                panic!("unknown opcode!");
            }
        }
    }
}

fn main() {
    // Create a program in bytecode.
    // We just hardcode the bytes in an array here:
    let pgm = [op::NOP, op::PUSH_U8, 100, op::PUSH_U8, 77, op::ADD, op::POP, 0xff];
    // Crate our VM instance.
    let mut vm = VM{
        stack: Vec::with_capacity(100),
        pc: 0,
        op_cnt: 0
    };
    // Execute the program in our VM:
    vm.run(&pgm);
}
