use std::collections::HashMap;

#[derive(Debug)]
pub struct Pgm {
    pub ext: Vec<String>,
    pub text: Vec<u8>,
    pub labels: HashMap<String, usize>,
}
