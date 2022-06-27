/// Holds a program to be executed in VM.
#[derive(Debug)]
pub struct Pgm {
    /// Bytecode holding the programs instructions.
    pub text: Vec<u8>,
}
