use std::fs::File;
use lovem::{VM};

pub fn main() {
    let file = File::open("a/k1.lva").unwrap();

    let pgm = match lovem::asm::assemble_file(file) {
        Ok(pgm) => {
            pgm
        },
        Err((e, n)) => {
            println!("Error in line {}: {:?}", n, e);
            return;
        }
    };

    println!("PGM: {:?}", &pgm);
    println!("sz: {}", &pgm.text.len());

    let mut vm = VM::new();
    if let Err(e) = vm.run(&pgm, "") {
        println!("Runtime Error: {:?}", e);
    } else {
        println!("Terminated:\n{:?}.", vm);
    }
}
