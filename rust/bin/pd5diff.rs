/*
 * File:    pd5diff.rs
 * Brief:   Intelligent correctness checker for ECE 320's PD5
 *
 * Copyright (C) 2024 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * The first argument is the golden trace file, and the second argument is your trace file.
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

const LOGO: &'static str = concat!("\x1b[1;36m", r"
           _ ____      _ _  __  __ 
 _ __   __| | ___|  __| (_)/ _|/ _|
| '_ \ / _` |___ \ / _` | | |_| |_ 
| |_) | (_| |___) | (_| | |  _|  _|
| .__/ \__,_|____/ \__,_|_|_| |_|  
|_|                                  for ECE 320
", "\x1b[0m");

/* ------------------------------------------------------------------------------------------------
 * Static Variables
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Types
 * --------------------------------------------------------------------------------------------- */

type Result<T> = std::result::Result<T, ()>;

#[derive(Default)]
struct StageState {
    pc:     u32,
    instr:  Option<Instruction>,
}

#[derive(Default)]
struct Pipeline {
    f:  StageState,
    d:  StageState,
    e:  StageState,
    m:  StageState,
    w:  StageState,
}

/* ------------------------------------------------------------------------------------------------
 * Associated Functions and Methods
 * --------------------------------------------------------------------------------------------- */

impl StageState {
    fn dis(&self) -> String {
        if let Some(instr_ref) = self.instr.as_ref() {
            format!("PC: {:08x} -> Inst: {}", self.pc, disassemble(instr_ref))
        } else {
            String::from("(Bubble)")
        }
    }

    fn is_bubble(&self) -> bool {
        self.instr.is_none()
    }
}

impl Pipeline {
    fn clock(&mut self) {
        self.w = std::mem::take(&mut self.m);
        self.m = std::mem::take(&mut self.e);
        self.e = std::mem::take(&mut self.d);
        self.d = std::mem::take(&mut self.f);
        self.f = StageState::default();
    }
}

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

fn main() -> std::process::ExitCode {
    println!("{}", LOGO);
    println!("pd5diff v{} by \x1b[1;35mJZJ\x1b[0m", env!("CARGO_PKG_VERSION"));

    let main_body_result = (|| {
        let (golden_path, test_path) = args()?;

        let golden_trace    = load_trace(golden_path)?;
        let test_trace      = load_trace(test_path)?;
        println!("\x1b[1;32mSuccessfully loaded both traces!\x1b[0m");

        println!("\x1b[1mComparing traces...\x1b[0m");
        let errors = compare(golden_trace, test_trace);

        if errors > 0 {
            println!("\x1b[1;31mFound {} errors!\x1b[0m", errors);
            Err(())
        } else {
            println!("\x1b[1;32mNo errors found!\x1b[0m");
            Ok(())
        }
    })();

    if let Err(()) = main_body_result {
        println!("\x1b[1;31mpd5diff encountered at least one error!\x1b[0m");
        std::process::ExitCode::FAILURE
    } else {
        println!("\x1b[1;32mpd5diff is exiting with success!\x1b[0m");
        std::process::ExitCode::SUCCESS
    }
}

fn args() -> Result<(String, String)> {
    let mut args = std::env::args();

    if args.len() != 3 {
        println!("\x1b[1;31mUsage: pd5diff path/to/golden_trace.trace path/to/your_trace.trace\x1b[0m");
        return Err(());
    }

    let golden_path = args.nth(1).ok_or(())?;
    let test_path   = args.next().ok_or(())?;

    println!("Path to golden trace: \x1b[1;33m{}\x1b[0m", golden_path);
    println!("Path to your trace:   \x1b[1;37m{}\x1b[0m", test_path);

    Ok((golden_path, test_path))
}

fn load_trace(path: impl AsRef<std::path::Path>) -> Result<ParsedLineIterator> {
    let iterator = ParsedLineIterator::from_path(path.as_ref());

    match iterator {
        Ok(iterator) => {
            Ok(iterator)
        },
        Err(e) => {
            println!("\x1b[1;31mError loading trace at path {}: {}\x1b[0m", path.as_ref().display(), e);
            Err(())
        }
    }
}

