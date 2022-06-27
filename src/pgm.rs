use std::collections::HashMap;

#[derive(Debug)]
pub struct Pgm {
    pub ext: Vec<String>,
    pub text: Vec<u8>,
    pub labels: HashMap<String, usize>,
    pub vars: u8,
}

impl Pgm {
    pub fn dump(&self) {
        println!("lovem program");
        println!("  ext:    {:?}", self.ext);
        println!("  labels: {:?}", self.labels);
        println!("  vars:   {:?}", self.vars);
        println!("  len:    {:?}", self.text.len());
        println!("  text:   {:?}", self.text);
    }
}
