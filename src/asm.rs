use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use lazy_static::lazy_static;
use regex::Regex;
use crate::{op, Pgm};

#[derive(Debug, Clone)]
pub enum AsmError {
    InvalidLine,
    LineTooLong,
    InvalidLabel,
    DuplicateLabel(usize),
    UnknownLabel(String),
    UnknownOperation(String),
    UnexpectedArgument,
    MissingArgument,
    InvalidArgument,
    BranchTooLong,
    TooManyExternalSymbols,
    UndefinedGlobal(String),
}

#[derive(Debug)]
pub struct Operation {
    opcode: u8,
    line_number: usize,
    pos: usize,
    parm: Vec<u8>,
    label: Option<String>,
}

impl Operation {
    pub fn size(&self) -> usize {
        1 + self.parm.len()
    }
}

#[derive(Debug)]
pub struct AsmPgm {
    pub labels: HashMap<String, usize>,
    pub globals: HashSet<String>,
    pub exts: Vec<String>,
    pub operations: Vec<Operation>,
    pub line_number: usize,
    pub text_pos: usize,
    pub error: Option<AsmError>,
}

lazy_static! {
    static ref LABEL_LINE_RE: Regex = regex::Regex::new(r"^([^:\s]+):$").unwrap();
    static ref LABEL_NAME_RE: Regex = regex::Regex::new(r"^\.?[A-Za-z][0-9A-Za-z]*$").unwrap();
    static ref OP_LINE_RE: Regex = regex::Regex::new(r"^(\S+)(?:\s+(.+))?$").unwrap();
    static ref EXT_NAME_RE: Regex = regex::Regex::new(r"^?[A-Za-z][0-9A-Za-z]*$").unwrap();
}

impl AsmPgm {
    fn clean_line(line: String) -> String {
        // remove comments
        let line = if let Some(pair) = line.split_once("#") {
            pair.0
        } else {
            &line
        };
        // Trim
        String::from(line.trim())
    }

    fn parse_label(&mut self, name: String) -> Result<(), AsmError> {
        if LABEL_NAME_RE.is_match(&name) {
            if let Some(prev) = self.labels.insert(name, self.text_pos) {
                Err(AsmError::DuplicateLabel(prev))
            } else {
                Ok(())
            }
        } else {
            Err(AsmError::InvalidLabel)
        }
    }

    fn push_op_1_parm(&mut self, opcode: u8, p0: u8) -> Result<(), AsmError> {
        let o = Operation { opcode, line_number: self.line_number, pos: self.text_pos, parm: vec![p0], label: None };
        self.push_op(o);
        Ok(())
    }

    fn push_op_2_parms(&mut self, opcode: u8, p0: u8, p1: u8) -> Result<(), AsmError> {
        let o = Operation { opcode, line_number: self.line_number, pos: self.text_pos, parm: vec![p0, p1], label: None };
        self.push_op(o);
        Ok(())
    }

    fn push_op_4_parms(&mut self, opcode: u8, p0: u8, p1: u8, p2: u8, p3: u8) -> Result<(), AsmError> {
        let o = Operation { opcode, line_number: self.line_number, pos: self.text_pos, parm: vec![p0, p1, p2, p3], label: None };
        self.push_op(o);
        Ok(())
    }

    fn push_op_no_parm(&mut self, opcode: u8) -> Result<(), AsmError> {
        let o = Operation { opcode, line_number: self.line_number, pos: self.text_pos, parm: vec![], label: None };
        self.push_op(o);
        Ok(())
    }

    fn parse_op_no_parm(&mut self, opcode: u8, parm: Option<&str>) -> Result<(), AsmError> {
        if parm.is_some() {
            Err(AsmError::UnexpectedArgument)
        } else {
            self.push_op_no_parm(opcode)
        }
    }

