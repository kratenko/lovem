//! A simple assembler program to easily create bytecode.
use std::fs::File;
use std::io;
use std::io::BufRead;
use regex::Regex;
use lazy_static::lazy_static;
use crate::{op, Pgm};

lazy_static! {
    static ref ANY_WHITESPACES: Regex = regex::Regex::new(r"\s+").unwrap();
    static ref OP_LINE_RE: Regex = regex::Regex::new(r"^(\S+)(?: (.+))?$").unwrap();
}

#[derive(Debug, Clone)]
pub enum AsmError {
    InvalidLine,
    UnknownInstruction(String),
    UnexpectedArgument,
    MissingArgument,
    InvalidArgument,
}

#[derive(Debug)]
pub struct AsmLineError {
    pub line: usize,
    pub error: AsmError,
}

#[derive(Debug)]
struct AsmInstruction {
    /// Number of line the instruction was read from.
    ///
    /// The number of the line the instruction was taken from, most likely
    /// from a source file. Line counting starts at 1.
    line_number: usize,
    /// Opcode defining which operation is to be executed.
    opcode: u8,
    /// Arguments used for execution of the operation.
    ///
    /// Zero or more bytes.
    oparg: Vec<u8>,
    /// Position inside bytecode (starting at 0).
    ///
    /// Number of bytes that come before this instruction in the program.
    pos: usize,
}

impl AsmInstruction {
    /// Returns the size of the instruction including its parameters.
    ///
    /// Gives the number of bytes this instruction will use inside bytecode. Needed to calculate
    /// branching distances.
    pub fn size(&self) -> usize {
        1 + self.oparg.len()
    }
}


#[derive(Debug)]
struct AsmPgm {
    instructions: Vec<AsmInstruction>,
    line_number: usize,
    text_pos: usize,
    error: Option<AsmError>,
}

impl AsmPgm {
    /// Removes all noise from an assembler program's line.
    fn clean_line(line: &str) -> String {
        // remove comments
        let line = if let Some(pair) = line.split_once("#") {
            pair.0
        } else {
            &line
        };
        // Trim start and end:
        let line = String::from(line.trim());
        // Reduce all whitespaces to single spaces
        ANY_WHITESPACES.replace_all(&line, " ").to_string()
    }

    /// Handles a single line from an Assembly program.
    fn parse_line(&mut self, line: String) -> Result<(), AsmError> {
        if line == "" {
            // empty line (or comment only) - skip
            return Ok(());
        }
        if let Some(caps) = OP_LINE_RE.captures(&line) {
            let opname = caps.get(1).unwrap().as_str();
            let parm = caps.get(2).map(|m| m.as_str());
            return self.parse_instruction(opname, parm);
        }
        Err(AsmError::InvalidLine)
    }

    /// Adds a single instruction to the end of the AsmProgram.
    fn push_instruction(&mut self, i: AsmInstruction) -> Result<(), AsmError> {
        self.text_pos += i.size();
        self.instructions.push(i);
        Ok(())
    }

    /// Helper that creates an instruction with 0 bytes of oparg and pushes it.
    fn push_a0_instruction(&mut self, opcode: u8) -> Result<(), AsmError> {
        let i = AsmInstruction{
            line_number: self.line_number,
            opcode,
            oparg: vec![],
            pos: self.text_pos,
        };
        self.push_instruction(i)
    }

    /// Helper that parses an instruction with no oparg and pushes it.
    fn parse_a0_instruction(&mut self, opcode: u8, oparg: Option<&str>) -> Result<(), AsmError> {
        if oparg.is_some() {
            Err(AsmError::UnexpectedArgument)
        } else {
            self.push_a0_instruction(opcode)
        }
    }

    /// Helper that creates an instruction with 1 byte of oparg and pushes it.
    fn push_a1_instruction(&mut self, opcode: u8, a1: u8) -> Result<(), AsmError> {
        let i = AsmInstruction{
            line_number: self.line_number,
            opcode,
            oparg: vec![a1],
            pos: self.text_pos,
        };
        self.push_instruction(i)
    }

    /// Handles a single instruction of opcode an optional oparg parsed from Assembly file.
    fn parse_instruction(&mut self, opname: &str, oparg: Option<&str>) -> Result<(), AsmError> {
        match opname {
            "nop" => self.parse_a0_instruction(op::NOP, oparg),
            "fin" => self.parse_a0_instruction(op::FIN, oparg),
            "pop" => self.parse_a0_instruction(op::POP, oparg),
            "dup" => self.parse_a0_instruction(op::DUP, oparg),
            "add" => self.parse_a0_instruction(op::ADD, oparg),
            "sub" => self.parse_a0_instruction(op::SUB, oparg),
            "mul" => self.parse_a0_instruction(op::MUL, oparg),
            "div" => self.parse_a0_instruction(op::DIV, oparg),
            "mod" => self.parse_a0_instruction(op::MOD, oparg),
            "neg" => self.parse_a0_instruction(op::NEG, oparg),
            "push_u8" => {
                let oparg = oparg.ok_or(AsmError::MissingArgument)?;
                let v = parse_int::parse::<u8>(oparg).or(Err(AsmError::InvalidArgument))?;
                self.push_a1_instruction(op::PUSH_U8, v)
            },
            _ => Err(AsmError::UnknownInstruction(String::from(opname)))
        }
    }

