/*
 * File:    pd6diff.rs
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

use std::fmt::Display;

/* ------------------------------------------------------------------------------------------------
 * Macros
 * --------------------------------------------------------------------------------------------- */

//TODO (also pub(crate) use the_macro statements here too)

/* ------------------------------------------------------------------------------------------------
 * Constants
 * --------------------------------------------------------------------------------------------- */

const LOGO: &'static str = concat!("\x1b[1;36m", r"
           _  __       _ _  __  __
 _ __   __| |/ /_   __| (_)/ _|/ _|
| '_ \ / _` | '_ \ / _` | | |_| |_
| |_) | (_| | (_) | (_| | |  _|  _|
| .__/ \__,_|\___/ \__,_|_|_| |_|
|_|                                  for ECE 320
", "\x1b[0m");

/* ------------------------------------------------------------------------------------------------
 * Static Variables
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Types
 * --------------------------------------------------------------------------------------------- */

type Result<T>  = std::result::Result<T, ()>;
type MaybeInstr = std::result::Result<Instruction, InstrNotPresentReason>;

#[derive(Debug)]
enum InstrNotPresentReason {
    Bubble,
    StallSoInstrWordNotAvailable,
}

struct StageState {
    pc:     u32,
    instr:  MaybeInstr,
}

#[derive(Default)]
struct Pipeline {
    f:  StageState,
    d:  StageState,
    e:  StageState,
    m:  StageState,
    w:  StageState,
}

enum Mode {
    Board,//pd6boarddiff
    Sim//pd6simdiff
}

/* ------------------------------------------------------------------------------------------------
 * Associated Functions and Methods
 * --------------------------------------------------------------------------------------------- */

impl StageState {
    fn dis(&self) -> String {
        match self.instr.as_ref() {
            Ok(instr_ref)                                               => format!("instruction @PC {:08x}: {:08x}: {}", self.pc, instr_ref.assume_uncompressed(), disassemble(instr_ref)),
            Err(InstrNotPresentReason::Bubble)                          => String::from("nothing (bubble)"),
            Err(InstrNotPresentReason::StallSoInstrWordNotAvailable)    => format!("instruction @PC {:08x}: ????????: instruction word not available from golden trace due to stall next cycle", self.pc),
        }
    }

    const fn is_bubble(&self) -> bool {
        matches!(self.instr, Err(InstrNotPresentReason::Bubble))
    }
}

impl Pipeline {
    fn dumb_advance(&mut self, f_pc: u32, f_instr: MaybeInstr) {
        self.w = std::mem::take(&mut self.m);
        self.m = std::mem::take(&mut self.e);
        self.e = std::mem::take(&mut self.d);
        self.d = std::mem::take(&mut self.f);
        self.f = StageState {
            pc:     f_pc,
            instr:  f_instr,
        };
    }

    fn dumb_advance_with_stalled_fetch_and_decode(&mut self) {
        self.w = std::mem::take(&mut self.m);
        self.m = std::mem::take(&mut self.e);
        self.e = StageState::default();
    }
}

impl Mode {
    fn get() -> Self {
        match env!("CARGO_BIN_NAME") {
            "pd6boarddiff"  => Self::Board,
            "pd6simdiff"    => Self::Sim,
            _               => panic!("pd6diff was not compiled as pd6boarddiff or pd6simdiff!"),
        }
    }
}

/* ------------------------------------------------------------------------------------------------
 * Traits And Default Implementations
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Trait Implementations
 * --------------------------------------------------------------------------------------------- */

impl Default for StageState {
    fn default() -> Self {
        Self {
            pc:     0,
            instr:  Err(InstrNotPresentReason::Bubble),
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Board => write!(f, "board"),
            Self::Sim   => write!(f, "sim"),
        }
    }
}

/* ------------------------------------------------------------------------------------------------
 * Functions
 * --------------------------------------------------------------------------------------------- */

fn main() -> std::process::ExitCode {
    println!("{}", LOGO);
    println!("pd6diff v{} by \x1b[1;35mJZJ :)\x1b[0m", env!("CARGO_PKG_VERSION"));
    println!("\x1b[1;94m\"From the ashes of disaster grow the roses of success!\"\x1b[0m");
    println!();

    let main_body_result = (|| {
        let mode = Mode::get();
        println!("Running in \x1b[1;36m{}\x1b[0m mode", mode);

        let (golden_path, test_path) = args()?;

        let golden_trace    = load_trace(golden_path)?;
        let test_trace      = load_trace(test_path)?;
        println!("\x1b[1;32mSuccessfully loaded both traces!\x1b[0m");

        println!("\x1b[1mComparing traces...\x1b[0m");
        let errors = match mode {
            Mode::Board => compare_board(golden_trace, test_trace),
            Mode::Sim   => compare_sim  (golden_trace, test_trace),
        };

        if errors > 0 {
            println!("\x1b[1;31mFound {} error(s)!\x1b[0m", errors);
            Err(())
        } else {
            println!("\x1b[1;32mNo errors found!\x1b[0m");
            Ok(())
        }
    })();

    if let Err(()) = main_body_result {
        println!("\x1b[1;31mpd6diff encountered at least one error!\x1b[0m");
        std::process::ExitCode::FAILURE
    } else {
        println!("\x1b[1;32mpd6diff is exiting with success! Nicely done! :)\x1b[0m");
        std::process::ExitCode::SUCCESS
    }
}

fn args() -> Result<(String, String)> {
    let mut args = std::env::args();

    if args.len() != 3 {
        println!("\x1b[1;31mUsage: pd6diff path/to/golden_trace.trace path/to/your_trace.trace\x1b[0m");
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
fn compare_board(_golden: ParsedLineIterator, _test: ParsedLineIterator) -> u32 {
    panic!("Board mode not yet implemented, need to handle only checking writeback!");
}

//Returns the number of errors
fn compare_sim(golden: ParsedLineIterator, test: ParsedLineIterator) -> u32 {
    let mut total_error_count   = 0;
    let mut pipeline            = Pipeline::default();

    //TODO for better performance, avoid collecting here (too bad array_chunks() is unstable)
    let lines_vec: Vec<(ParsedLine, ParsedLine)>    = golden.zip(test).collect();
    let line_chunks: Vec<_>                         = lines_vec.chunks(6).collect();//[F], [D], [R], [E], [M], [W]
    let line_chunks_windowed                        = line_chunks.windows(2);

    let mut squash_fetch_and_decode_next_cycle = false;

    for (window_num, chunk_window) in line_chunks_windowed.enumerate() {
        //Convenient aliases
        let window_num = window_num + 1;//Since enumerate() is zero-indexed
        let (g_fline, t_fline)  = chunk_window[0][0];
        let (g_dline, t_dline)  = chunk_window[0][1];
        let (g_rline, t_rline)  = chunk_window[0][2];
        let (g_eline, t_eline)  = chunk_window[0][3];
        let (g_mline, t_mline)  = chunk_window[0][4];
        let (g_wline, t_wline)  = chunk_window[0][5];

        let (g_fline_next, _t_fline_next)   = chunk_window[1][0];
        let (g_dline_next, _t_dline_next)   = chunk_window[1][1];
        let (g_rline_next, t_rline_next)    = chunk_window[1][2];
        let (g_eline_next, _t_eline_next)   = chunk_window[1][3];
        let (_g_mline_next, _t_mline_next)  = chunk_window[1][4];
        let (_g_wline_next, _t_wline_next)  = chunk_window[1][5];

        //////////////////////////////////////////////////////////////////////////////////////////////////////
        //Pipeline updating logic
        //////////////////////////////////////////////////////////////////////////////////////////////////////
        if squash_fetch_and_decode_next_cycle {
            pipeline.f = StageState::default();
            pipeline.d = StageState::default();
            squash_fetch_and_decode_next_cycle = false;
        }

        //Check if fetch and decode stalled, in which case we shouldn't touch fetch or decode and should squash execute
        let fetch_and_decode_stalled = if let (ParsedLine::D{pc: g_d_pc, ..}, ParsedLine::E{pc: g_e_pc, ..}) = (g_dline, g_eline) {
            //Don't check if E is a bubble because it could be we're stalling multiple cycles
            //We do need to check D though because if the PCs just happen to match but were squashed we're not actually stalling
            if !pipeline.d.is_bubble() {
                g_d_pc == g_e_pc
            } else {
                false
            }
        } else {
            println!("\x1b[1;31mWeirdness in golden trace, are your arguments to pd6diff correct?\x1b[0m");
            false
        };

        //Similar check for if the stall is happening next cycle
        let fetch_and_decode_stalled_next_cycle = if let (ParsedLine::D{pc: g_d_pc_next, ..}, ParsedLine::E{pc: g_e_pc_next, ..}) = (g_dline_next, g_eline_next) {
            if !pipeline.d.is_bubble() {
                g_d_pc_next == g_e_pc_next
            } else {
                false
            }
        } else {
            println!("\x1b[1;31mWeirdness in golden trace, are your arguments to pd6diff correct?\x1b[0m");
            false
        };

        if !fetch_and_decode_stalled_next_cycle {
            if let Err(InstrNotPresentReason::StallSoInstrWordNotAvailable) = pipeline.f.instr {
                //No longer stalled, need to populate the instruction now that we should have it
                //(in the next cycle we have it that is, which is within lookahead range)
                if let ParsedLine::F{instr: g_instr_next, ..} = g_fline_next {
                    pipeline.f.instr = Ok(g_instr_next.into());
                } else {
                    println!("\x1b[1;31mWeirdness in golden trace, are your arguments to pd6diff correct?\x1b[0m");
                }
            }
        }

        if fetch_and_decode_stalled {
            pipeline.dumb_advance_with_stalled_fetch_and_decode();
        } else if let (ParsedLine::F{pc: g_pc, ..}, ParsedLine::F{instr: g_instr_next, ..}) = (g_fline, g_fline_next) {
            //Need to look at the fetch stage a cycle in the future to get the instruction word
            //because in PD6, imemory has one cycle of additional latency.

            if g_pc == 0 {
                println!("PC in golden trace became 00000000, assuming we've reached the end!");
                println!("\x1b[90m(This is expected for simple-programs golden traces, since if you");
                println!("look at their assembly, when they return from main, since `ra` is initialized");
                println!("to 0 by our hardware, but never by their code, the PC naturally becomes 0.");
                println!("Technically a bug in their test programs, but it's a nice end-of-code flag for us!)\x1b[0m");

                break;
            }

            if fetch_and_decode_stalled_next_cycle {
                pipeline.dumb_advance(g_pc, Err(InstrNotPresentReason::StallSoInstrWordNotAvailable));
            } else {
                if g_instr_next == 0 {
                    println!("Encountered illegal instruction in golden trace, assuming we've reached the end!");
                    println!("\x1b[90m(This is expected for individual-instruction golden traces, since we simply");
                    println!("implement ecall as a NOP, and since these traces end in an ecall, we thus run");
                    println!("into the data afterwards in memory, interpreting it as an instruction)\x1b[0m");
                    break;
                }

                pipeline.dumb_advance(g_pc, Ok(g_instr_next.into()));
            }
        } else {
            println!("\x1b[1;31mWeirdness in golden trace, are your arguments to pd6diff correct?\x1b[0m");
        }

        //If execute is processing a branch and the branch taken flag is set, or this is an
        //unconditional jump, squash fetch and decode next cycle
        if let Ok(instr) = pipeline.e.instr.as_ref() {
            if instr.is_btype() {
                if let ParsedLine::E{branch_taken: g_branch_taken, ..} = g_eline {
                    //It seems that branch taken in their traces for PD5 is now also set for
                    //unconditional branches? Weird that that wasn't the case for PD4...
                    squash_fetch_and_decode_next_cycle = g_branch_taken;
                }
            } else if instr.is_uncond_jump() {
                squash_fetch_and_decode_next_cycle = true;
            }
        }

        //////////////////////////////////////////////////////////////////////////////////////////////////////
        //Error handling used by line checking below
        //////////////////////////////////////////////////////////////////////////////////////////////////////
        let mut chunk_error_count = 0;
        let mut print_error = |message| {
            if chunk_error_count == 0 {
                println!(
                    "At least one error on clock cycle #{} containing lines {} thru {} (inclusive):",
                    window_num,
                    window_num * 6 - 5,
                    window_num * 6
                );
                println!("  \x1b[1;33mGolden\x1b[0m                                      | \x1b[1mYours\x1b[0m");
                println!("  \x1b[1;33m  {}\x1b[0m                     |   \x1b[1m{}\x1b[0m", g_fline, t_fline);
                println!("  \x1b[1;33m  {}\x1b[0m |   \x1b[1m{}\x1b[0m", g_dline, t_dline);
                println!("  \x1b[1;33m  {}\x1b[0m               |   \x1b[1m{}\x1b[0m", g_rline, t_rline);
                println!("  \x1b[1;33m  {}\x1b[0m                   |   \x1b[1m{}\x1b[0m", g_eline, t_eline);
                println!("  \x1b[1;33m  {}\x1b[0m        |   \x1b[1m{}\x1b[0m", g_mline, t_mline);
                println!("  \x1b[1;33m  {}\x1b[0m                |   \x1b[1m{}\x1b[0m", g_wline, t_wline);
                println!("  \x1b[1;33mGolden Disassembly:");
                println!("    \x1b[1;33m[F]     is processing {}\x1b[0m", pipeline.f.dis());
                println!("    \x1b[1;33m[D]/[R] is processing {}\x1b[0m", pipeline.d.dis());
                println!("    \x1b[1;33m[E]     is processing {}\x1b[0m", pipeline.e.dis());
                println!("    \x1b[1;33m[M]     is processing {}\x1b[0m", pipeline.m.dis());
                println!("    \x1b[1;33m[W]     is processing {}\x1b[0m", pipeline.w.dis());
                println!("  \x1b[1;31mError(s):\x1b[0m");
            }
            chunk_error_count += 1;
            println!("    \x1b[1;31mError {}: {}\x1b[0m", chunk_error_count, message);
        };

        //////////////////////////////////////////////////////////////////////////////////////////////////////
        //[F] Line Checking
        //////////////////////////////////////////////////////////////////////////////////////////////////////
        if let (ParsedLine::F{pc: g_pc, instr: g_instr}, ParsedLine::F{pc: t_pc, instr: t_instr}) = (g_fline, t_fline) {
            if g_pc != t_pc {
                print_error("[F] PCs do not match!");
            }
            assert_eq!(g_pc, pipeline.f.pc, "pd6diff bug or bad golden trace");

            if g_instr != t_instr {
                print_error("[F] Fetched instructions do not match!");
            }
        } else {
            print_error("[F] Mismatched line types or bad traces! Something is VERY wrong!");
        }

        //////////////////////////////////////////////////////////////////////////////////////////////////////
        //[D] and [R] Line Checking
        //////////////////////////////////////////////////////////////////////////////////////////////////////
        if !pipeline.d.is_bubble() {
            let instr = pipeline.d.instr.as_ref().unwrap();

            if let (
                ParsedLine::D{pc: g_pc, opcode: g_opcode, rd: g_rd, rs1: g_rs1, rs2: g_rs2, funct3: g_funct3, funct7: g_funct7, imm: g_imm, shamt: g_shamt},
                ParsedLine::D{pc: t_pc, opcode: t_opcode, rd: t_rd, rs1: t_rs1, rs2: t_rs2, funct3: t_funct3, funct7: t_funct7, imm: t_imm, shamt: t_shamt}
            ) = (g_dline, t_dline) {
                if g_pc != t_pc {
                    print_error("[D] PCs do not match!");
                }
                assert_eq!(g_pc, pipeline.d.pc, "pd6diff bug or bad golden trace");

                if !instr.is_fence() {
                    if g_opcode != t_opcode {
                        print_error("[D] Opcodes do not match!");
                    }
                }

                //We sometimes don't do comparisons if they are don't cares

                if let Some(jzj_rd) = instr.get_rd() {
                    if g_rd != t_rd {
                        print_error("[D] RDs do not match!");
                    }
                    assert_eq!(g_rd, jzj_rd, "pd6diff bug or bad golden trace");
                }

                if let Some(jzj_rs1) = instr.get_rs1() {
                    if g_rs1 != t_rs1 {
                        print_error("[D] RS1s do not match!");
                    }
                    assert_eq!(g_rs1, jzj_rs1, "pd6diff bug or bad golden trace");
                }

                if let Some(jzj_rs2) = instr.get_rs2() {
                    if g_rs2 != t_rs2 {
                        print_error("[D] RS2s do not match!");
                    }
                    assert_eq!(g_rs2, jzj_rs2, "pd6diff bug or bad golden trace");
                }

                if let Some(jzj_funct3) = instr.get_funct3() {
                    if g_funct3 != t_funct3 {
                        print_error("[D] Funct3s do not match!");
                    }
                    assert_eq!(g_funct3, jzj_funct3, "pd6diff bug or bad golden trace");
                }

                if let Some(jzj_funct7) = instr.get_funct7() {
                    if g_funct7 != t_funct7 {
                        print_error("[D] Funct7s do not match!");
                    }
                    assert_eq!(g_funct7, jzj_funct7, "pd6diff bug or bad golden trace");
                }

                if let Some(jzj_imm) = instr.get_imm() {
                    if g_imm != t_imm {
                        print_error("[D] IMMs do not match!");
                    }
                    assert_eq!(g_imm, jzj_imm as u32, "pd6diff bug or bad golden trace");
                }

                if let Some(jzj_shamt) = instr.get_shamt() {
                    if g_shamt != t_shamt {
                        print_error("[D] SHAMTs do not match!");
                    }
                    assert_eq!(g_shamt, jzj_shamt, "pd6diff bug or bad golden trace");
                }
            } else {
                print_error("[D] Mismatched line types or bad traces! Something is VERY wrong!");
            }

            if let (
                ParsedLine::R{addr_rs1: g_addr_rs1, addr_rs2: g_addr_rs2, ..},
                ParsedLine::R{addr_rs1: t_addr_rs1, addr_rs2: t_addr_rs2, ..},
                ParsedLine::R{data_rs1: g_data_rs1_next, data_rs2: g_data_rs2_next, ..},
                ParsedLine::R{data_rs1: t_data_rs1_next, data_rs2: t_data_rs2_next, ..}
            ) = (g_rline, t_rline, g_rline_next, t_rline_next) {
                if let Some(jzj_rs1) = instr.get_rs1() {
                    if g_addr_rs1 != t_addr_rs1 {
                        print_error("[R] RS1 addresses do not match!");
                    }
                    assert_eq!(g_addr_rs1, jzj_rs1, "pd6diff bug or bad golden trace");

                    if g_data_rs1_next != t_data_rs1_next {
                        print_error("[R] RS1 data does not match!");
                    }
                }

                if let Some(jzj_rs2) = instr.get_rs2() {
                    if g_addr_rs2 != t_addr_rs2 {
                        print_error("[R] RS2 addresses do not match!");
                    }
                    assert_eq!(g_addr_rs2, jzj_rs2, "pd6diff bug or bad golden trace");

                    if g_data_rs2_next != t_data_rs2_next {
                        print_error("[R] RS2 data does not match!");
                    }
                }
            } else {
                print_error("[R] Mismatched line types or bad traces! Something is VERY wrong!");
            }
        }

        //////////////////////////////////////////////////////////////////////////////////////////////////////
        //[E] Line Checking
        //////////////////////////////////////////////////////////////////////////////////////////////////////
        if !pipeline.e.is_bubble() {
            let instr = pipeline.e.instr.as_ref().unwrap();

            if let (
                ParsedLine::E{pc: g_pc, alu_result: g_alu_result, branch_taken: g_branch_taken},
                ParsedLine::E{pc: t_pc, alu_result: t_alu_result, branch_taken: t_branch_taken}
            ) = (g_eline, t_eline) {
                if g_pc != t_pc {
                    print_error("[E] PCs do not match!");
                }
                assert_eq!(g_pc, pipeline.e.pc, "pd6diff bug or bad golden trace");

                if !instr.is_fence() && !instr.is_system() {
                    if g_alu_result != t_alu_result {
                        print_error("[E] ALU results do not match!");
                    }
                }

                if instr.is_btype() {
                    if g_branch_taken != t_branch_taken {
                        print_error("[E] Branch taken line does not match!");
                    }
                }
            } else {
                print_error("[E] Mismatched line types or bad traces! Something is VERY wrong!");
            }
        }

        //////////////////////////////////////////////////////////////////////////////////////////////////////
        //[M] Line Checking
        //////////////////////////////////////////////////////////////////////////////////////////////////////
        if !pipeline.m.is_bubble() {
            let instr = pipeline.m.instr.as_ref().unwrap();
            if let (
                ParsedLine::M{pc: g_pc, addr: g_addr, read_not_write: g_read_not_write, access_size: g_access_size, memory_wdata: g_memory_wdata},
                ParsedLine::M{pc: t_pc, addr: t_addr, read_not_write: t_read_not_write, access_size: t_access_size, memory_wdata: t_memory_wdata}
            ) = (g_mline, t_mline) {
                if g_pc != t_pc {
                    print_error("[M] PCs do not match!");
                }
                assert_eq!(g_pc, pipeline.m.pc, "pd6diff bug or bad golden trace");

                if g_read_not_write != t_read_not_write {
                    print_error("[M] Read-not-write line does not match!");
                }

                if instr.is_memory() {
                    if g_addr != t_addr {
                        print_error("[M] Addresses do not match!");
                    }

                    if g_access_size != t_access_size {
                        print_error("[M] Access sizes do not match!");
                    }
                }

                if instr.is_stype() {
                    if g_memory_wdata != t_memory_wdata {
                        print_error("[M] Memory write data does not match!");
                    }
                }
            } else {
                print_error("[M] Mismatched line types or bad traces! Something is VERY wrong!");
            }
        }

        //////////////////////////////////////////////////////////////////////////////////////////////////////
        //[W] Line Checking
        //////////////////////////////////////////////////////////////////////////////////////////////////////
        if !pipeline.w.is_bubble() {
            let instr = pipeline.w.instr.as_ref().unwrap();
            if let (
                ParsedLine::W{pc: g_pc, we: g_we, addr_rd: g_addr_rd, data_rd: g_data_rd},
                ParsedLine::W{pc: t_pc, we: t_we, addr_rd: t_addr_rd, data_rd: t_data_rd}
            ) = (g_wline, t_wline) {
                if g_pc != t_pc {
                    print_error("[W] PCs do not match!");
                }
                assert_eq!(g_pc, pipeline.w.pc, "pd6diff bug or bad golden trace");

                if !instr.is_fence() {
                    if g_we != t_we {
                        print_error("[W] Write enable line does not match!");
                    }

                    if let Some(jzj_addr_rd) = instr.get_rd() {
                        if g_addr_rd != t_addr_rd {
                            print_error("[W] RD addresses do not match!");
                        }
                        assert_eq!(g_addr_rd, jzj_addr_rd, "pd6diff bug or bad golden trace");

                        if g_data_rd != t_data_rd {
                            print_error("[W] RD data does not match!");
                        }
                    }
                }
            } else {
                print_error("[W] Mismatched line types or bad traces! Something is VERY wrong!");
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