    fn parse_op_label(&mut self, opcode: u8, parm: Option<&str>) -> Result<(), AsmError> {
        if let Some(label_name) = parm {
            if LABEL_NAME_RE.is_match(label_name) {
                let o = Operation {
                    opcode,
                    line_number: self.line_number,
                    pos: self.text_pos,
                    parm: vec![0, 0],
                    label: Some(String::from(label_name)),
                };
                self.push_op(o);
                Ok(())
            } else {
                Err(AsmError::InvalidLabel)
            }
        } else {
            Err(AsmError::MissingArgument)
        }

    }
    fn parse_op_ext(&mut self, opcode: u8, parm: Option<&str>) -> Result<(), AsmError> {
        let parm = parm.ok_or(AsmError::MissingArgument)?;
        if !EXT_NAME_RE.is_match(parm) {
            return Err(AsmError::InvalidArgument);
        }

        let index = if let Some(index) = self.exts.iter().position(|r| r == parm) {
            index
        } else {
            self.exts.push(String::from(parm));
            self.exts.len() - 1
        };
        if index > 0xffff {
            return Err(AsmError::TooManyExternalSymbols);
        }
        let index = index as u16;
        let o = Operation {
            opcode,
            line_number: self.line_number,
            pos: self.text_pos,
            parm: index.to_be_bytes().to_vec(),
            label: None,
        };
        self.push_op(o);
        Ok(())
    }

    fn push_op(&mut self, o: Operation) {
        self.text_pos += o.size();
        self.operations.push(o);
    }

    fn parse_op(&mut self, opname: &str, parm: Option<&str>) -> Result<(), AsmError> {
        match opname {
            "const_0" => self.parse_op_no_parm(op::CONST_0, parm),
            "const_1" => self.parse_op_no_parm(op::CONST_1, parm),
            "fconst_0" => self.parse_op_no_parm(op::FCONST_0, parm),
            "fconst_1" => self.parse_op_no_parm(op::FCONST_1, parm),
            "push_rnd" => self.parse_op_no_parm(op::PUSH_RND, parm),
            "push_u8" => {
                let parm = parm.ok_or(AsmError::MissingArgument)?;
                let v = parse_int::parse::<u8>(parm).or(Err(AsmError::InvalidArgument))?;
                self.push_op_1_parm(op::PUSH_U8, v)
            }
            "and" => self.parse_op_no_parm(op::AND, parm),
            "sub" => self.parse_op_no_parm(op::SUB, parm),
            "fin" => self.parse_op_no_parm(op::FIN, parm),
            "dup" => self.parse_op_no_parm(op::DUP, parm),
            "pop" => self.parse_op_no_parm(op::POP, parm),
            "ifgt" => self.parse_op_label(op::IFGT, parm),
            "add_1" => self.parse_op_no_parm(op::ADD_1, parm),
            "sub_1" => self.parse_op_no_parm(op::SUB_1, parm),
            "mul" => self.parse_op_no_parm(op::MUL, parm),
            "div" => self.parse_op_no_parm(op::DIV, parm),
            "mod" => self.parse_op_no_parm(op::MOD, parm),
            "fadd" => self.parse_op_no_parm(op::FADD, parm),
            "fsub" => self.parse_op_no_parm(op::FSUB, parm),
            "fmul" => self.parse_op_no_parm(op::FMUL, parm),
            "fdiv" => self.parse_op_no_parm(op::FDIV, parm),
            "push_f32" => {
                let parm = parm.ok_or(AsmError::MissingArgument)?;
                let v = parm.parse::<f32>().or(Err(AsmError::InvalidArgument))?;
                let b = v.to_be_bytes();
                let o = Operation { opcode: op::PUSH_F32, line_number: self.line_number, pos: self.text_pos, parm: b.to_vec(), label: None };
                self.push_op(o);
                Ok(())
            }
            "push_f64" => {
                let parm = parm.ok_or(AsmError::MissingArgument)?;
                let v = parm.parse::<f64>().or(Err(AsmError::InvalidArgument))?;
                let b = v.to_be_bytes();
                let o = Operation { opcode: op::PUSH_F64, line_number: self.line_number, pos: self.text_pos, parm: b.to_vec(), label: None };
                self.push_op(o);
                Ok(())
            }
            "call" => self.parse_op_label(op::CALL, parm),
            "ret" => self.parse_op_no_parm(op::RET, parm),
            "dev" => self.parse_op_no_parm(op::DEV, parm),
            "dev2" => self.parse_op_no_parm(op::DEV2, parm),
            "global" => {
                let parm = parm.ok_or(AsmError::MissingArgument)?;
                if !EXT_NAME_RE.is_match(parm) {
                    return Err(AsmError::InvalidArgument);
                }
                self.globals.insert(String::from(parm));
                Ok(())
            }
            "ecall" => {
                self.parse_op_ext(op::ECALL, parm)
            },
            "push_i" => {
                // macro
                let parm = parm.ok_or(AsmError::MissingArgument)?;
                let v = parse_int::parse::<i64>(parm).or(Err(AsmError::InvalidArgument))?;
                match v {
                    0 => self.push_op_no_parm(op::CONST_0),
                    1 => self.push_op_no_parm(op::CONST_1),
                    2..=255 => self.push_op_1_parm(op::PUSH_U8, v as u8),
                    0x100..=0xffff => {
                        let b = (v as u16).to_be_bytes();
                        self.push_op_2_parms(op::PUSH_U16, b[0], b[1])
                    }
                    0x10000..=0xffffffff => {
                        let b = (v as u32).to_be_bytes();
                        self.push_op_4_parms(op::PUSH_U32, b[0], b[1], b[2], b[3])
                    }
                    -0xff..=-1 => {
                        self.push_op_1_parm(op::PUSH_U8, -v as u8)?;
                        self.push_op_no_parm(op::INV)
                    }
                    -0xffff..=-0x100 => {
                        let b = (-v as u16).to_be_bytes();
                        self.push_op_2_parms(op::PUSH_U16, b[0], b[1])?;
                        self.push_op_no_parm(op::INV)
                    }
                    -0xffffffff..=-0x10000 => {
                        let b = (-v as u32).to_be_bytes();
                        self.push_op_4_parms(op::PUSH_U32, b[0], b[1], b[2], b[3])?;
                        self.push_op_no_parm(op::INV)
                    }
                    _ => {
                        let b = v.to_be_bytes();
                        let o = Operation { opcode: op::PUSH_I64, line_number: self.line_number, pos: self.text_pos, parm: b.to_vec(), label: None };
                        self.push_op(o);
                        Ok(())
                    }
                }
            }
            _ => Err(AsmError::UnknownOperation(String::from(opname)))
        }
    }

