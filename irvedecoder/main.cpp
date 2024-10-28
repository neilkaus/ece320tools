/**
 * @file    main.cpp
 * @brief   irvedecoder entry point
 * 
 * @copyright Copyright (C) 2024 John Jekel and Nick Chan
 *
*/

/* ------------------------------------------------------------------------------------------------
 * Includes
 * --------------------------------------------------------------------------------------------- */

#include "common.h"
#include "decode.h"

#include <cstdlib>
#include <cstdio>
#include <stdexcept>
#include <fstream>
#include <iostream>
#include <ios>
#include <optional>
#include <unordered_map>
#include <utility>
#include <cstring>

/* ------------------------------------------------------------------------------------------------
 * Static Variables
 * --------------------------------------------------------------------------------------------- */

static const uint32_t register_init_values[32] = {
    0x00000000,             //x0:  0
    0x00000000,             //x1:  0
    0x01000000 + 0x00100000,//x2:  32'h01000000 + `MEM_DEPTH
    0x00000000,             //x3:  0
    0x00000000,             //x4:  0
    0x00000000,             //x5:  0
    0x00000000,             //x6:  0
    0x00000000,             //x7:  0
    0x00000000,             //x8:  0
    0x00000000,             //x9:  0
    0x00000000,             //x10: 0
    0x00000000,             //x11: 0
    0x00000000,             //x12: 0
    0x00000000,             //x13: 0
    0x00000000,             //x14: 0
    0x00000000,             //x15: 0
    0x00000000,             //x16: 0
    0x00000000,             //x17: 0
    0x00000000,             //x18: 0
    0x00000000,             //x19: 0
    0x00000000,             //x20: 0
    0x00000000,             //x21: 0
    0x00000000,             //x22: 0
    0x00000000,             //x23: 0
    0x00000000,             //x24: 0
    0x00000000,             //x25: 0
    0x00000000,             //x26: 0
    0x00000000,             //x27: 0
    0x00000000,             //x28: 0
    0x00000000,             //x29: 0
    0x00000000,             //x30: 0
    0x00000000              //x31: 0
};

/* ------------------------------------------------------------------------------------------------
 * Static Function Declarations
 * --------------------------------------------------------------------------------------------- */

static void usage(void);
static void dump(uint32_t instruction);
static void pd2(uint32_t instruction, uint32_t addr);
static void pd2_trace(const char* vhex32_file_path);
static void pd2_check(const char* vhex32_file_path, const char* trace_file_path);
static void pd3_golden_preprocess(const char* trace_file_path);
static void pd4_golden_preprocess(const char* trace_file_path);

/* ------------------------------------------------------------------------------------------------
 * Function Implementations
 * --------------------------------------------------------------------------------------------- */

int main(int argc, const char* const* argv) {
    switch (argc) {
        case 3: {
            switch (argv[1][0]) {
                case 'd': {
                    uint32_t instruction = std::strtoul(argv[2], nullptr, 16);
                    dump(instruction);
                    return 0;
                }
                case 't': {
                    pd2_trace(argv[2]);
                    return 0;
                }
                case 'e': {
                    pd3_golden_preprocess(argv[2]);
                    return 0;
                }
                case '4': {
                    pd4_golden_preprocess(argv[2]);
                    return 0;
                }
                default: usage(); return 1;
            }
        }
        case 4: {
            switch (argv[1][0]) {
                case 'c': {
                    pd2_check(argv[2], argv[3]);
                    return 0;
                }
                case '2': {
                    uint32_t instruction = std::strtoul(argv[2], nullptr, 16);
                    uint32_t addr = std::strtoul(argv[3], nullptr, 16);
                    pd2(instruction, addr);
                    return 0;
                }
                default: usage(); return 1;
            }
        }
        default: usage(); return 1;
    }
}

/* ------------------------------------------------------------------------------------------------
 * Static Function Implementations
 * --------------------------------------------------------------------------------------------- */