    /// Parses a complete lva file to internal parsed program representation.
    fn parse(file: File) -> AsmPgm {
        let mut p = AsmPgm {
            instructions: vec![],
            line_number: 0,
            text_pos: 0,
            error: None,
        };
        let lines = io::BufReader::new(file).lines().enumerate();
        for (n, line) in lines {
            p.line_number = n + 1;
            let line = AsmPgm::clean_line(&line.unwrap());
            if let Err(e) = p.parse_line(line) {
                // Store error in program and abort parsing:
                p.error = Some(e);
                break;
            }
        }
        /*
        // fix branch offsets
        if p.error.is_none() {
            for o in &mut p.instructions {
                if let Err(e) = o.update_label_parm_if_needed(&p.labels) {
                    // Safe error in program and update line number to that of the operation
                    // where the error occurred:
                    p.error = Some(e);
                    p.line_number = o.line_number;
                    // Abort parsing on first error:
                    break;
                }
            }
        }

         */
        // The parsed assembly program (might have run into error during parsing)
        p
    }

    fn to_program(&self) -> Result<Pgm, AsmLineError> {
        if let Some(e) = &self.error {
            // Cannot compile program with error:
            return Err(AsmLineError{line: self.line_number, error: e.clone()});
        }
        // concat bytes of all instructions to single chunk of bytecode
        let mut text: Vec<u8> = vec![];
        for o in &self.instructions {
            text.push(o.opcode);
            text.extend(&o.oparg);
        }
        Ok(Pgm{
            text
        })
    }
}

pub fn assemble(file: File) -> Result<Pgm, AsmLineError> {
    let asm_pgm = AsmPgm::parse(file);
    asm_pgm.to_program()
}


#[cfg(test)]
mod tests {
    use crate::asm::AsmPgm;
    #[test]
    fn test_clean_line() {
        // empty must remain empty:
        assert_eq!(AsmPgm::clean_line(""), String::from(""));
        // remove whitespaces and comments:
        assert_eq!(AsmPgm::clean_line(" "), String::from(""));
        assert_eq!(AsmPgm::clean_line("    "), String::from(""));
        assert_eq!(AsmPgm::clean_line("#"), String::from(""));
        assert_eq!(AsmPgm::clean_line("# "), String::from(""));
        assert_eq!(AsmPgm::clean_line("#    "), String::from(""));
        assert_eq!(AsmPgm::clean_line(" #"), String::from(""));
        assert_eq!(AsmPgm::clean_line(" # "), String::from(""));
        assert_eq!(AsmPgm::clean_line("\t#\t"), String::from(""));
        assert_eq!(AsmPgm::clean_line("#s"), String::from(""));
        assert_eq!(AsmPgm::clean_line("#somewhat"), String::from(""));
        assert_eq!(AsmPgm::clean_line("# somewhat"), String::from(""));
        assert_eq!(AsmPgm::clean_line("# somewhat else"), String::from(""));
        assert_eq!(AsmPgm::clean_line("# somewhat else   "), String::from(""));
        assert_eq!(AsmPgm::clean_line("    \t  #    d   somewhat else   \t"), String::from(""));
        assert_eq!(AsmPgm::clean_line("##"), String::from(""));
        assert_eq!(AsmPgm::clean_line("##########"), String::from(""));
        assert_eq!(AsmPgm::clean_line("            \t  ###\t###   #### === #"), String::from(""));
        // instructions without oparg
        assert_eq!(AsmPgm::clean_line("pop"), String::from("pop"));
        assert_eq!(AsmPgm::clean_line(" pop"), String::from("pop"));
        assert_eq!(AsmPgm::clean_line("pop "), String::from("pop"));
        assert_eq!(AsmPgm::clean_line("  push\t "), String::from("push"));
        assert_eq!(AsmPgm::clean_line("nopidubidu # I don't think we have that opname"), String::from("nopidubidu"));
        assert_eq!(AsmPgm::clean_line("    \t   \t      push                         # whatnow?\" &%$§@äöplkü"), String::from("push"));
        // instructions with oparg
        assert_eq!(AsmPgm::clean_line("load 12"), String::from("load 12"));
        assert_eq!(AsmPgm::clean_line("  load   0xa12  # sdf"), String::from("load 0xa12"));
        assert_eq!(AsmPgm::clean_line("  goto  \t   nowhere  # s    df # "), String::from("goto nowhere"));
    }
}
