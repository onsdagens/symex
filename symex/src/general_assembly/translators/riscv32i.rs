//! Translator for the RV32I instruction set

use std::ops::Add;

use riscv_instruction_parser::{
    instructons::{Instruction, Operation},
    registers::Register,
};
use regex::Regex;
use tracing::trace;

use crate::{
    elf_util::{ExpressionType, Variable},
    general_assembly::{
        instruction::{Condition, CycleCount, Operand},
        project::PCHook,
        state::GAState,
        translator::Translatable,
        DataWord,
    },
};

type GAInstruction = crate::general_assembly::instruction::Instruction;
type GAOperation = crate::general_assembly::instruction::Operation;

fn cycle_count(operation: &Operation) -> CycleCount{
    CycleCount::Value(1) //all instructions are guaranteed to execute within one cycle (save for memory
    //ops which are more complex)
}

impl Translatable for Instruction {
    fn translate(&self) -> GAInstruction{
       match self.operation {
           Operation::LUI { rd, imm } => {
                let operations = vec![

                ];

                GAInstruction{
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            } 
            _=> {todo!()}
        } 
    }
    fn add_pc_hooks(hooks: &mut Vec<(Regex, PCHook)>) {
        
    }
}