static void usage(void) {
    printf("Usage: One of (numbers always in hex):\n");
    printf("`irvedecoder d <instruction>`, which will simply dump the fields of an instruction\n");
    printf("`irvedecoder 2 <instruction> <addr>`, which will dump the instruction in the PD2 decode trace format\n");
    printf("`irvedecoder t <vhex32 file>`, which will emit a PD2 trace file you can diff against\n");
    printf("`irvedecoder c <vhex32 file> <trace file>`, which will check the PD2 trace file against the vhex32 file for correctness\n");
    printf("`irvedecoder e <trace file>`, which sets alu_result to 0 for all execute instructions in the trace file -> stdout\n");
    printf("`irvedecoder 4 <trace file>`, which is useful for PD4");
}

static void dump(uint32_t instruction) {
    DecodedInst(instruction).log();
}

static void pd2(uint32_t instruction, uint32_t addr) {
    auto decoded_inst = DecodedInst(instruction);
    uint32_t opcode = decoded_inst.get_full_opcode();

    uint32_t rd     = decoded_inst.get_rd();
    uint32_t rs1    = decoded_inst.get_rs1();
    uint32_t rs2    = decoded_inst.get_rs2();
    uint32_t funct3 = decoded_inst.get_funct3();
    uint32_t funct7 = decoded_inst.get_funct7();

    uint32_t imm    = 0;
    switch (decoded_inst.get_format()) {
        case InstFormat::R_TYPE:
            break;
        case InstFormat::I_TYPE:
            imm    = decoded_inst.get_imm().u;
            break;
        case InstFormat::S_TYPE:
        case InstFormat::B_TYPE:
            imm    = decoded_inst.get_imm().u;
            break;
        case InstFormat::U_TYPE:
            imm    = decoded_inst.get_imm().u;
            break;
        case InstFormat::J_TYPE:
            imm    = decoded_inst.get_imm().u;
            break;
        default:
            throw std::invalid_argument("Illegal instruction");
    }

    uint32_t shamt  = imm & 0x1F;

    printf("[D] %08x %02x %02x %02x %02x %x %02x %08x %02x\n", addr, opcode, rd, rs1, rs2, funct3, funct7, imm, shamt);
}

static void pd2_trace(const char* vhex32_file_path) {
    std::ifstream vhex32_file(vhex32_file_path);

    if (!vhex32_file.is_open()) {
        throw std::runtime_error("Failed to open file");
    }

    vhex32_file >> std::hex;

    uint32_t addr = 0x01000000;
    uint32_t vhex32_inst;

    try {
        while (vhex32_file >> vhex32_inst) {
            pd2(vhex32_inst, addr);
            addr += 4;
        }
    } catch (const std::invalid_argument& e) {
        //Stop emitting trace upon encountering an illegal instruction silently
    }
}

static void pd2_check(const char* vhex32_file_path, const char* trace_file_path) {
    throw std::runtime_error("not yet implemented");
    std::ifstream vhex32_file(vhex32_file_path);
    std::ifstream trace_file(trace_file_path);

    if (!vhex32_file.is_open() || !trace_file.is_open()) {
        throw std::runtime_error("Failed to open file");
    }

    vhex32_file >> std::hex;
    trace_file >> std::hex;

    uint32_t addr = 0x01000000;
    uint32_t vhex32_inst;
    while (vhex32_file >> vhex32_inst) {
        printf("vhex32: %08X\n", vhex32_inst);
        addr += 4;
        //TODO
    }
}

