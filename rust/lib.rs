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

#[derive(Debug)]
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

/* ------------------------------------------------------------------------------------------------
 * Functions
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Tests
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Benchmarks
 * --------------------------------------------------------------------------------------------- */

//TODO
