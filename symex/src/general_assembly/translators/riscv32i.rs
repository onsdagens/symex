//! Translator for the RV32I instruction set
use regex::Regex;
use riscv_instruction_parser::{
    instructons::{Instruction, Operation},
    registers::Register,
};
use tracing::trace;

use crate::{
    elf_util::{ExpressionType, Variable},
    general_assembly::{
        instruction::{Condition, CycleCount, Operand},
        project::PCHook,
        state::GAState,
        translator::Translatable,
        DataWord, RunConfig,
    },
};

type GAInstruction = crate::general_assembly::instruction::Instruction;
type GAOperation = crate::general_assembly::instruction::Operation;

/*fn cycle_count(operation: &Operation) -> CycleCount {
    CycleCount::Value(1) //all instructions are guaranteed to execute within one cycle (save for memory
                         //ops which are arch dependent)
}*/

fn riscv_reg_to_ga_operand(reg: Register) -> Operand {
    Operand::Register(match reg {
        Register::ZERO => "ZERO".to_string(),
        Register::RA => "RA".to_string(),
        Register::SP => "SP".to_string(),
        Register::GP => "GP".to_string(),
        Register::TP => "TP".to_string(),
        Register::T0 => "T0".to_string(),
        Register::T1 => "T1".to_string(),
        Register::T2 => "T2".to_string(),
        Register::T3 => "T3".to_string(),
        Register::T4 => "T4".to_string(),
        Register::T5 => "T5".to_string(),
        Register::T6 => "T6".to_string(),
        Register::S0 => "S0".to_string(),
        Register::S1 => "S1".to_string(),
        Register::S2 => "S2".to_string(),
        Register::S3 => "S3".to_string(),
        Register::S4 => "S4".to_string(),
        Register::S5 => "S5".to_string(),
        Register::S6 => "S6".to_string(),
        Register::S7 => "S7".to_string(),
        Register::S8 => "S8".to_string(),
        Register::S9 => "S9".to_string(),
        Register::S10 => "S10".to_string(),
        Register::S11 => "S11".to_string(),
        Register::A0 => "A0".to_string(),
        Register::A1 => "A1".to_string(),
        Register::A2 => "A2".to_string(),
        Register::A3 => "A3".to_string(),
        Register::A4 => "A4".to_string(),
        Register::A5 => "A5".to_string(),
        Register::A6 => "A6".to_string(),
        Register::A7 => "A7".to_string(),
    })
}

