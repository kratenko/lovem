use std::fs::File;
use lovem::{op, Pgm, VM};
use rand::Rng;
use lovem::asm::{AsmError, AsmPgm};

pub fn main() {
    let file = File::open("a/k1.lva").unwrap();
    let p = AsmPgm::parse(file);
    println!("{:?}", &p);
    if let Some(e) = &p.error {
        println!("Error in line {}: {:?}", &p.line_number, e);
        return;
    }
    let r = p.compile();
    let p = match r {
        Ok(p) => p,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };

    println!("PGM: {:?}", &p);
    println!("sz: {}", &p.text.len());

    let mut vm = VM::new();
    if let Err(e) = vm.run(&p, "other") {
        println!("Runtime Error: {:?}", e);
    } else {
        println!("Terminated:\n{:?}.", vm);
    }
}
