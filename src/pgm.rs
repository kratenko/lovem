/// Holds a program to be executed in VM.
#[derive(Debug)]
pub struct Pgm {
    /// Some name identifying the program.
    pub name: String,
    /// Bytecode holding the programs instructions.
    pub text: Vec<u8>,
    /// Number of global variables in program.
    pub vars: u8,
}
