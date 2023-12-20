//! Provides a instruction type and a enum with all operations and there arguments.

use crate::{
    //    conditions::Condition,
    registers::Register,
};

/// Struct describing an instruction.
#[derive(Debug)]
pub struct Instruction {
    pub width: InstructionWidth,
    pub operation: Operation,
}

/// Enum describing the with of the corresponding binary representation of the instruction.
#[derive(Debug)]
pub enum InstructionWidth {
    Bit32,
}

impl Instruction {
    /// To check if instruction width is 32 bits.
    pub fn is_32bit(&self) -> bool {
        matches!(self.width, InstructionWidth::Bit32)
    }
}

/// Describes operation i.e. what type of instruction it is.
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Operation {
    LUI {
        rd: Register,
        imm: u32,
    },
    AUIPC {
        rd: Register,
        imm: u32,
    },
    JAL {
        rd: Register,
        imm: u32,
    },
    JALR {
        rd: Register,
        rs1: Register,
        imm: u16,
    },
    BEQ {
        imm: u16,
        rs1: Register,
        rs2: Register,
    },
    BNE {
        imm: u16,
        rs1: Register,
        rs2: Register,
    },
    BLT {
        imm: u16,
        rs1: Register,
        rs2: Register,
    },
    BGE {
        imm: u16,
        rs1: Register,
        rs2: Register,
    },
    BLTU {
        imm: u16,
        rs1: Register,
        rs2: Register,
    },
    BGEU {
        imm: u16,
        rs1: Register,
        rs2: Register,
    },
    LB {
        imm: u16,
        rs1: Register,
        rd: Register,
    },
    LH {
        imm: u16,
        rs1: Register,
        rd: Register,
    },
    LW {
        imm: u16,
        rs1: Register,
        rd: Register,
    },
    LBU {
        imm: u16,
        rs1: Register,
        rd: Register,
    },
    LHU {
        imm: u16,
        rs1: Register,
        rd: Register,
    },
    SB {
        imm: u16,
        rs2: Register,
        rs1: Register,
    },
    SH {
        imm: u16,
        rs2: Register,
        rs1: Register,
    },
    SW {
        imm: u16,
        rs2: Register,
        rs1: Register,
    },
    ADDI {
        imm: u16,
        rs1: Register,
        rd: Register,
    },
    SLTI {
        imm: u16,
        rs1: Register,
        rd: Register,
    },
    SLTIU {
        imm: u16,
        rs1: Register,
        rd: Register,
    },
    XORI {
        imm: u16,
        rs1: Register,
        rd: Register,
    },
    ORI {
        imm: u16,
        rs1: Register,
        rd: Register,
    },
    ANDI {
        imm: u16,
        rs1: Register,
        rd: Register,
    },
    SLLI {
        shamt: u8,
        rs1: Register,
        rd: Register,
    },
    SRLI {
        shamt: u8,
        rs1: Register,
        rd: Register,
    },
    SRAI {
        shamt: u8,
        rs1: Register,
        rd: Register,
    },
    ADD {
        rs2: Register,
        rs1: Register,
        rd: Register,
    },
    SUB {
        rs2: Register,
        rs1: Register,
        rd: Register,
    },
    SLL {
        rs2: Register,
        rs1: Register,
        rd: Register,
    },
    SLT {
        rs2: Register,
        rs1: Register,
        rd: Register,
    },
    SLTU {
        rs2: Register,
        rs1: Register,
        rd: Register,
    },
    XOR {
        rs2: Register,
        rs1: Register,
        rd: Register,
    },
    SRL {
        rs2: Register,
        rs1: Register,
        rd: Register,
    },
    SRA {
        rs2: Register,
        rs1: Register,
        rd: Register,
    },
    OR {
        rs2: Register,
        rs1: Register,
        rd: Register,
    },
    AND {
        rs2: Register,
        rs1: Register,
        rd: Register,
    },
    FENCE {
        //fence is nop under syncrim
    },
    FENCE_I {
        //fence is nop under syncrim
    },
    ECALL {
        //magic number
    },
    EBREAK {
        //magic number
    },
    MRET {
        //magic number
    },
    CSRRW {
        csr: u16,
        rs1: Register,
        rd: Register,
    },
    CSRRS {
        csr: u16,
        rs1: Register,
        rd: Register,
    },
    CSRRC {
        csr: u16,
        rs1: Register,
        rd: Register,
    },
    CSRRWI {
        csr: u16,
        zimm: u8,
        rd: Register,
    },
    CSRRSI {
        csr: u16,
        zimm: u8,
        rd: Register,
    },
    CSRRCI {
        csr: u16,
        zimm: u8,
        rd: Register,
    },
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn instruction_size() {
        let instruction_32 = Instruction {
            width: InstructionWidth::Bit32,
            operation: Operation::ADD {
                rs2: Register::ZERO,
                rs1: Register::ZERO,
                rd: Register::ZERO,
            }, //nop,
        };
        assert_eq!(instruction_32.is_32bit(), true);
    }
}
