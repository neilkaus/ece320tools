/*
 * File:    lib.rs
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

use riscv_tools::*;

use std::path::Path;
use std::fs::*;
use std::io::{BufReader, BufRead, Lines};

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

#[derive(Copy, Clone, Debug)]
pub enum ParsedLine {
    F{
        pc:     u32,
        instr:  u32,
    },
    D{
        pc:     u32,
        opcode: u8,
        rd:     u8,
        rs1:    u8,
        rs2:    u8,
        funct3: u8,
        funct7: u8,
        imm:    u32,
        shamt:  u8,
    },
    R{
        addr_rs1:   u8,
        addr_rs2:   u8,
        data_rs1:   u32,
        data_rs2:   u32,
    },
    E{
        pc:             u32,
        alu_result:     u32,
        branch_taken:   bool,
    },
    M{
        pc:             u32,
        addr:           u32,
        read_not_write: bool,
        access_size:    u8,
        memory_wdata:   u32,
    },
    W{
        pc:         u32,
        we:         bool,
        addr_rd:    u8,
        data_rd:    u32,
    },
}

pub struct ParsedLineIterator {
    buffered_lines: Lines<BufReader<File>>,
}

/* ------------------------------------------------------------------------------------------------
 * Associated Functions and Methods
 * --------------------------------------------------------------------------------------------- */

impl ParsedLineIterator {
    pub fn from_path(path: impl AsRef<Path>) -> std::io::Result<ParsedLineIterator> {
        Ok(ParsedLineIterator {
            buffered_lines: BufReader::new(File::open(path)?).lines(),
        })
    }
}

/* ------------------------------------------------------------------------------------------------
 * Traits And Default Implementations
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Trait Implementations
 * --------------------------------------------------------------------------------------------- */

impl Iterator for ParsedLineIterator {
    type Item = ParsedLine;

    fn next(&mut self) -> Option<Self::Item> {
        Some((&*self.buffered_lines.next()?.ok()?).into())
    }
}

impl From<&str> for ParsedLine {
    fn from(s: &str) -> Self {
        let mut tokens = s.split_whitespace();
        match tokens.next().unwrap() {
            "[F]" => ParsedLine::F {
                pc:     u32::from_str_radix(tokens.next().unwrap(), 16).unwrap(),
                instr:  u32::from_str_radix(tokens.next().unwrap(), 16).unwrap(),
            },
            "[D]" => ParsedLine::D {
                pc:     u32::from_str_radix(tokens.next().unwrap(), 16).unwrap(),
                opcode: u8::from_str_radix (tokens.next().unwrap(), 16).unwrap(),
                rd:     u8::from_str_radix (tokens.next().unwrap(), 16).unwrap(),
                rs1:    u8::from_str_radix (tokens.next().unwrap(), 16).unwrap(),
                rs2:    u8::from_str_radix (tokens.next().unwrap(), 16).unwrap(),
                funct3: u8::from_str_radix (tokens.next().unwrap(), 16).unwrap(),
                funct7: u8::from_str_radix (tokens.next().unwrap(), 16).unwrap(),
                imm:    u32::from_str_radix(tokens.next().unwrap(), 16).unwrap(),
                shamt:  u8::from_str_radix (tokens.next().unwrap(), 16).unwrap(),
            },
            "[R]" => ParsedLine::R {
                addr_rs1:   u8::from_str_radix (tokens.next().unwrap(), 16).unwrap(),
                addr_rs2:   u8::from_str_radix (tokens.next().unwrap(), 16).unwrap(),
                data_rs1:   u32::from_str_radix(tokens.next().unwrap(), 16).unwrap(),
                data_rs2:   u32::from_str_radix(tokens.next().unwrap(), 16).unwrap(),
            },
            "[E]" => ParsedLine::E {
                pc:             u32::from_str_radix(tokens.next().unwrap(), 16).unwrap(),
                alu_result:     u32::from_str_radix(tokens.next().unwrap(), 16).unwrap(),
                branch_taken:   tokens.next().unwrap() == "1",
            },
            "[M]" => ParsedLine::M {
                pc:             u32::from_str_radix(tokens.next().unwrap(), 16).unwrap(),
                addr:           u32::from_str_radix(tokens.next().unwrap(), 16).unwrap(),
                read_not_write: tokens.next().unwrap() == "1",
                access_size:    u8::from_str_radix (tokens.next().unwrap(), 16).unwrap(),
                memory_wdata:   u32::from_str_radix(tokens.next().unwrap(), 16).unwrap(),
            },
            "[W]" => ParsedLine::W {
                pc:         u32::from_str_radix(tokens.next().unwrap(), 16).unwrap(),
                we:         tokens.next().unwrap() == "1",
                addr_rd:    u8::from_str_radix (tokens.next().unwrap(), 16).unwrap(),
                data_rd:    u32::from_str_radix(tokens.next().unwrap(), 16).unwrap(),
            },
            _ => panic!("Bad syntax on line: {}", s),
        }
    }
}

impl std::fmt::Display for ParsedLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsedLine::F{pc, instr} => write!(f, "[F] {:08x} {:08x}", pc, instr),
            ParsedLine::D{pc, opcode, rd, rs1, rs2, funct3, funct7, imm, shamt} => write!(f, "[D] {:08x} {:02x} {:02x} {:02x} {:02x} {:01x} {:02x} {:08x} {:02x}", pc, opcode, rd, rs1, rs2, funct3, funct7, imm, shamt),
            ParsedLine::R{addr_rs1, addr_rs2, data_rs1, data_rs2} => write!(f, "[R] {:02x} {:02x} {:08x} {:08x}", addr_rs1, addr_rs2, data_rs1, data_rs2),
            ParsedLine::E{pc, alu_result, branch_taken} => write!(f, "[E] {:08x} {:08x} {}", pc, alu_result, if *branch_taken {1} else {0}),
            ParsedLine::M{pc, addr, read_not_write, access_size, memory_wdata} => write!(f, "[M] {:08x} {:08x} {} {:01x} {:08x}", pc, addr, if *read_not_write {1} else {0}, access_size, memory_wdata),
            ParsedLine::W{pc, we, addr_rd, data_rd} => write!(f, "[W] {:08x} {} {:02x} {:08x}", pc, if *we {1} else {0}, addr_rd, data_rd),
        }
    }
}

/* ------------------------------------------------------------------------------------------------
 * Functions
 * --------------------------------------------------------------------------------------------- */

pub fn disassemble(instr: &Instruction) -> String {
    let mut buffer = Vec::new();
    instr.disassemble(&mut buffer).unwrap();
    String::from_utf8(buffer).unwrap().trim().to_string()
}

/* ------------------------------------------------------------------------------------------------
 * Tests
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Benchmarks
 * --------------------------------------------------------------------------------------------- */

//TODO
