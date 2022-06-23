use std::fs::File;
use lovem::asm::AsmPgm;

fn main() {
    let file = File::open("a/k1.lva").unwrap();
    let p = AsmPgm::parse(file);
    println!("{:?}", &p);
    if let Some(e) = &p.error {
        println!("Error in line {}: {:?}", &p.line_number, e);
    }
    println!("{:?}", p.compile());
}
