use std::fmt::Display;

pub use crate::modules::riscv::basic::assembler::assembler::RiscVAssembler;
use crate::{
    interface::assembler::{InstructionSet, InstructionSetTrait},
    modules::riscv::basic::interface::parser::*,
};

impl InstructionSetTrait for RISCV {
    type Register = ParserRISCVRegister;
    type Immediate = RISCVImmediate;
}

impl Display for InstructionSet<RISCV> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "line_number:{:3}; address:0x{:08x}; code:0x{:08x}; basic:{}; Instruction:{:?}",
            self.line_number, self.address, self.code, self.basic, self.instruction.operation,
        )?;
        write!(
            f,
            "{}",
            self.instruction
                .operands
                .iter()
                .map(|ins| ins.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
        .expect("panic");
        Ok(())
    }
}
