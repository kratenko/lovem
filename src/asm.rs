use std::collections::HashMap;
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
    static ref VALID_LABEL: Regex = regex::Regex::new(r"^[A-Za-z][0-9A-Za-z_]{0,31}$").unwrap();
}

/// Errors that can happen during assembly.
#[derive(Debug, Clone)]
pub enum AsmError {
    InvalidLine,
    UnknownInstruction(String),
    UnexpectedArgument,
    MissingArgument,
    InvalidArgument,
    InvalidLabel(String),
    DuplicateLabel(String),
    UnknownLabel(String),
    JumpTooLong,
    InvalidVariable,
    TooManyVariables,
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
    /// Names argument taken from instruction (e.g. a label).
    ///
    /// This is used to store an identifier that acts as an argument to an instruction.
    /// This is saved, because we do not know how to execute those at the time of parsing;
    /// the complete source file must have been parsed before we know destination addresses
    /// of labels.
    /// This information will be used on the "2nd run" to set the oparg bytes.
    argument_token: Option<String>,
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
    /// A map storing label definitions by name with there position in bytecode.
    labels: HashMap<String, usize>,
    /// List holding all global variable names in order.
    vars: Vec<String>,
}

impl AsmPgm {
    /// Remove comments from line
    fn remove_comment(line: &str) -> &str {
       if let Some(pair) = line.split_once("#") {
            pair.0
        } else {
            line
        }
    }

    /// Removes all noise from an assembler program's line.
    fn clean_line(line: &str) -> String {
        // Trim start and end:
        let line = line.trim();
        // Reduce all whitespaces to a single space (0x20):
        ANY_WHITESPACES.replace_all(line, " ").to_string()
    }

