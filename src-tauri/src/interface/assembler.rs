use std::fmt::Display;

use crate::{
    interface::parser::{ParserInstSet, ParserResult},
    types::middleware_types::AssemblerConfig,
};

pub trait Assembler<IS>: Send + Sync
where
    IS: ParserInstSet + InstructionSetTrait,
{
    fn assemble(&mut self, ast: ParserResult<IS>)
        -> Result<AssembleResult<IS>, Vec<AssemblyError>>;
    fn update_config(&mut self, config: &AssemblerConfig);
    fn dump(&mut self, ast: ParserResult<IS>) -> Result<Memory, Vec<AssemblyError>>;
}

pub trait InstructionSetTrait {
    type Register: Clone;
    type Immediate: Clone;
}

#[derive(Clone)]
pub struct AssembleResult<IS: ParserInstSet + InstructionSetTrait> {
    pub data: Vec<u8>,
    pub instruction: Vec<InstructionSet<IS>>,
}

#[derive(Clone)]
pub struct InstructionSet<IS: ParserInstSet + InstructionSetTrait> {
    pub line_number: u64,
    pub instruction: Instruction<IS>,
    pub address: u32,
    pub code: u32,
    pub basic: String,
}

#[derive(Clone)]
pub struct Instruction<IS: ParserInstSet + InstructionSetTrait> {
    pub operation: IS::Operator,
    pub operands: Vec<Operand<IS>>,
}

#[allow(type_alias_bounds)]
pub type Operand<IS: ParserInstSet + InstructionSetTrait> = IS::Immediate;

pub struct Memory {
    pub data: String,
    pub text: String,
}

#[derive(Debug)]
pub struct AssemblyError {
    pub line: usize,
    pub msg: String,
}

impl Display for AssemblyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line:{} {}", self.line, self.msg)
    }
}

impl<IS: ParserInstSet + InstructionSetTrait> Instruction<IS> {
    pub fn new(operation: IS::Operator) -> Self {
        Instruction {
            operation,
            operands: vec![],
        }
    }
}

impl<IS: ParserInstSet + InstructionSetTrait> InstructionSet<IS> {
    pub fn new(instruction: Instruction<IS>) -> Self {
        InstructionSet {
            line_number: 0,
            instruction,
            address: 0,
            code: 0,
            basic: String::new(),
        }
    }
}
