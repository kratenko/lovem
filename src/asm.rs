use std::error;
use std::fmt::{Display, Formatter};
use lazy_static::lazy_static;
use regex::Regex;
use crate::{op, Pgm};

// Regular expressions used by the assembler.
// lazy static takes care that they are compiled only once and then reused.
lazy_static! {
    static ref ANY_WHITESPACES: Regex = regex::Regex::new(r"\s+").unwrap();
    static ref OP_LINE_RE: Regex = regex::Regex::new(r"^(\S+)(?: (.+))?$").unwrap();
}

/// Errors that can happen during assembly.
#[derive(Debug, Clone)]
pub enum AsmError {
    InvalidLine,
    UnknownInstruction(String),
    UnexpectedArgument,
    MissingArgument,
    InvalidArgument,
}

impl Display for AsmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for AsmError {

}

/// Report of failed assembly attempt.
///
/// Wraps the error that occurred during assembly and supplied information where it did.
#[derive(Debug)]
pub struct AsmErrorReport {
    /// Name of the program that failed to assemble.
    name: String,
    /// Line the error occurred during assembly.
    line: usize,
    /// Error that occurred.
    error: AsmError,
}

impl Display for AsmErrorReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "assembly failed in line {} of program '{}'", self.line, self.name)
    }
}

impl error::Error for AsmErrorReport {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.error)
    }
}

/// A single instruction parsed from the line of an assembly program.
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


/// A assembler program during parsing/assembling.
#[derive(Debug)]
struct AsmPgm {
    /// Name of the program (just a string supplied by caller).
    name: String,
    /// Vector of parsed assembler instructions, in the order they are in the source file.
    instructions: Vec<AsmInstruction>,
    /// Current line number during parsing.
    ///
    /// Used for error reporting.
    line_number: usize,
    /// Current position inside bytecode during parsing.
    ///
    /// Used to calculate the exact position an instruction will be in the bytecode.
    text_pos: usize,
    /// The error that happened during parsing/assembling, if any.
    error: Option<AsmError>,
}

impl AsmPgm {
    /// Removes all noise from an assembler program's line.
    fn clean_line(line: &str) -> String {
        // Remove comments:
        let line = if let Some(pair) = line.split_once("#") {
            pair.0
        } else {
            &line
        };
        // Trim start and end:
        let line = line.trim();
        // Reduce all whitespaces to a single space (0x20):
        ANY_WHITESPACES.replace_all(line, " ").to_string()
    }

    /// Handles a single cleaned line from an Assembly program.
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
    fn push_a1_instruction(&mut self, opcode: u8, a0: u8) -> Result<(), AsmError> {
        let i = AsmInstruction{
            line_number: self.line_number,
            opcode,
            oparg: vec![a0],
            pos: self.text_pos,
        };
        self.push_instruction(i)
    }

    /// Helper that creates an instruction with 1 byte of oparg and pushes it.
    fn push_a2_instruction(&mut self, opcode: u8, a0: u8, a1: u8) -> Result<(), AsmError> {
        let i = AsmInstruction{
            line_number: self.line_number,
            opcode,
            oparg: vec![a0, a1],
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
            "add" => self.parse_a0_instruction(op::ADD, oparg),
            "sub" => self.parse_a0_instruction(op::SUB, oparg),
            "mul" => self.parse_a0_instruction(op::MUL, oparg),
            "div" => self.parse_a0_instruction(op::DIV, oparg),
            "mod" => self.parse_a0_instruction(op::MOD, oparg),
            "push_u8" => {
                let oparg = oparg.ok_or(AsmError::MissingArgument)?;
                let v = parse_int::parse::<u8>(oparg).or(Err(AsmError::InvalidArgument))?;
                self.push_a1_instruction(op::PUSH_U8, v)
            },
            "goto" => {
                let oparg = oparg.ok_or(AsmError::MissingArgument)?;
                let v = parse_int::parse::<i16>(oparg).or(Err(AsmError::InvalidArgument))?;
                let a = v.to_be_bytes();
                self.push_a2_instruction(op::GOTO, a[0], a[1])
            },
            _ => Err(AsmError::UnknownInstruction(String::from(opname)))
        }
    }

    /// Parse an assembly program from source into `AsmPgm` struct.
    fn parse(name: &str, content: &str) -> AsmPgm {
        // create a new, clean instance to fill during parsing:
        let mut p = AsmPgm {
            name: String::from(name),
            instructions: vec![],
            line_number: 0,
            text_pos: 0,
            error: None,
        };
        // read the source, one line at a time, adding instructions:
        for (n, line) in content.lines().enumerate() {
            p.line_number = n + 1;
            let line = AsmPgm::clean_line(line);
            if let Err(e) = p.parse_line(line) {
                // Store error in program and abort parsing:
                p.error = Some(e);
                break;
            }
        }
        p
    }

    /// Convert parsed assembly source to runnable program (or error report).
    fn to_program(&self) -> Result<Pgm, AsmErrorReport> {
        if let Some(e) = &self.error {
            // Assembling failed:
            Err(AsmErrorReport{
                name: self.name.clone(),
                line: self.line_number,
                error: e.clone(),
            })
        } else {
            // Assembling succeeded, return a Pgm instance:
            let mut text: Vec<u8> = vec![];
            for i in &self.instructions {
                text.push(i.opcode);
                text.extend(&i.oparg);
            }
            Ok(Pgm{
                name: self.name.clone(),
                text,
            })
        }
    }
}

/// Parse assembly source code and turn it into a runnable program (or create report).
pub fn assemble(name: &str, content: &str) -> Result<Pgm, AsmErrorReport> {
    let asm_pgm = AsmPgm::parse(name, content);
    asm_pgm.to_program()
}
