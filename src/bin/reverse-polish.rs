//! A small program demonstrating execution of arithmetics in our VM.
//!
//! For an explanation of what we are doing here, look at this wikipedia article:
//! https://en.wikipedia.org/w/index.php?title=Stack_machine&oldid=1097292883#Design
use lovem::{op, VM};

// A*(B-C)+(D+E)
// A B C - * D E + +
// A = 5, B = 7, C = 11, D = 13, E = 17
// 5 * (7 - 11) + (13 + 17) = 10

fn main() {
    // Create a program in bytecode.
    // We just hardcode the bytes in an array here:
    let pgm = [op::PUSH_U8, 5, op::PUSH_U8, 7, op::PUSH_U8, 11, op::SUB, op::MUL,
        op::PUSH_U8, 13, op::PUSH_U8, 17, op::ADD, op::ADD, op::POP, op::FIN];
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