static void pd3_golden_preprocess(const char* trace_file_path) {
    std::ifstream trace_file(trace_file_path);

    if (!trace_file.is_open()) {
        throw std::runtime_error("Failed to open file");
    }

    trace_file >> std::hex;

    //If we should replace the register indexes/data for a given PC address
    std::unordered_map<uint32_t, std::optional<std::pair<uint8_t, uint32_t>>> replace_rs1;
    std::unordered_map<uint32_t, std::optional<std::pair<uint8_t, uint32_t>>> replace_rs2;

    uint32_t most_recent_fetch_pc_address = 0;

    std::string token;
    while (trace_file >> token) {
        if (token == "[F]") {
            uint32_t pc_address, content;
            trace_file >> pc_address >> content;
            try {
                auto decoded_inst = DecodedInst(content);

                //Replace with instruction garbage for instructions that don't take from rs1 or rs2
                switch (decoded_inst.get_format()) {
                    //Have RS1 and RS2
                    case InstFormat::R_TYPE:
                    case InstFormat::S_TYPE:
                    case InstFormat::B_TYPE:
                        replace_rs1[pc_address] = std::nullopt;
                        replace_rs2[pc_address] = std::nullopt;
                        break;
                    //Have RS1 but not RS2
                    case InstFormat::I_TYPE:
                        replace_rs1[pc_address] = std::nullopt;
                        replace_rs2[pc_address] = std::make_pair(decoded_inst.get_rs2(), register_init_values[decoded_inst.get_rs2()]);
                        break;
                    //Have neither RS1 nor RS2
                    case InstFormat::U_TYPE:
                    case InstFormat::J_TYPE:
                        replace_rs1[pc_address] = std::make_pair(decoded_inst.get_rs1(), register_init_values[decoded_inst.get_rs1()]);
                        replace_rs2[pc_address] = std::make_pair(decoded_inst.get_rs2(), register_init_values[decoded_inst.get_rs2()]);
                        break;
                    default:
                        throw std::invalid_argument("Illegal instruction");
                }

                most_recent_fetch_pc_address = pc_address;
            } catch (const std::invalid_argument& e) {
                //Finish early upon encountering an illegal instruction
                return;
            }
        } else if (token == "[D]") {
            uint32_t pc_address, opcode, rd, rs1, rs2, funct3, funct7, imm, shamt;
            trace_file >> pc_address >> opcode >> rd >> rs1 >> rs2 >> funct3 >> funct7 >> imm >> shamt;
        } else if (token == "[R]") {
            uint32_t addr_rs1, addr_rs2, data_rs1, data_rs2;
            trace_file >> addr_rs1 >> addr_rs2 >> data_rs1 >> data_rs2;

            //Have to use the most recent PC address to get the correct values since we don't have a PC address for this line

            auto maybe_new_rs1_vals = replace_rs1[most_recent_fetch_pc_address].value_or(std::make_pair(addr_rs1, data_rs1));
            addr_rs1 = maybe_new_rs1_vals.first;
            data_rs1 = maybe_new_rs1_vals.second;

            auto maybe_new_rs2_vals = replace_rs2[most_recent_fetch_pc_address].value_or(std::make_pair(addr_rs2, data_rs2));
            addr_rs2 = maybe_new_rs2_vals.first;
            data_rs2 = maybe_new_rs2_vals.second;

            printf("[R] %02x %02x %08x %08x\n", addr_rs1, addr_rs2, data_rs1, data_rs2);
        } else if (token == "[E]") {
            uint32_t pc_address, alu_result, branch_taken;
            trace_file >> pc_address >> alu_result >> branch_taken;

            //Also written out, unchanged, for comparison
            //Originally we were sometimes zeroing out alu_result, thinking it was sometimes don't care, but in their scheme
            //even for branches it's always used for something...
            printf("[E] %08x %08x %d\n", pc_address, alu_result, branch_taken);
        } else {
            throw std::invalid_argument("Unexpected token in trace: " + token);
        }
    }
}

