/*
 * File:    betterpd4diff.rs
 * Brief:   TODO
 *
 * Copyright (C) 2024 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * TODO longer description
 *
*/

/*!
 * TODO rustdoc for this file here
*/

/* ------------------------------------------------------------------------------------------------
 * Submodules
 * --------------------------------------------------------------------------------------------- */

//TODO (includes "mod ..." and "pub mod ...")

/* ------------------------------------------------------------------------------------------------
 * Uses
 * --------------------------------------------------------------------------------------------- */

use common::*;
use riscv_tools::*;

/* ------------------------------------------------------------------------------------------------
 * Macros
 * --------------------------------------------------------------------------------------------- */

//TODO (also pub(crate) use the_macro statements here too)

/* ------------------------------------------------------------------------------------------------
 * Constants
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Static Variables
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Types
 * --------------------------------------------------------------------------------------------- */

//TODO includes "type"-defs, structs, enums, unions, etc

/* ------------------------------------------------------------------------------------------------
 * Associated Functions and Methods
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Traits And Default Implementations
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Trait Implementations
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Functions
 * --------------------------------------------------------------------------------------------- */

fn main() {
    let golden = std::env::args().nth(1).expect("No golden trace file provided!");
    let test   = std::env::args().nth(2).expect("No test trace file provided!");

    let golden = ParsedLineIterator::from_path(golden).expect("Failed to open golden trace file!");
    let test   = ParsedLineIterator::from_path(test).expect("Failed to open test trace file!");

    compare(golden, test);
}

fn compare(golden: ParsedLineIterator, test: ParsedLineIterator) {
    let mut last_fetched_pc: Option<u32> = None;
    let mut last_fetched_instr: Option<Instruction> = None;
    for (ii, (g, t)) in golden.zip(test).enumerate() {
        //Common code for nicely printing errors
        let disassembly = last_fetched_instr.as_ref().map(|instr| disassemble(instr));
        let mut error_count_this_line = 0;
        let mut print_error = |msg: &str| {
            if error_count_this_line == 0 {
                println!("At least one error on line {}:", ii + 1);
                println!("  Golden:         {}", g);
                println!("  Yours:          {}", t);
                if let Some(disassembly) = disassembly.as_ref() {
                    println!("  Disassembly:    {}", disassembly);
                }
            }
            println!("  Error {}: {}", error_count_this_line, msg);
            error_count_this_line += 1;
        };

        match (g, t) {
            (ParsedLine::F{pc: g_pc, instr: g_instr}, ParsedLine::F{pc: t_pc, instr: t_instr}) => {
                last_fetched_pc     = Some(g_pc);
                last_fetched_instr  = Some(Instruction::from(g_instr));
                if g_pc != t_pc {
                    print_error("PCs do not match!");
                }

                if g_instr != t_instr {
                    print_error("Instructions do not match!");
                }
            },
            (ParsedLine::D{pc: g_pc, opcode: g_opcode, rd: g_rd, rs1: g_rs1, rs2: g_rs2, funct3: g_funct3, funct7: g_funct7, imm: g_imm, shamt: g_shamt},
            ParsedLine::D{pc: t_pc, opcode: t_opcode, rd: t_rd, rs1: t_rs1, rs2: t_rs2, funct3: t_funct3, funct7: t_funct7, imm: t_imm, shamt: t_shamt}) => {
                let last_fetched_pc     = last_fetched_pc.unwrap();
                let last_fetched_instr  = last_fetched_instr.as_ref().unwrap();

                if g_pc != t_pc {
                    print_error("PCs do not match!");
                }

                if g_opcode != t_opcode {
                    print_error("Opcodes do not match!");
                }

                if g_rd != t_rd {
                    print_error("RDs do not match!");
                }

                if g_rs1 != t_rs1 {
                    print_error("RS1s do not match!");
                }

                if g_rs2 != t_rs2 {
                    print_error("RS2s do not match!");
                }

                if g_funct3 != t_funct3 {
                    print_error("Funct3s do not match!");
                }

                if g_funct7 != t_funct7 {
                    print_error("Funct7s do not match!");
                }

                if g_imm != t_imm {
                    print_error("IMMs do not match!");
                }

                if g_shamt != t_shamt {
                    print_error("SHAMTs do not match!");
                }
            },
            (ParsedLine::R{addr_rs1: g_addr_rs1, addr_rs2: g_addr_rs2, data_rs1: g_data_rs1, data_rs2: g_data_rs2},
            ParsedLine::R{addr_rs1: t_addr_rs1, addr_rs2: t_addr_rs2, data_rs1: t_data_rs1, data_rs2: t_data_rs2}) => {
                let last_fetched_pc     = last_fetched_pc.unwrap();
                let last_fetched_instr  = last_fetched_instr.as_ref().unwrap();

                if g_addr_rs1 != t_addr_rs1 {
                    print_error("RS1 addresses do not match!");
                }

                if g_addr_rs2 != t_addr_rs2 {
                    print_error("RS2 addresses do not match!");
                }

                if g_data_rs1 != t_data_rs1 {
                    print_error("RS1 data does not match!");
                }

                if g_data_rs2 != t_data_rs2 {
                    print_error("RS2 data does not match!");
                }
            },
            (ParsedLine::E{pc: g_pc, alu_result: g_alu_result, branch_taken: g_branch_taken},
            ParsedLine::E{pc: t_pc, alu_result: t_alu_result, branch_taken: t_branch_taken}) => {
                let last_fetched_pc     = last_fetched_pc.unwrap();
                let last_fetched_instr  = last_fetched_instr.as_ref().unwrap();

                if g_pc != t_pc {
                    print_error("PCs do not match!");
                }

                if g_alu_result != t_alu_result {
                    print_error("ALU results do not match!");
                }

                if g_branch_taken != t_branch_taken {
                    print_error("Branch taken flags do not match!");
                }
            },
            (ParsedLine::M{pc: g_pc, addr: g_addr, read_not_write: g_read_not_write, access_size: g_access_size, memory_wdata: g_memory_wdata},
            ParsedLine::M{pc: t_pc, addr: t_addr, read_not_write: t_read_not_write, access_size: t_access_size, memory_wdata: t_memory_wdata}) => {
                let last_fetched_pc     = last_fetched_pc.unwrap();
                let last_fetched_instr  = last_fetched_instr.as_ref().unwrap();

                if g_pc != t_pc {
                    print_error("PCs do not match!");
                }

                if g_addr != t_addr {
                    print_error("Addresses do not match!");
                }

                if g_read_not_write != t_read_not_write {
                    print_error("Read/write flags do not match!");
                }
            },
            (ParsedLine::W{pc: g_pc, we: g_we, addr_rd: g_addr_rd, data_rd: g_data_rd},
            ParsedLine::W{pc: t_pc, we: t_we, addr_rd: t_addr_rd, data_rd: t_data_rd}) => {
                let last_fetched_pc     = last_fetched_pc.unwrap();
                let last_fetched_instr  = last_fetched_instr.as_ref().unwrap();

                if g_pc != t_pc {
                    print_error("PCs do not match!");
                }

                if g_we != t_we {
                    print_error("Write enable flags do not match!");
                }

                if g_addr_rd != t_addr_rd {
                    print_error("RD addresses do not match!");
                }

                if g_data_rd != t_data_rd {
                    print_error("RD data does not match!");
                }
            },
            _ => panic!("Mismatched lines! Golden: {:?}, Test: {:?}", g, t),
        }
    }
}

/* ------------------------------------------------------------------------------------------------
 * Tests
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Benchmarks
 * --------------------------------------------------------------------------------------------- */

//TODO