    /// Handles a single cleaned line from an Assembly program.
    fn parse_clean_line(&mut self, line: String) -> Result<(), AsmError> {
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
            argument_token: None,
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
            argument_token: None,
        };
        self.push_instruction(i)
    }

    /*
    We have no a2-instructions at the moment
    /// Helper that creates an instruction with 1 byte of oparg and pushes it.
    fn push_a2_instruction(&mut self, opcode: u8, a0: u8, a1: u8) -> Result<(), AsmError> {
        let i = AsmInstruction{
            line_number: self.line_number,
            opcode,
            oparg: vec![a0, a1],
            pos: self.text_pos,
            label: None,
        };
        self.push_instruction(i)
    }
     */

    /// Helper, that pushes an instruction, that as two bytes oparg and has a label in assembler.
    ///
    /// Instruction will reserve two bytes of oparg, filled with zeros. The actual value will be
    /// updated in the second run through the program, when the whole source has been parsed.
    fn push_label_instruction(&mut self, opcode: u8, label: &str) -> Result<(), AsmError> {
        let i = AsmInstruction{
            line_number: self.line_number,
            opcode,
            oparg: vec![0, 0],
            pos: self.text_pos,
            argument_token: Some(String::from(label)),
        };
        self.push_instruction(i)
    }

    /// Helper that parses (and pushes) a line with an operation, that takes a label as arg and stores it in two bytes
    fn parse_label_instruction(&mut self, opcode: u8, oparg: Option<&str>) -> Result<(), AsmError> {
        let label = oparg.ok_or(AsmError::MissingArgument)?;
        if VALID_LABEL.is_match(label) {
            self.push_label_instruction(opcode, label)
        } else {
            Err(AsmError::InvalidLabel(String::from(label)))
        }
    }

    /// Returns index number of a variable by name.
    ///
    /// This will create new numbers for previously unseen variable names.
    /// Will emit AsmError::TooManyVariables if number exceeds `u8`.
    fn get_variable_index(&mut self, name: &str) -> Result<u8, AsmError> {
        let index = if let Some(index) = self.vars.iter().position(|r| r == name) {
            index
        } else {
            self.vars.push(String::from(name));
            self.vars.len() - 1
        };
        if index <= 0xff {
            Ok(index as u8)
        } else {
            Err(AsmError::TooManyVariables)
        }
    }

    /// Handles a single instruction of opcode an optional oparg parsed from Assembly file.
    fn parse_instruction(&mut self, opname: &str, oparg: Option<&str>) -> Result<(), AsmError> {
        match opname {
            "nop" => self.parse_a0_instruction(op::NOP, oparg),
            "fin" => self.parse_a0_instruction(op::FIN, oparg),
            "pop" => self.parse_a0_instruction(op::POP, oparg),
            "dup" => self.parse_a0_instruction(op::DUP, oparg),
            "out" => self.parse_a0_instruction(op::OUT, oparg),
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
            "goto" => self.parse_label_instruction(op::GOTO, oparg),
            "ifeq" => self.parse_label_instruction(op::IFEQ, oparg),
            "ifne" => self.parse_label_instruction(op::IFNE, oparg),
            "iflt" => self.parse_label_instruction(op::IFLT, oparg),
            "ifle" => self.parse_label_instruction(op::IFLE, oparg),
            "ifgt" => self.parse_label_instruction(op::IFGT, oparg),
            "ifge" => self.parse_label_instruction(op::IFGE, oparg),
            "load" => {
                let name = oparg.ok_or(AsmError::MissingArgument)?;
                if !VALID_LABEL.is_match(name) {
                    return Err(AsmError::InvalidVariable);
                }
                let ix = self.get_variable_index(name)?;
                self.push_a1_instruction(op::LOAD, ix)
            }
            "store" => {
                let name = oparg.ok_or(AsmError::MissingArgument)?;
                if !VALID_LABEL.is_match(name) {
                    return Err(AsmError::InvalidVariable);
                }
                let ix = self.get_variable_index(name)?;
                self.push_a1_instruction(op::STORE, ix)
            }
            "ret" => self.parse_a0_instruction(op::RET, oparg),
            "call" => self.parse_label_instruction(op::CALL, oparg),
            _ => Err(AsmError::UnknownInstruction(String::from(opname)))
        }
    }

    /// Parses and extracts optional label definition from line.
    ///
    /// Looks for a colon ':'. If one exists, the part before the first colon will be
    /// seen as the name for a label, that is defined on this line. Instructions inside
    /// the program that execute jumps can refer to these labels as a destination.
    /// Lines containing a label definition may also contain an instruction and/or a comment.
    /// This can return `AsmError::InvalidLabel` if the part before the colon is not a valid
    /// label name, or `AsmError::DuplicateLabel` if a label name is reused.
    /// If a label could be parsed, it will be stored to the `AsmPgm`.
    /// On success, the line without the label definition is returned, so that it can be
    /// used to extract an instruction. This will be the complete line, if there was no
    /// label definition.
    fn parse_label_definition<'a>(&mut self, line: &'a str) -> Result<&'a str, AsmError> {
        if let Some((label, rest)) = line.split_once(":") {
            let label = label.trim_start();
            if VALID_LABEL.is_match(label) {
                if self.labels.contains_key(label) {
                    Err(AsmError::DuplicateLabel(String::from(label)))
                } else {
                    self.labels.insert(String::from(label), self.text_pos);
                    Ok(rest)
                }
            } else {
                Err(AsmError::InvalidLabel(String::from(label)))
            }
        } else {
            Ok(line)
        }
    }

    /// Parses source code and fills AsmPgm with instructions from it.
    ///
    /// If there is an error, parsing is aborted, and the error is stored.
    fn parse(&mut self, content: &str) -> Result<(), AsmError> {
        // read the source, one line at a time, adding instructions:
        for (n, line) in content.lines().enumerate() {
            // File lines start counting at 1:
            self.line_number = n + 1;
            let line = AsmPgm::remove_comment(line);
            let line = self.parse_label_definition(line)?;
            let line = AsmPgm::clean_line(line);
            self.parse_clean_line(line)?;
        }
        Ok(())
    }

    /// Update those instructions that need post processing.
    ///
    /// Some instructions need information that is only present, after the complete
    /// source file has been parsed. Those will be updated in this "second run".
    /// Opargs have been filled with placeholders before. The number of bytes should not be
    /// altered, because jump destinations are calculated from the number of bytes.
    fn update_instructions(&mut self) -> Result<(), AsmError> {
        for i in &mut self.instructions {
            self.line_number = i.line_number;
            if let Some(label) = &i.argument_token {
                if let Some(&dest) = self.labels.get(label) {
                    let src = i.pos + i.size();
                    if src.abs_diff(dest) > i16::MAX as usize {
                        return Err(AsmError::JumpTooLong);
                    }
                    let delta = (dest as i64 - src as i64) as i16;
                    i.oparg[..2].copy_from_slice(&delta.to_be_bytes()[..2]);
                } else {
                    return Err(AsmError::UnknownLabel(String::from(label)));
                }
            }
        }
        Ok(())
    }

    fn process(&mut self, content: &str) -> Result<(), AsmError> {
        // Go over complete source, extracting instructions. Some will have their opargs
        // left empty (with placeholders).
        self.parse(content)?;
        self.update_instructions()
    }

    /// Process assembly source code. Must be used with "empty" AsmPgm.
    fn process_assembly(&mut self, content: &str) {
        // this function is just a wrapper around `process()`, so that I can use the
        // return magic and don't need to write the error check twice.
        if let Err(e) = self.process(content) {
            self.error = Some(e);
        }
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
                vars: self.vars.len() as u8,
            })
        }
    }
}

/// Parse assembly source code and turn it into a runnable program (or create report).
pub fn assemble(name: &str, content: &str) -> Result<Pgm, AsmErrorReport> {
    // create a new, clean instance to fill during parsing:
    let mut asm_pgm = AsmPgm {
        name: String::from(name),
        instructions: vec![],
        line_number: 0,
        text_pos: 0,
        error: None,
        labels: Default::default(),
        vars: Default::default(),
    };
    // evaluate the source code:
    asm_pgm.process_assembly(content);
    // convert to Pgm instance if successful, or to Error Report, if assembly failed:
    asm_pgm.to_program()
}
