pub mod op;
pub mod pgm;
pub mod vm;
pub mod asm;
pub mod chunk;

extern crate rand;
extern crate lazy_static;
extern crate parse_int;

// re-export main types
pub use crate::pgm::Pgm;
pub use crate::vm::VM;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
