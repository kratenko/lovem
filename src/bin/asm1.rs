use std::fs::File;
use lovem::asm::{AsmError, AsmPgm};
use lovem::Pgm;

fn main() {
    let file = File::open("a/k1.lva").unwrap();
    match lovem::asm::assemble_file(file) {
        Ok(pgm) => println!("{:?}", &p),
        Err((e, n)) => {
            println!("Error in line {}: {:?}", n, e);
        }
    }
}
