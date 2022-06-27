use std::fs::File;

fn main() {
    let file = File::open("pgm/adding.lva").unwrap();
    println!("{:?}", lovem::asm::assemble(file));
}
