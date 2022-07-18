//! Create a VM and run a small bytecode program in it.
//!
//! This demonstrates the goto operation with an endless loop.
use lovem::{op, VM};

fn main() {
    // Create a program in bytecode.
    // We just hardcode the bytes in an array here:
    let pgm = [op::PUSH_U8, 123, op::GOTO, 0xff, 0xfb, op::FIN];
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