    fn parse_line(&mut self, line: String) -> Result<(), AsmError> {
        if line == "" {
            return Ok(());
        }
        if line.len() > 255 {
            return Err(AsmError::LineTooLong);
        }
        if let Some(caps) = LABEL_LINE_RE.captures(&line) {
            // "label_name:"
            let name = String::from(caps.get(1).unwrap().as_str());
            return self.parse_label(name);
        }
        if let Some(caps) = OP_LINE_RE.captures(&line) {
            let opname = caps.get(1).unwrap().as_str();
            let parm = caps.get(2).map(|m| m.as_str());
            return self.parse_op(opname, parm);
        }
        Err(AsmError::InvalidLine)
    }

    pub fn parse(file: File) -> AsmPgm {
        let mut p = AsmPgm {
            labels: HashMap::new(),
            globals: HashSet::new(),
            exts: vec![],
            operations: vec![],
            line_number: 0,
            text_pos: 0,
            error: None,
        };
        let lines = io::BufReader::new(file).lines().enumerate();
        for (n, line) in lines {
            p.line_number = n + 1;
            let line = AsmPgm::clean_line(line.unwrap());
            match p.parse_line(line) {
                Ok(_) => {}
                Err(e) => {
                    p.error = Some(e);
                    break;
                }
            }
        }
        // fix branch offsets
        for o in &mut p.operations {
            if let Some(label_name) = &o.label {
                if let Some(label_pos) = p.labels.get(label_name) {
                    let offset = *label_pos as isize - (o.pos + o.size()) as isize;
                    if let Ok(offset) = i16::try_from(offset) {
                        assert_eq!(o.parm.len(), 2);
                        let bb = offset.to_be_bytes();
                        o.parm[0] = bb[0];
                        o.parm[1] = bb[1];
                    } else {
                        p.error = Some(AsmError::BranchTooLong);
                        p.line_number = o.line_number;
                        break;
                    }
                } else {
                    p.error = Some(AsmError::UnknownLabel(String::from(label_name)));
                    p.line_number = o.line_number;
                    break;
                }
            }
        }
        p
    }

    pub fn compile(&self) -> Result<Pgm, AsmError> {
        let mut code: Vec<u8> = vec![];
        for o in &self.operations {
            code.push(o.opcode);
            code.extend(&o.parm);
        }
        if let Some(e) = &self.error {
            return Err(e.clone());
        }
        let mut labels = HashMap::new();
        for n in &self.globals {
            let pos = self.labels.get(n).ok_or(AsmError::UndefinedGlobal(String::from(n)))?;
            labels.insert(String::from(n), *pos);
        }
        Ok(Pgm{
            ext: self.exts.clone(),
            text: code,
            labels,
        })
    }
}