impl Translatable for Instruction {
    fn translate(&self) -> GAInstruction {
        match self.operation {
            Operation::LUI { rd, imm } => {
                let operations = vec![
                    //rd[31:12] == imm
                    GAOperation::Move {
                        destination: riscv_reg_to_ga_operand(rd),
                        source: Operand::Immidiate(DataWord::Word32(imm)),
                    },
                ];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::ADD { rs2, rs1, rd } => {
                //rd = rs1 + rs2
                let operations = vec![GAOperation::Add {
                    destination: riscv_reg_to_ga_operand(rd),
                    operand1: riscv_reg_to_ga_operand(rs1),
                    operand2: riscv_reg_to_ga_operand(rs2),
                }];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::SUB { rs2, rs1, rd } => {
                //rd = rs1 - rs2
                let operations = vec![GAOperation::Sub {
                    destination: riscv_reg_to_ga_operand(rd),
                    operand1: riscv_reg_to_ga_operand(rs1),
                    operand2: riscv_reg_to_ga_operand(rs2),
                }];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::BEQ { imm, rs1, rs2 } => {
                let operations = vec![
                    GAOperation::Sub {
                        destination: Operand::Register("temp".to_string()),
                        operand1: riscv_reg_to_ga_operand(rs1),
                        operand2: riscv_reg_to_ga_operand(rs2),
                    },
                    GAOperation::SetZFlag(Operand::Register("temp".to_string())),
                    GAOperation::Add {
                        destination: Operand::Register("temp".to_string()),
                        operand1: Operand::Register("PC".to_string()),
                        operand2: Operand::Immidiate(DataWord::Word32(imm as u32)),
                    },
                    GAOperation::Sub {
                        destination: Operand::Register("temp".to_string()),
                        operand1: Operand::Register("temp".to_string()),
                        operand2: Operand::Immidiate(DataWord::Word32(4i32 as u32)),
                    },
                    GAOperation::ConditionalJump {
                        destination: Operand::Register("temp".to_string()),
                        condition: Condition::EQ,
                    },
                ];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::LB { imm, rs1, rd } => {
                let operations = vec![
                    GAOperation::Move {
                        destination: riscv_reg_to_ga_operand(rd),
                        source: Operand::AddressWithOffset {
                            address: DataWord::Word32(imm as u32),
                            offset_reg: match riscv_reg_to_ga_operand(rs1) {
                                Operand::Register(s) => s,
                                _ => panic!(),
                            },
                            width: 8,
                        },
                    },
                    GAOperation::SignExtend {
                        destination: riscv_reg_to_ga_operand(rd),
                        operand: riscv_reg_to_ga_operand(rd),
                        bits: 32,
                    },
                ];

                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::LH { imm, rs1, rd } => {
                let operations = vec![
                    GAOperation::Move {
                        destination: riscv_reg_to_ga_operand(rd),
                        source: Operand::AddressWithOffset {
                            address: DataWord::Word32(imm as u32),
                            offset_reg: match riscv_reg_to_ga_operand(rs1) {
                                Operand::Register(s) => s,
                                _ => panic!(),
                            },
                            width: 16,
                        },
                    },
                    GAOperation::SignExtend {
                        destination: riscv_reg_to_ga_operand(rd),
                        operand: riscv_reg_to_ga_operand(rd),
                        bits: 32,
                    },
                ];

                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::LW { imm, rs1, rd } => {
                let operations = vec![GAOperation::Move {
                    destination: riscv_reg_to_ga_operand(rd),
                    source: Operand::AddressWithOffset {
                        address: DataWord::Word32(imm as u32),
                        offset_reg: match riscv_reg_to_ga_operand(rs1) {
                            Operand::Register(s) => s,
                            _ => panic!(),
                        },
                        width: 32,
                    },
                }];

                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::SB { imm, rs2, rs1 } => {
                let operations = vec![GAOperation::Move {
                    destination: Operand::AddressWithOffset {
                        address: DataWord::Word32(imm as u32),
                        offset_reg: match riscv_reg_to_ga_operand(rs1) {
                            Operand::Register(s) => s,
                            _ => panic!(),
                        },
                        width: 8,
                    },
                    source: riscv_reg_to_ga_operand(rs2),
                }];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::SH { imm, rs2, rs1 } => {
                let operations = vec![GAOperation::Move {
                    destination: Operand::AddressWithOffset {
                        address: DataWord::Word32(imm as u32),
                        offset_reg: match riscv_reg_to_ga_operand(rs1) {
                            Operand::Register(s) => s,
                            _ => panic!(),
                        },
                        width: 16,
                    },
                    source: riscv_reg_to_ga_operand(rs2),
                }];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::SW { imm, rs2, rs1 } => {
                let operations = vec![GAOperation::Move {
                    destination: Operand::AddressWithOffset {
                        address: DataWord::Word32(imm as u32),
                        offset_reg: match riscv_reg_to_ga_operand(rs1) {
                            Operand::Register(s) => s,
                            _ => panic!(),
                        },
                        width: 32,
                    },
                    source: riscv_reg_to_ga_operand(rs2),
                }];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::OR { rs2, rs1, rd } => {
                let operations = vec![GAOperation::Or {
                    destination: riscv_reg_to_ga_operand(rd),
                    operand1: riscv_reg_to_ga_operand(rs1),
                    operand2: riscv_reg_to_ga_operand(rs2),
                }];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::JAL { rd, imm } => {
                let operations = vec![
                    GAOperation::Move {
                        destination: riscv_reg_to_ga_operand(rd),
                        source: Operand::Register("PC".to_string()),
                    },
                    GAOperation::Add {
                        destination: Operand::Register("temp".to_string()),
                        operand1: Operand::Register("PC".to_string()),
                        operand2: Operand::Immidiate(DataWord::Word32(imm)),
                    },
                    GAOperation::ConditionalJump {
                        destination: Operand::Register("temp".to_string()),
                        condition: Condition::None,
                    },
                ];

                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::BNE { imm, rs1, rs2 } => {
                let operations = vec![
                    GAOperation::Sub {
                        destination: Operand::Register("temp".to_string()),
                        operand1: riscv_reg_to_ga_operand(rs1),
                        operand2: riscv_reg_to_ga_operand(rs2),
                    },
                    GAOperation::SetZFlag(Operand::Register("temp".to_string())),
                    GAOperation::Add {
                        destination: Operand::Register("temp".to_string()),
                        operand1: Operand::Register("PC".to_string()),
                        operand2: Operand::Immidiate(DataWord::Word32(imm as u32)),
                    },
                    GAOperation::Add {
                        destination: Operand::Register("temp".to_string()),
                        operand1: Operand::Register("temp".to_string()),
                        operand2: Operand::Immidiate(DataWord::Word32(-4i32 as u32)),
                    },
                    GAOperation::ConditionalJump {
                        destination: Operand::Register("temp".to_string()),
                        condition: Condition::NE,
                    },
                ];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::BLT { imm, rs1, rs2 } => {
                let operations = vec![
                    GAOperation::Sub {
                        destination: Operand::Register("temp".to_string()),
                        operand1: riscv_reg_to_ga_operand(rs1),
                        operand2: riscv_reg_to_ga_operand(rs2),
                    },
                    GAOperation::SetNFlag(Operand::Register("temp".to_string())),
                    GAOperation::Add {
                        destination: Operand::Register("temp".to_string()),
                        operand1: Operand::Register("PC".to_string()),
                        operand2: Operand::Immidiate(DataWord::Word32(imm as u32)),
                    },
                    GAOperation::Add {
                        destination: Operand::Register("temp".to_string()),
                        operand1: Operand::Register("temp".to_string()),
                        operand2: Operand::Immidiate(DataWord::Word32(-4i32 as u32)),
                    },
                    GAOperation::ConditionalJump {
                        destination: Operand::Register("temp".to_string()),
                        condition: Condition::MI,
                    },
                ];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::BGE { imm, rs1, rs2 } => {
                let operations = vec![
                    GAOperation::Sub {
                        destination: Operand::Register("temp".to_string()),
                        operand1: riscv_reg_to_ga_operand(rs1),
                        operand2: riscv_reg_to_ga_operand(rs2),
                    },
                    GAOperation::SetNFlag(Operand::Register("temp".to_string())),
                    //GAOperation::SetZFlag(Operand::Register("temp".to_string())),
                    GAOperation::Add {
                        destination: Operand::Register("temp".to_string()),
                        operand1: Operand::Register("PC".to_string()),
                        operand2: Operand::Immidiate(DataWord::Word32(imm as u32)),
                    },
                    GAOperation::Add {
                        destination: Operand::Register("temp".to_string()),
                        operand1: Operand::Register("temp".to_string()),
                        operand2: Operand::Immidiate(DataWord::Word32(-4i32 as u32)),
                    },
                    GAOperation::ConditionalJump {
                        destination: Operand::Register("temp".to_string()),
                        condition: Condition::GE,
                    },
                ];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::BLTU { imm, rs1, rs2 } => {
                let operations = vec![
                    GAOperation::Sub {
                        destination: Operand::Register("temp".to_string()),
                        operand1: riscv_reg_to_ga_operand(rs1),
                        operand2: riscv_reg_to_ga_operand(rs2),
                    },
                    GAOperation::SetCFlag {
                        operand1: riscv_reg_to_ga_operand(rs1),
                        operand2: riscv_reg_to_ga_operand(rs2),
                        sub: true,
                        carry: false,
                    },
                    GAOperation::SetZFlag(Operand::Register("temp".to_string())),
                    GAOperation::Add {
                        destination: Operand::Register("temp".to_string()),
                        operand1: Operand::Register("PC".to_string()),
                        operand2: Operand::Immidiate(DataWord::Word32(imm as u32)),
                    },
                    GAOperation::Add {
                        destination: Operand::Register("temp".to_string()),
                        operand1: Operand::Register("temp".to_string()),
                        operand2: Operand::Immidiate(DataWord::Word32(-4i32 as u32)),
                    },
                    GAOperation::ConditionalJump {
                        destination: Operand::Register("temp".to_string()),
                        condition: Condition::LTU,
                    },
                ];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::BGEU { imm, rs1, rs2 } => {
                let operations = vec![
                    GAOperation::Sub {
                        destination: Operand::Register("temp".to_string()),
                        operand1: riscv_reg_to_ga_operand(rs1),
                        operand2: riscv_reg_to_ga_operand(rs2),
                    },
                    GAOperation::SetCFlag {
                        operand1: riscv_reg_to_ga_operand(rs1),
                        operand2: riscv_reg_to_ga_operand(rs2),
                        sub: true,
                        carry: false,
                    },
                    GAOperation::SetZFlag(Operand::Register("temp".to_string())),
                    GAOperation::Add {
                        destination: Operand::Register("temp".to_string()),
                        operand1: Operand::Register("PC".to_string()),
                        operand2: Operand::Immidiate(DataWord::Word32(imm as u32)),
                    },
                    GAOperation::Add {
                        destination: Operand::Register("temp".to_string()),
                        operand1: Operand::Register("temp".to_string()),
                        operand2: Operand::Immidiate(DataWord::Word32(-4i32 as u32)),
                    },
                    GAOperation::ConditionalJump {
                        destination: Operand::Register("temp".to_string()),
                        condition: Condition::GEU,
                    },
                ];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::LBU { imm, rs1, rd } => {
                let operations = vec![
                    GAOperation::Move {
                        destination: riscv_reg_to_ga_operand(rd),
                        source: Operand::AddressWithOffset {
                            address: DataWord::Word32(imm as u32),
                            offset_reg: match riscv_reg_to_ga_operand(rs1) {
                                Operand::Register(s) => s,
                                _ => panic!(),
                            },
                            width: 8,
                        },
                    },
                    GAOperation::ZeroExtend {
                        destination: riscv_reg_to_ga_operand(rd),
                        operand: riscv_reg_to_ga_operand(rd),
                        bits: 32,
                    },
                ];

                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::LHU { imm, rs1, rd } => {
                let operations = vec![
                    GAOperation::Move {
                        destination: riscv_reg_to_ga_operand(rd),
                        source: Operand::AddressWithOffset {
                            address: DataWord::Word32(imm as u32),
                            offset_reg: match riscv_reg_to_ga_operand(rs1) {
                                Operand::Register(s) => s,
                                _ => panic!(),
                            },
                            width: 16,
                        },
                    },
                    GAOperation::ZeroExtend {
                        destination: riscv_reg_to_ga_operand(rd),
                        operand: riscv_reg_to_ga_operand(rd),
                        bits: 32,
                    },
                ];

                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::AUIPC { rd, imm } => {
                let operations = vec![
                    GAOperation::Add {
                        destination: riscv_reg_to_ga_operand(rd),
                        operand1: Operand::Register("PC".to_string()),
                        operand2: Operand::Immidiate(DataWord::Word32((imm as u32) << 12)),
                    },
                    GAOperation::Sub {
                        destination: riscv_reg_to_ga_operand(rd),
                        operand1: riscv_reg_to_ga_operand(rd),
                        operand2: Operand::Immidiate(DataWord::Word32(4)),
                    },
                ];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::JALR { rd, rs1, imm } => {
                let operations = vec![
                    GAOperation::Add {
                        destination: Operand::Register("temp".to_string()),
                        operand1: Operand::Register("PC".to_string()),
                        operand2: Operand::Immidiate(DataWord::Word32(0)),
                    },
                    GAOperation::Add {
                        destination: Operand::Register("temp2".to_string()),
                        operand1: riscv_reg_to_ga_operand(rs1),
                        operand2: Operand::Immidiate(DataWord::Word32(imm.into())),
                    },
                    // GAOperation::Sub {
                    //     destination: Operand::Register("temp2".to_string()),
                    //     operand1: Operand::Register("temp2".to_string()),
                    //      operand2: Operand::Immidiate(DataWord::Word32(4)), //negate PC increase
                    //  },
                    GAOperation::ConditionalJump {
                        destination: Operand::Register("temp2".to_string()),
                        condition: Condition::None,
                    },
                    GAOperation::Move {
                        destination: riscv_reg_to_ga_operand(rd),
                        source: Operand::Register("temp".to_string()),
                    },
                ];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::ADDI { imm, rs1, rd } => {
                //println!("ADDI {}", imm);
                let operations = vec![GAOperation::Add {
                    destination: riscv_reg_to_ga_operand(rd),
                    operand1: riscv_reg_to_ga_operand(rs1),
                    operand2: Operand::Immidiate(DataWord::Word32(imm as u32)),
                }];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::SLT { rs2, rs1, rd } => {
                let operations = vec![
                    GAOperation::Sub {
                        operand1: riscv_reg_to_ga_operand(rs1),
                        operand2: riscv_reg_to_ga_operand(rs2),
                        destination: Operand::Register("temp".to_string()),
                    },
                    GAOperation::SetVFlag {
                        operand1: riscv_reg_to_ga_operand(rs1),
                        operand2: riscv_reg_to_ga_operand(rs2),
                        sub: true,
                        carry: false,
                    },
                    GAOperation::SetNFlag(Operand::Register("temp".to_string())),
                    GAOperation::Move {
                        destination: Operand::Register("temp_N".to_string()),
                        source: Operand::Flag("N".to_string()),
                    },
                    GAOperation::Move {
                        destination: Operand::Register("temp_V".to_string()),
                        source: Operand::Flag("V".to_string()),
                    },
                    GAOperation::Xor {
                        destination: riscv_reg_to_ga_operand(rd),
                        operand1: Operand::Register("temp_N".to_string()),
                        operand2: Operand::Register("temp_V".to_string()),
                    },
                    GAOperation::ZeroExtend {
                        destination: riscv_reg_to_ga_operand(rd),
                        operand: riscv_reg_to_ga_operand(rd),
                        bits: 32,
                    },
                ];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::SLTU { rs2, rs1, rd } => {
                let operations = vec![
                    GAOperation::Sub {
                        operand1: riscv_reg_to_ga_operand(rs1),
                        operand2: riscv_reg_to_ga_operand(rs2),
                        destination: Operand::Register("temp".to_string()),
                    },
                    GAOperation::SetZFlag(Operand::Register("temp".to_string())),
                    GAOperation::SetCFlag {
                        carry: false,
                        operand1: riscv_reg_to_ga_operand(rs1),
                        operand2: riscv_reg_to_ga_operand(rs2),
                        sub: true,
                    },
                    GAOperation::Move {
                        destination: Operand::Register("temp_C".to_string()),
                        source: Operand::Flag("C".to_string()),
                    },
                    GAOperation::Move {
                        destination: Operand::Register("temp_Z".to_string()),
                        source: Operand::Flag("Z".to_string()),
                    },
                    GAOperation::Not {
                        destination: Operand::Register("temp_C".to_string()),
                        operand: Operand::Register("temp_C".to_string()),
                    },
                    GAOperation::Not {
                        destination: Operand::Register("temp_Z".to_string()),
                        operand: Operand::Register("temp_Z".to_string()),
                    },
                    GAOperation::And {
                        destination: riscv_reg_to_ga_operand(rd),
                        operand1: Operand::Register("temp_C".to_string()),
                        operand2: Operand::Register("temp_Z".to_string()),
                    }, //GAOperation:: { destination: riscv_reg_to_ga_operand(rd), operand1: Operand::Register("temp_N".to_string()), operand2: Operand::Register("temp_Z".to_string()) }
                    GAOperation::ZeroExtend {
                        destination: riscv_reg_to_ga_operand(rd),
                        operand: riscv_reg_to_ga_operand(rd),
                        bits: 32,
                    },
                ];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::SLTI { imm, rs1, rd } => {
                let operations = vec![
                    GAOperation::Sub {
                        operand1: riscv_reg_to_ga_operand(rs1),
                        operand2: Operand::Immidiate(DataWord::Word32(imm as u32)),
                        destination: Operand::Register("temp".to_string()),
                    },
                    GAOperation::SetVFlag {
                        operand1: riscv_reg_to_ga_operand(rs1),
                        operand2: Operand::Immidiate(DataWord::Word32(imm as u32)),
                        sub: true,
                        carry: false,
                    },
                    GAOperation::SetNFlag(Operand::Register("temp".to_string())),
                    GAOperation::Move {
                        destination: Operand::Register("temp_N".to_string()),
                        source: Operand::Flag("N".to_string()),
                    },
                    GAOperation::Move {
                        destination: Operand::Register("temp_V".to_string()),
                        source: Operand::Flag("V".to_string()),
                    },
                    GAOperation::Xor {
                        destination: riscv_reg_to_ga_operand(rd),
                        operand1: Operand::Register("temp_N".to_string()),
                        operand2: Operand::Register("temp_V".to_string()),
                    },
                    GAOperation::ZeroExtend {
                        destination: riscv_reg_to_ga_operand(rd),
                        operand: riscv_reg_to_ga_operand(rd),
                        bits: 32,
                    },
                ];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::SLTIU { imm, rs1, rd } => {
                let operations = vec![
                    GAOperation::Sub {
                        operand1: riscv_reg_to_ga_operand(rs1),
                        operand2: Operand::Immidiate(DataWord::Word32(imm as u32)),
                        destination: Operand::Register("temp".to_string()),
                    },
                    GAOperation::SetZFlag(Operand::Register("temp".to_string())),
                    GAOperation::SetCFlag {
                        carry: false,
                        operand1: riscv_reg_to_ga_operand(rs1),
                        operand2: Operand::Immidiate(DataWord::Word32(imm as u32)),
                        sub: true,
                    },
                    GAOperation::Move {
                        destination: Operand::Register("temp_C".to_string()),
                        source: Operand::Flag("C".to_string()),
                    },
                    GAOperation::Move {
                        destination: Operand::Register("temp_Z".to_string()),
                        source: Operand::Flag("Z".to_string()),
                    },
                    GAOperation::Not {
                        destination: Operand::Register("temp_C".to_string()),
                        operand: Operand::Register("temp_C".to_string()),
                    },
                    GAOperation::Not {
                        destination: Operand::Register("temp_Z".to_string()),
                        operand: Operand::Register("temp_Z".to_string()),
                    },
                    GAOperation::And {
                        destination: riscv_reg_to_ga_operand(rd),
                        operand1: Operand::Register("temp_C".to_string()),
                        operand2: Operand::Register("temp_Z".to_string()),
                    }, //GAOperation:: { destination: riscv_reg_to_ga_operand(rd), operand1: Operand::Register("temp_N".to_string()), operand2: Operand::Register("temp_Z".to_string()) }
                    GAOperation::ZeroExtend {
                        destination: riscv_reg_to_ga_operand(rd),
                        operand: riscv_reg_to_ga_operand(rd),
                        bits: 32,
                    },
                ];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::XORI { imm, rs1, rd } => {
                let operations = vec![GAOperation::Xor {
                    destination: riscv_reg_to_ga_operand(rd),
                    operand1: riscv_reg_to_ga_operand(rs1),
                    operand2: Operand::Immidiate(DataWord::Word32(imm as u32)),
                }];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::XOR { rs2, rs1, rd } => {
                let operations = vec![GAOperation::Xor {
                    destination: riscv_reg_to_ga_operand(rd),
                    operand1: riscv_reg_to_ga_operand(rs1),
                    operand2: riscv_reg_to_ga_operand(rs2),
                }];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::SLL { rs2, rs1, rd } => {
                let operations = vec![GAOperation::Sl {
                    destination: riscv_reg_to_ga_operand(rd),
                    operand: riscv_reg_to_ga_operand(rs1),
                    shift: riscv_reg_to_ga_operand(rs2),
                }];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::SRL { rs2, rs1, rd } => {
                let operations = vec![GAOperation::Srl {
                    destination: riscv_reg_to_ga_operand(rd),
                    operand: riscv_reg_to_ga_operand(rs1),
                    shift: riscv_reg_to_ga_operand(rs2),
                }];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::SRA { rs2, rs1, rd } => {
                let operations = vec![GAOperation::Sra {
                    destination: riscv_reg_to_ga_operand(rd),
                    operand: riscv_reg_to_ga_operand(rs1),
                    shift: riscv_reg_to_ga_operand(rs2),
                }];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::ORI { imm, rs1, rd } => {
                let operations = vec![GAOperation::Or {
                    destination: riscv_reg_to_ga_operand(rd),
                    operand1: riscv_reg_to_ga_operand(rs1),
                    operand2: Operand::Immidiate(DataWord::Word32(imm as u32)),
                }];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::ANDI { imm, rs1, rd } => {
                let operations = vec![GAOperation::And {
                    destination: riscv_reg_to_ga_operand(rd),
                    operand1: riscv_reg_to_ga_operand(rs1),
                    operand2: Operand::Immidiate(DataWord::Word32(imm as u32)),
                }];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::SLLI { shamt, rs1, rd } => {
                let operations = vec![GAOperation::Sl {
                    destination: riscv_reg_to_ga_operand(rd),
                    operand: riscv_reg_to_ga_operand(rs1),
                    shift: Operand::Immidiate(DataWord::Word8(shamt)),
                }];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::SRLI { shamt, rs1, rd } => {
                let operations = vec![GAOperation::Srl {
                    destination: riscv_reg_to_ga_operand(rd),
                    operand: riscv_reg_to_ga_operand(rs1),
                    shift: Operand::Immidiate(DataWord::Word8(shamt)),
                }];

                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::SRAI { shamt, rs1, rd } => {
                let operations = vec![GAOperation::Sra {
                    destination: riscv_reg_to_ga_operand(rd),
                    operand: riscv_reg_to_ga_operand(rs1),
                    shift: Operand::Immidiate(DataWord::Word8(shamt)),
                }];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::MRET {} => {
                let operations = vec![GAOperation::Nop];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::FENCE {} => {
                let operations = vec![GAOperation::Nop];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::FENCE_I {} => {
                let operations = vec![GAOperation::Nop];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::ECALL {} => {
                let operations = vec![GAOperation::Nop];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::CSRRW { csr, rs1, rd } => {
                //these should not be nops maybe mov to
                //register CSR+1k or something
                let operations = vec![
                    GAOperation::Move {
                        destination: riscv_reg_to_ga_operand(rd),
                        source: Operand::Register(format!("{}", csr)),
                    },
                    GAOperation::Move {
                        destination: Operand::Register(format!("{}", csr)),
                        source: riscv_reg_to_ga_operand(rs1),
                    },
                ];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::EBREAK {} => {
                let operations = vec![GAOperation::Nop];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::CSRRS { csr, rs1, rd } => {
                let operations = vec![
                    GAOperation::Move {
                        destination: riscv_reg_to_ga_operand(rd),
                        source: Operand::Register(format!("{}", csr)),
                    },
                    GAOperation::Move {
                        destination: Operand::Register("temp".to_string()),
                        source: Operand::Register(format!("{}", csr)),
                    },
                    GAOperation::Or {
                        destination: Operand::Register(format!("{}", csr)),
                        operand1: riscv_reg_to_ga_operand(rs1),
                        operand2: Operand::Register("temp".to_string()),
                    },
                ];

                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::CSRRC { csr, rs1, rd } => {
                let operations = vec![
                    GAOperation::Move {
                        destination: riscv_reg_to_ga_operand(rd),
                        source: Operand::Register(format!("{}", csr)),
                    },
                    GAOperation::Not {
                        destination: Operand::Register("temp".to_string()),
                        operand: riscv_reg_to_ga_operand(rs1),
                    },
                    GAOperation::And {
                        destination: Operand::Register(format!("{}", csr)),
                        operand1: riscv_reg_to_ga_operand(rd),
                        operand2: Operand::Register("temp".to_string()),
                    },
                ];

                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::CSRRWI { csr, zimm, rd } => {
                let operations = vec![
                    GAOperation::Move {
                        destination: riscv_reg_to_ga_operand(rd),
                        source: Operand::Register(format!("{}", csr)),
                    },
                    GAOperation::Move {
                        destination: Operand::Register(format!("{}", csr)),
                        source: Operand::Immidiate(DataWord::Word8(zimm)),
                    },
                ];
                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::CSRRSI { csr, zimm, rd } => {
                let operations = vec![
                    GAOperation::Move {
                        destination: riscv_reg_to_ga_operand(rd),
                        source: Operand::Register(format!("{}", csr)),
                    },
                    GAOperation::Move {
                        destination: Operand::Register("temp".to_string()),
                        source: Operand::Register(format!("{}", csr)),
                    },
                    GAOperation::Or {
                        destination: Operand::Register(format!("{}", csr)),
                        operand1: Operand::Immidiate(DataWord::Word8(zimm)),
                        operand2: Operand::Register("temp".to_string()),
                    },
                ];

                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::CSRRCI { csr, zimm, rd } => {
                let operations = vec![
                    GAOperation::Move {
                        destination: riscv_reg_to_ga_operand(rd),
                        source: Operand::Register(format!("{}", csr)),
                    },
                    GAOperation::Not {
                        destination: Operand::Register("temp".to_string()),
                        operand: Operand::Immidiate(DataWord::Word8(zimm)),
                    },
                    GAOperation::And {
                        destination: Operand::Register(format!("{}", csr)),
                        operand1: riscv_reg_to_ga_operand(rd),
                        operand2: Operand::Register("temp".to_string()),
                    },
                ];

                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            }
            Operation::AND { rs2, rs1, rd } => {
                let operations = vec![GAOperation::And {
                    destination: riscv_reg_to_ga_operand(rd),
                    operand1: riscv_reg_to_ga_operand(rs1),
                    operand2: riscv_reg_to_ga_operand(rs2),
                }];

                GAInstruction {
                    instruction_size: 32,
                    operations,
                    max_cycle: CycleCount::Value(1),
                }
            } //_ => {
              //    todo!()
              //}
        }
    }
    fn add_hooks(cfg: &mut RunConfig) {
        let symbolic_sized = |state: &mut GAState| {
            let value_ptr = state.get_register("A0".to_owned())?;
            let size = 32;
            trace!(
                "trying to create symbolic: addr: {:?}, size: {}",
                value_ptr,
                size
            );
            let name = "any".to_owned() + &state.marked_symbolic.len().to_string();
            let symb_value = state.ctx.unconstrained(size as u32, &name);
            state.marked_symbolic.push(Variable {
                name: Some(name),
                value: symb_value.clone(),
                ty: ExpressionType::Integer(size as usize),
            });
            state.memory.write(&value_ptr, symb_value)?;

            let lr = state.get_register("RA".to_owned())?;
            state.set_register("PC".to_owned(), lr)?;
            Ok(())
        };

        cfg.pc_hooks.push((
            Regex::new(r"^symbolic_size<.+>$").unwrap(),
            PCHook::Intrinsic(symbolic_sized),
        ));
    }
}
