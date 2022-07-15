//! Create a VM and run a small bytecode program in it.
//!
//! This will blow up with a runtime error, because it
//! tries modulo division by zero.
use lovem::{op, VM};

fn main() {
    // Create a program in bytecode.
    // We just hardcode the bytes in an array here:
    let pgm = [op::PUSH_U8, 4, op::PUSH_U8, 0, op::MOD, op::POP, op::FIN];
    // Create our VM instance.
    let mut vm = VM::new(100);
    // Execute the program in our VM:
    match vm.run(&pgm) {
        Ok(_) => {
            println!("Execution successful.")
        }
        Err(e) => {
            println!("Error during execution: {:?}", e);
        }
    }
}
