use std::fs::File;
use lovem::{op, VM};

fn main() {
    let file = File::open("pgm/adding-with-noise.lva").unwrap();
    let pgm = lovem::asm::assemble(file).unwrap();
    // Crate our VM instance.
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