static void pd4_golden_preprocess(const char* trace_file_path) {
    std::ifstream trace_file(trace_file_path);

    if (!trace_file.is_open()) {
        throw std::runtime_error("Failed to open file");
    }

    trace_file >> std::hex;

    //NEW: We actually keep track of register values now
    uint32_t register_values[32];
    std::memcpy(register_values, register_init_values, sizeof(register_values));

    //If we should replace the register indexes/data for a given PC address
    std::unordered_map<uint32_t, std::optional<std::pair<uint8_t, uint32_t>>> replace_rs1;
    std::unordered_map<uint32_t, std::optional<std::pair<uint8_t, uint32_t>>> replace_rs2;

    uint32_t most_recent_fetch_pc_address   = 0;
    uint32_t most_recent_fetch_instruction  = 0;
    bool     most_recent_fetch_is_fence     = false;

    std::string token;
    while (trace_file >> token) {
        if (token == "[F]") {
            uint32_t pc_address, content;
            trace_file >> pc_address >> content;

            printf("[F] %08x %08x\n", pc_address, content);

            try {
                auto decoded_inst = DecodedInst(content);

                most_recent_fetch_is_fence = (decoded_inst.get_opcode() == Opcode::MISC_MEM);

                //Replace with instruction garbage for instructions that don't take from rs1 or rs2
                switch (decoded_inst.get_format()) {
                    //Have RS1 and RS2
                    case InstFormat::R_TYPE:
                    case InstFormat::S_TYPE:
                    case InstFormat::B_TYPE:
                        replace_rs1[pc_address] = std::nullopt;
                        replace_rs2[pc_address] = std::nullopt;
                        break;
                    //Have RS1 but not RS2
                    case InstFormat::I_TYPE:
                        replace_rs1[pc_address] = std::nullopt;
                        replace_rs2[pc_address] = std::make_pair(decoded_inst.get_rs2(), register_values[decoded_inst.get_rs2()]);
                        break;
                    //Have neither RS1 nor RS2
                    case InstFormat::U_TYPE:
                    case InstFormat::J_TYPE:
                        replace_rs1[pc_address] = std::make_pair(decoded_inst.get_rs1(), register_values[decoded_inst.get_rs1()]);
                        replace_rs2[pc_address] = std::make_pair(decoded_inst.get_rs2(), register_values[decoded_inst.get_rs2()]);
                        break;
                    default:
                        throw std::invalid_argument("Illegal instruction");
                }

                most_recent_fetch_pc_address    = pc_address;
                most_recent_fetch_instruction   = content;
            } catch (const std::invalid_argument& e) {
                //Finish early upon encountering an illegal instruction
                return;
            }
        } else if (token == "[D]") {
            //Consume all the tokens (we won't use these)
            uint32_t pc_address, opcode, rd, rs1, rs2, funct3, funct7, imm, shamt;
            trace_file >> pc_address >> opcode >> rd >> rs1 >> rs2 >> funct3 >> funct7 >> imm >> shamt;

            //Just use our PD2 decoder to emit the same output
            pd2(most_recent_fetch_instruction, most_recent_fetch_pc_address);
        } else if (token == "[R]") {
            uint32_t addr_rs1, addr_rs2, data_rs1, data_rs2;
            trace_file >> addr_rs1 >> addr_rs2 >> data_rs1 >> data_rs2;

            //Have to use the most recent PC address to get the correct values since we don't have a PC address for this line

            auto maybe_new_rs1_vals = replace_rs1[most_recent_fetch_pc_address].value_or(std::make_pair(addr_rs1, data_rs1));
            addr_rs1 = maybe_new_rs1_vals.first;
            data_rs1 = maybe_new_rs1_vals.second;

            auto maybe_new_rs2_vals = replace_rs2[most_recent_fetch_pc_address].value_or(std::make_pair(addr_rs2, data_rs2));
            addr_rs2 = maybe_new_rs2_vals.first;
            data_rs2 = maybe_new_rs2_vals.second;

            printf("[R] %02x %02x %08x %08x\n", addr_rs1, addr_rs2, data_rs1, data_rs2);
        } else if (token == "[E]") {
            uint32_t pc_address, alu_result, branch_taken;
            trace_file >> pc_address >> alu_result >> branch_taken;

            //Also written out, unchanged, for comparison
            //Originally we were sometimes zeroing out alu_result, thinking it was sometimes don't care, but in their scheme
            //even for branches it's always used for something...
            printf("[E] %08x %08x %d\n", pc_address, alu_result, branch_taken);
        } else if (token == "[M]") {
            uint32_t pc_address, memory_address, read_write, access_size, memory_data;
            trace_file >> pc_address >> memory_address >> read_write >> access_size >> memory_data;

            //TODO

            printf("[M] %08x %08x %d %d %08x\n", pc_address, memory_address, read_write, access_size, memory_data);
        } else if (token == "[W]") {
            uint32_t pc_address, write_enable, write_rd, data_rd;
            trace_file >> pc_address >> write_enable >> write_rd >> data_rd;

            if (most_recent_fetch_is_fence) {
                //Fences don't write to registers
                write_enable = 0;
            }

            if ((write_enable == 1) && (write_rd != 0)) {
                register_values[write_rd] = data_rd;
            }

            printf("[W] %08x %d %02x %08x\n", pc_address, write_enable, write_rd, data_rd);
        } else {
            throw std::invalid_argument("Unexpected token in trace: " + token);
        }
    }
}