//Returns the number of errors
fn compare(golden: ParsedLineIterator, test: ParsedLineIterator) -> u32 {
    let mut total_error_count   = 0;
    let mut pipeline            = Pipeline::default();

    let lines_vec: Vec<(ParsedLine, ParsedLine)> = golden.zip(test).collect();
    let line_chunks = lines_vec.chunks(6);//[F], [D], [R], [E], [M], [W]

    for (chunk_num, chunk) in line_chunks.enumerate() {
        //Convenient aliases
        let chunk_num           = chunk_num + 1;
        let (g_fline, t_fline)  = chunk[0];
        let (g_dline, t_dline)  = chunk[1];
        let (g_rline, t_rline)  = chunk[2];
        let (g_eline, t_eline)  = chunk[3];
        let (g_mline, t_mline)  = chunk[4];
        let (g_wline, t_wline)  = chunk[5];


        //Error handling
        //TODO make it so Instruction implements Clone so we don't have to pass pipeline every time
        let mut chunk_error_count = 0;
        let mut print_error = |pipeline: &Pipeline, msg: &str| {
            if chunk_error_count == 0 {
                println!(
                    "At least one error on clock cycle #{} containing lines {} thru {}:",
                    chunk_num,
                    chunk_num * 6 - 5,
                    chunk_num * 6
                );
                println!("  \x1b[1;33mGolden\x1b[0m                                      | \x1b[1mYours\x1b[0m");
                println!("  \x1b[1;33m  {}\x1b[0m                     |   \x1b[1m{}\x1b[0m", g_fline, t_fline);
                println!("  \x1b[1;33m  {}\x1b[0m |   \x1b[1m{}\x1b[0m", g_dline, t_dline);
                println!("  \x1b[1;33m  {}\x1b[0m               |   \x1b[1m{}\x1b[0m", g_rline, t_rline);
                println!("  \x1b[1;33m  {}\x1b[0m                   |   \x1b[1m{}\x1b[0m", g_eline, t_eline);
                println!("  \x1b[1;33m  {}\x1b[0m        |   \x1b[1m{}\x1b[0m", g_mline, t_mline);
                println!("  \x1b[1;33m  {}\x1b[0m                |   \x1b[1m{}\x1b[0m", g_wline, t_wline);
                println!("  \x1b[1;33mGolden Disassembly:");
                println!("    \x1b[1;33m[F]     is processing: {}\x1b[0m", pipeline.f.dis());
                println!("    \x1b[1;33m[D]/[R] is processing: {}\x1b[0m", pipeline.d.dis());
                println!("    \x1b[1;33m[E]     is processing: {}\x1b[0m", pipeline.e.dis());
                println!("    \x1b[1;33m[M]     is processing: {}\x1b[0m", pipeline.m.dis());
                println!("    \x1b[1;33m[W]     is processing: {}\x1b[0m", pipeline.w.dis());
                println!("  \x1b[1;31mErrors:\x1b[0m");
            }
            chunk_error_count += 1;
            println!("    \x1b[1;31mError {}: {}\x1b[0m", chunk_error_count, msg);
        };

        //Pipeline updating logic

        //TODO squashing logic/etc here
        pipeline.clock();

        //Line checking below

        if let (ParsedLine::F{pc: g_pc, instr: g_instr}, ParsedLine::F{pc: t_pc, instr: t_instr}) = (g_fline, t_fline) {
            pipeline.f.pc = g_pc;
            if g_instr == 0 {//Likely the end of the trace
                pipeline.f.instr = None;
            } else {
                pipeline.f.instr = Some(Instruction::from(g_instr));
            }

            if g_pc != t_pc {
                print_error(&pipeline, "[F] PCs do not match!");
            }

            if g_instr != t_instr {
                print_error(&pipeline, "[F] Fetched instructions do not match!");
            }
        } else {
            print_error(&pipeline, "[F] Mismatched line types or bad traces! Something is VERY wrong!");
        }

        if !pipeline.d.is_bubble() {
            let instr = pipeline.d.instr.as_ref().unwrap();

            if let (
                ParsedLine::D{pc: g_pc, opcode: g_opcode, rd: g_rd, rs1: g_rs1, rs2: g_rs2, funct3: g_funct3, funct7: g_funct7, imm: g_imm, shamt: g_shamt},
                ParsedLine::D{pc: t_pc, opcode: t_opcode, rd: t_rd, rs1: t_rs1, rs2: t_rs2, funct3: t_funct3, funct7: t_funct7, imm: t_imm, shamt: t_shamt}
                ) = (g_dline, t_dline) {

                if g_pc != t_pc {
                    print_error(&pipeline, "[D] PCs do not match!");
                }

                if !instr.is_fence() {
                    if g_opcode != t_opcode {
                        print_error(&pipeline, "[D] Opcodes do not match!");
                    }
                }

                //We sometimes don't do comparisons if they are don't cares

                if let Some(jzj_rd) = instr.get_rd() {
                    if g_rd != t_rd {
                        print_error(&pipeline, "[D] RDs do not match!");
                    }
                    assert_eq!(g_rd, jzj_rd, "pd5diff bug or bad golden trace");
                }

                if let Some(jzj_rs1) = instr.get_rs1() {
                    if g_rs1 != t_rs1 {
                        print_error(&pipeline, "[D] RS1s do not match!");
                    }
                    assert_eq!(g_rs1, jzj_rs1, "pd5diff bug or bad golden trace");
                }

                if let Some(jzj_rs2) = instr.get_rs2() {
                    if g_rs2 != t_rs2 {
                        print_error(&pipeline, "[D] RS2s do not match!");
                    }
                    assert_eq!(g_rs2, jzj_rs2, "pd5diff bug or bad golden trace");
                }

                if let Some(jzj_funct3) = instr.get_funct3() {
                    if g_funct3 != t_funct3 {
                        print_error(&pipeline, "[D] Funct3s do not match!");
                    }
                    assert_eq!(g_funct3, jzj_funct3, "pd5diff bug or bad golden trace");
                }

                if let Some(jzj_funct7) = instr.get_funct7() {
                    if g_funct7 != t_funct7 {
                        print_error(&pipeline, "[D] Funct7s do not match!");
                    }
                    assert_eq!(g_funct7, jzj_funct7, "pd5diff bug or bad golden trace");
                }

                if let Some(jzj_imm) = instr.get_imm() {
                    if g_imm != t_imm {
                        print_error(&pipeline, "[D] IMMs do not match!");
                    }
                    assert_eq!(g_imm, jzj_imm as u32, "pd5diff bug or bad golden trace");
                }

                if let Some(jzj_shamt) = instr.get_shamt() {
                    if g_shamt != t_shamt {
                        print_error(&pipeline, "[D] SHAMTs do not match!");
                    }
                    assert_eq!(g_shamt, jzj_shamt, "pd5diff bug or bad golden trace");
                }
            } else {
                print_error(&pipeline, "[D] Mismatched line types or bad traces! Something is VERY wrong!");
            }

            if let (
                ParsedLine::R{addr_rs1: g_addr_rs1, addr_rs2: g_addr_rs2, data_rs1: g_data_rs1, data_rs2: g_data_rs2},
                ParsedLine::R{addr_rs1: t_addr_rs1, addr_rs2: t_addr_rs2, data_rs1: t_data_rs1, data_rs2: t_data_rs2}
                ) = (g_rline, t_rline) {

                if let Some(jzj_rs1) = instr.get_rs1() {
                    if g_addr_rs1 != t_addr_rs1 {
                        print_error(&pipeline, "[R] RS1 addresses do not match!");
                    }
                    assert_eq!(g_addr_rs1, jzj_rs1, "pd5diff bug or bad golden trace");

                    if g_data_rs1 != t_data_rs1 {
                        print_error(&pipeline, "[R] RS1 data does not match!");
                    }
                }

                if let Some(jzj_rs2) = instr.get_rs2() {
                    if g_addr_rs2 != t_addr_rs2 {
                        print_error(&pipeline, "[R] RS2 addresses do not match!");
                    }
                    assert_eq!(g_addr_rs2, jzj_rs2, "pd5diff bug or bad golden trace");

                    if g_data_rs2 != t_data_rs2 {
                        print_error(&pipeline, "[R] RS2 data does not match!");
                    }
                }
            } else {
                print_error(&pipeline, "[R] Mismatched line types or bad traces! Something is VERY wrong!");
            }
        }

        if !pipeline.e.is_bubble() {
            let instr = pipeline.e.instr.as_ref().unwrap();

            if let (
                ParsedLine::E{pc: g_pc, alu_result: g_alu_result, branch_taken: g_branch_taken},
                ParsedLine::E{pc: t_pc, alu_result: t_alu_result, branch_taken: t_branch_taken}
                ) = (g_eline, t_eline) {

                if g_pc != t_pc {
                    print_error(&pipeline, "[E] PCs do not match!");
                }

                if !instr.is_fence() && !instr.is_system() {
                    if g_alu_result != t_alu_result {
                        print_error(&pipeline, "[E] ALU results do not match!");
                    }
                }

                if instr.is_btype() {
                    if g_branch_taken != t_branch_taken {
                        print_error(&pipeline, "[E] Branch taken line does not match!");
                    }
                }
            } else {
                print_error(&pipeline, "[E] Mismatched line types or bad traces! Something is VERY wrong!");
            }
        }

        if !pipeline.m.is_bubble() {
            let instr = pipeline.m.instr.as_ref().unwrap();
            if let (
                ParsedLine::M{pc: g_pc, addr: g_addr, read_not_write: g_read_not_write, access_size: g_access_size, memory_wdata: g_memory_wdata},
                ParsedLine::M{pc: t_pc, addr: t_addr, read_not_write: t_read_not_write, access_size: t_access_size, memory_wdata: t_memory_wdata}
            ) = (g_mline, t_mline) {

                if g_pc != t_pc {
                    print_error(&pipeline, "[M] PCs do not match!");
                }

                if g_read_not_write != t_read_not_write {
                    print_error(&pipeline, "[M] Read-not-write line does not match!");
                }

                if instr.is_memory() {
                    if g_addr != t_addr {
                        print_error(&pipeline, "[M] Addresses do not match!");
                    }

                    if g_access_size != t_access_size {
                        print_error(&pipeline, "[M] Access sizes do not match!");
                    }
                }

                if instr.is_stype() {
                    if g_memory_wdata != t_memory_wdata {
                        print_error(&pipeline, "[M] Memory write data does not match!");
                    }
                }
            } else {
                print_error(&pipeline, "[M] Mismatched line types or bad traces! Something is VERY wrong!");
            }
        }

        if !pipeline.m.is_bubble() {
            let instr = pipeline.m.instr.as_ref().unwrap();
            if let (
                ParsedLine::W{pc: g_pc, we: g_we, addr_rd: g_addr_rd, data_rd: g_data_rd},
                ParsedLine::W{pc: t_pc, we: t_we, addr_rd: t_addr_rd, data_rd: t_data_rd}
                ) = (g_wline, t_wline) {

                if g_pc != t_pc {
                    print_error(&pipeline, "PCs do not match!");
                }

                if !instr.is_fence() {
                    if g_we != t_we {
                        print_error(&pipeline, "Write enable line does not match!");
                    }

                    if let Some(jzj_addr_rd) = instr.get_rd() {
                        if g_addr_rd != t_addr_rd {
                            print_error(&pipeline, "RD addresses do not match!");
                        }
                        assert_eq!(g_addr_rd, jzj_addr_rd, "pd5diff bug or bad golden trace");

                        if g_data_rd != t_data_rd {
                            print_error(&pipeline, "RD data does not match!");
                        }
                    }
                }
            } else {
                print_error(&pipeline, "[W] Mismatched line types or bad traces! Something is VERY wrong!");
            }
        }

        total_error_count += chunk_error_count;
    }

    total_error_count
}

/* ------------------------------------------------------------------------------------------------
 * Tests
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Benchmarks
 * --------------------------------------------------------------------------------------------- */

//TODO
