//! Create a VM and run a small bytecode program in it.
//!
//! This will demonstrate how fetching and executing operations work in the VM.
use lovem::{op, VM};

fn main() {
    // Create a program in bytecode.
    // We just hardcode the bytes in an array here:
    let pgm = [op::NOP, op::PUSH_U8, 100, op::PUSH_U8, 77, op::ADD, op::POP, op::FIN];
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
