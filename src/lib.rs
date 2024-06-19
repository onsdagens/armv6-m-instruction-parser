//! Library to parse ARMv6-M thumb instructions.
//!
//! Provides a enum with all instructions, register types and a function to parse binary representation into the enum with proper arguments.
//!
//! # Example
//! ```
//! # use riscv_instruction_parser::parse;
//! # fn main() {
//! #   let program_memory = [0xb0, 0xb5, 0xaf, 0x02];
//!     match parse(&program_memory[0..4]) {
//!         Ok(instruction) => println!("Instruction: {:?}", instruction),
//!         Err(_) => println!("Not a valid instruction.")
//!     }
//! # }
//! ```

pub mod conditions;
pub mod instructons;
pub mod registers;

use instructons::*;

/// This function parses a input byte slice into one instruction.
/// Returns Err(&str) if instruction is invalid.
pub fn parse(input: &[u8]) -> Result<Instruction, String> {
    Ok(Instruction {
        width: InstructionWidth::Bit32,
        operation: parse_32bit_operation(<u32>::from_le_bytes([
            input[0], input[1], input[2], input[3],
        ]))?,
    })
}
use bitutils::sign_extend32;
fn parse_32bit_operation(instruction: u32) -> Result<Operation, String> {
    let opcode = instruction & 0b1111111;
    let funct3 = (instruction & (0b111 << 12)) >> 12;
    let funct7 = (instruction & (0b1111111 << 25)) >> 25;
    let imm = sign_extend32(instruction >> 20, 12);
    let shamt = (instruction & (0b11111 << 20)) >> 20;
    let imm_big = instruction & 0xFFFFF000;
    let imm_big_shuffled = sign_extend32(
        (((instruction & (0b1 << 31)) >> (31 - 20))
            | ((instruction & (0b1111111111 << 21)) >> (30 - 10))
            | ((instruction & (0b1 << 20)) >> (20 - 11))
            | (instruction & (0b11111111 << 12)))
            & 0b1111_1111_1111_1111_1111_1111_1111_1110,
        21,
    );
    //no idea why this is encoded this way but the ISA is what it is
    let imm_store =
        ((instruction & (0b11111 << 7)) >> 7) | ((instruction & (0b1111111 << 25)) >> 20);
    match opcode {
        0b0110011 => {
            //OP                                         //rs1 [19:15] rs2 [24:20] rd [11:7]
            let rd = (((instruction & (0b11111 << 7)) >> 7) as u8).try_into()?;
            let rs1 = (((instruction & (0b11111 << 15)) >> 15) as u8).try_into()?;
            let rs2 = (((instruction & (0b11111 << 20)) >> 20) as u8).try_into()?;
            match funct3 {
                0b000 => {
                    // add/sub
                    match funct7 {
                        0b0000000 => Ok(Operation::ADD { rs1, rs2, rd }), //add
                        0b0100000 => Ok(Operation::SUB { rs1, rs2, rd }), //sub
                        _ => Err(format!("Invalid funct7 {:b}", funct7)),
                    }
                }
                0b001 => {
                    match funct7 {
                        // sll
                        0b0000000 => Ok(Operation::SLL { rs1, rs2, rd }), //sll
                        _ => Err(format!("Invalid funct7 {:b}", funct7)),
                    }
                }
                0b010 => {
                    match funct7 {
                        // slt
                        0b0000000 => Ok(Operation::SLT { rs1, rs2, rd }), //slt
                        _ => Err(format!("Invalid funct7 {:b}", funct7)),
                    }
                }
                0b011 => {
                    match funct7 {
                        // sltu
                        0b0000000 => Ok(Operation::SLTU { rs1, rs2, rd }), //sltu
                        _ => Err(format!("Invalid funct7 {:b}", funct7)),
                    }
                }
                0b100 => {
                    match funct7 {
                        // xor
                        0b0000000 => Ok(Operation::XOR { rs1, rs2, rd }), //xor
                        _ => Err(format!("Invalid funct7 {:b}", funct7)),
                    }
                }
                0b101 => {
                    match funct7 {
                        // srl
                        0b0000000 => Ok(Operation::SRL { rs1, rs2, rd }), //srl
                        0b0100000 => Ok(Operation::SRA { rs1, rs2, rd }), //sra
                        _ => Err(format!("Invalid funct7 {:b}", funct7)),
                    }
                }
                0b110 => {
                    match funct7 {
                        // or
                        0b0000000 => Ok(Operation::OR { rs1, rs2, rd }), //or
                        _ => Err(format!("Invalid funct7 {:b}", funct7)),
                    }
                }
                0b111 => {
                    //and
                    match funct7 {
                        0b0000000 => Ok(Operation::AND { rs1, rs2, rd }), //and
                        _ => Err(format!("Invalid funct7 {:b}", funct7)),
                    }
                }
                _ => Err(format!("Invalid funct3 {:b}", funct3)),
            }
        }
        0b0010011 => {
            //OP_IMM
            let rd = (((instruction & (0b11111 << 7)) >> 7) as u8).try_into()?;
            let rs1 = (((instruction & (0b11111 << 15)) >> 15) as u8).try_into()?;
            //trace!("opcode=OP_IMM");
            match funct3 {
                0b000 => {
                    //ADDI
                    Ok(Operation::ADDI {
                        imm: imm as i32,
                        rs1,
                        rd,
                    })
                }
                0b010 => {
                    //SLTI
                    Ok(Operation::SLTI {
                        imm: imm as u16,
                        rs1,
                        rd,
                    })
                }
                0b011 => {
                    //SLTIU
                    Ok(Operation::SLTIU {
                        imm: imm as u16,
                        rs1,
                        rd,
                    })
                }
                0b100 => {
                    //XORI
                    Ok(Operation::XORI {
                        imm: imm as u16,
                        rs1,
                        rd,
                    })
                }
                0b110 => {
                    //ORI
                    Ok(Operation::ORI {
                        imm: imm as u16,
                        rs1,
                        rd,
                    })
                }
                0b111 => {
                    //ANDI
                    Ok(Operation::ANDI {
                        imm: imm as u16,
                        rs1,
                        rd,
                    })
                }
                0b001 => {
                    //SLLI
                    Ok(Operation::SLLI {
                        shamt: shamt as u8,
                        rs1,
                        rd,
                    })
                }
                0b101 => {
                    //SRLI SRAI
                    match funct7 {
                        0b0000000 => Ok(Operation::SRLI {
                            shamt: shamt as u8,
                            rs1,
                            rd,
                        }), //SRLI
                        0b0100000 => Ok(Operation::SRAI {
                            shamt: shamt as u8,
                            rs1,
                            rd,
                        }), //SRAI
                        _ => Err(format!("Invalid funct7 {:b}", funct7)),
                    }
                }
                _ => Err(format!("Invalid funct3 {:b}", funct3)),
            }
        }
        0b0110111 => {
            //LUI
            let rd = (((instruction & (0b11111 << 7)) >> 7) as u8).try_into()?;
            let imm = imm_big;
            Ok(Operation::LUI { rd, imm })
        }
        0b0010111 => {
            //AUIPC
            let rd = (((instruction & (0b11111 << 7)) >> 7) as u8).try_into()?;
            let imm = imm_big;
            Ok(Operation::AUIPC { rd, imm })
        }
        0b1101111 => {
            //JAL
            let rd = (((instruction & (0b11111 << 7)) >> 7) as u8).try_into()?;
            let imm = imm_big_shuffled;
            Ok(Operation::JAL {
                rd,
                imm: imm as u32,
            })
        }
        0b1100111 => {
            //JALR
            let rd = (((instruction & (0b11111 << 7)) >> 7) as u8).try_into()?;
            let rs1 = (((instruction & (0b11111 << 15)) >> 15) as u8).try_into()?;
            let jalr_imm = imm;
            Ok(Operation::JALR {
                rd,
                rs1,
                imm: jalr_imm as u32,
            })
        }
        0b1100011 => {
            //BRANCH
            let rd = (((instruction & (0b11111 << 7)) >> 7) as u8)
                .try_into()
                .unwrap();
            let rs1 = (((instruction & (0b11111 << 15)) >> 15) as u8)
                .try_into()
                .unwrap();
            let rs2 = (((instruction & (0b11111 << 20)) >> 20) as u8)
                .try_into()
                .unwrap();
            let imm = sign_extend32(
                ((instruction & (0b1 << 31)) >> 19)
                    | ((instruction & (0b111111 << 25)) >> 20)
                    | ((instruction & (0b1111 << 8)) >> 7)
                    | ((instruction & (0b1 << 7)) << 4),
                13,
            );
            match funct3 {
                0b000 => {
                    //beq
                    Ok(Operation::BEQ {
                        imm: imm as u32,
                        rs1,
                        rs2,
                    })
                } //beq
                0b001 => Ok(Operation::BNE {
                    imm: imm as u32,
                    rs1,
                    rs2,
                }), //bne
                0b100 => Ok(Operation::BLT {
                    imm: imm as u32,
                    rs1,
                    rs2,
                }), //blt
                0b101 => Ok(Operation::BGE {
                    imm: imm as u32,
                    rs1,
                    rs2,
                }), //bge
                0b110 => Ok(Operation::BLTU {
                    imm: imm as u32,
                    rs1,
                    rs2,
                }), //bltu
                0b111 => Ok(Operation::BGEU {
                    imm: imm as u32,
                    rs1,
                    rs2,
                }), //bgeu
                0b011 => Ok(Operation::JALR {
                    rd,
                    rs1,
                    imm: imm as u32,
                }), //jalr
                0b010 => Ok(Operation::JAL {
                    rd,
                    imm: imm as u32,
                }),
                _ => {
                    unreachable!()
                }
            }
            /* let branch_imm = (((instruction & (0b1 << 31)) >> 19)
            | ((instruction & (0b111111 << 25)) >> 20)
            | ((instruction & (0b1111 << 8)) >> 7)
            | ((instruction & (0b1 << 7)) << 4))
            .into();*/
            //todo!()
        }

        0b0000011 => {
            //LOAD
            //let rd = (((instruction & (0b11111 << 7)) >> 7) as u8).try_into().unwrap();
            //let rs1 = (((instruction & (0b11111 << 15)) >> 15) as u8).try_into().unwrap();

            let imm = imm as u16; //immediate
                                  //todo!();
            let rd = (((instruction & (0b11111 << 7)) >> 7) as u8)
                .try_into()
                .unwrap();
            let rs1 = (((instruction & (0b11111 << 15)) >> 15) as u8)
                .try_into()
                .unwrap();

            match funct3 {
                0b000 => Ok(Operation::LB { imm, rs1, rd }),  //lb
                0b001 => Ok(Operation::LH { imm, rs1, rd }),  //lh
                0b010 => Ok(Operation::LW { imm, rs1, rd }),  //lw
                0b100 => Ok(Operation::LBU { imm, rs1, rd }), //lbu
                0b101 => Ok(Operation::LHU { imm, rs1, rd }), //lhu
                _ => {
                    panic!("Unsupported funct3 {:b}", funct3)
                }
            }
        }
        0b0100011 => {
            //STORE

            let rs1 = (((instruction & (0b11111 << 15)) >> 15) as u8)
                .try_into()
                .unwrap();
            let rs2 = (((instruction & (0b11111 << 20)) >> 20) as u8)
                .try_into()
                .unwrap();

            let imm = imm_store as u16; //immediate store type
            match funct3 {
                //size
                0b000 => Ok(Operation::SB { imm, rs2, rs1 }),
                0b001 => Ok(Operation::SH { imm, rs2, rs1 }),
                0b010 => Ok(Operation::SW { imm, rs2, rs1 }),
                _ => panic!("Unsupported funct3 {:b}", funct3),
            }
        }
        0b1110011 => {
            //SYSTEM
            let rd = (((instruction & (0b11111 << 7)) >> 7) as u8).try_into()?;
            let rs1 = (((instruction & (0b11111 << 15)) >> 15) as u8).try_into()?;
            let csr = imm as u16; //imm
            if instruction == 807403635
            //mret, basically magic number
            {
                Ok(Operation::MRET {})
            } else {
                match funct3 {
                    0b001 => {
                        //CSRRW
                        Ok(Operation::CSRRW { csr, rs1, rd })
                    }
                    0b010 => {
                        //CSRRS
                        Ok(Operation::CSRRS { csr, rs1, rd })

                        //rs1
                    }
                    0b011 => {
                        //CSRRC
                        Ok(Operation::CSRRC { csr, rs1, rd })

                        //rs1
                    }
                    0b101 => {
                        //CSRRWI
                        let zimm = (instruction & (0b11111 << 15)) >> 15;
                        Ok(Operation::CSRRWI {
                            csr,
                            zimm: zimm as u8,
                            rd,
                        })
                        //zimm
                    }
                    0b110 => {
                        //CSRRSI
                        let zimm = (instruction & (0b11111 << 15)) >> 15;
                        Ok(Operation::CSRRSI {
                            csr,
                            zimm: zimm as u8,
                            rd,
                        })
                        //zimm
                    }
                    0b111 => {
                        //CSRRCI
                        let zimm = (instruction & (0b11111 << 15)) >> 15;
                        Ok(Operation::CSRRCI {
                            csr,
                            zimm: zimm as u8,
                            rd,
                        })
                        //zimm
                    }
                    _ => panic!("Unsupported funct3 {:b}", funct3),
                }
            }
        }
        _ => Err(format!("Invalid opcode! {:b}", opcode.clone())),
    }
}

/*trait SignExtend {
    fn sign_extend(&self, valid_bits: usize) -> u32;
}

impl SignExtend for u16 {
    fn sign_extend(&self, valid_bits: usize) -> u32 {
        let shift = 16 - valid_bits;
        ((((self << shift) as i16) >> shift) as i32) as u32
    }
}

impl SignExtend for u32 {
    fn sign_extend(&self, valid_bits: usize) -> u32 {
        let shift = 32 - valid_bits;
        (((self << shift) as i32) >> shift) as u32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sign_extend_u16() {
        assert_eq!(0xffffffff, 0x1u16.sign_extend(1));
        assert_eq!(0x1, 0x1u16.sign_extend(2));
        assert_eq!(0xfffffff9, 0x9u16.sign_extend(4));
        assert_eq!(0x00000009, 0x9u16.sign_extend(5));
    }

    #[test]
    fn sign_extend_u32() {
        assert_eq!(0xffffffff, 0x1u32.sign_extend(1));
        assert_eq!(0x1, 0x1u32.sign_extend(2));
        assert_eq!(0xfffffff9, 0x9u32.sign_extend(4));
        assert_eq!(0x00000009, 0x9u32.sign_extend(5));
    }
}*/
