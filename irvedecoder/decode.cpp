/**
 * @brief   Code to decode RISC-V instructions
 * 
 * @copyright
 *  Copyright (C) 2023-2024 John Jekel\n
 *  Copyright (C) 2023 Nick Chan\n
 *  See the LICENSE file at the root of the project for licensing info.
 * 
 * Based on IRVE code, which was based on code from rv32esim
 *
*/

/* ------------------------------------------------------------------------------------------------
 * Includes
 * --------------------------------------------------------------------------------------------- */

#include "decode.h"

#include <cassert>
#include <cstdint>
#include <cstdio>
#include <string>
#include <stdexcept>

#include "common.h"

/* ------------------------------------------------------------------------------------------------
 * Function Implementations
 * --------------------------------------------------------------------------------------------- */

DecodedInst::DecodedInst(Word instruction) :
    m_opcode((Opcode)instruction.bits(6, 2).u),
    m_full_opcode(instruction.bits(6, 0).u),
    m_funct3(instruction.bits(14, 12).u),
    m_funct5(instruction.bits(31, 27).u),
    m_funct7(instruction.bits(31, 25).u),
    m_rd    (instruction.bits(11, 7) .u),
    m_rs1   (instruction.bits(19, 15).u),
    m_rs2   (instruction.bits(24, 20).u),
    m_imm_I (instruction.bits(31, 20).sign_extend_from_bit_number(11).u),
    m_imm_S (
        (
            (instruction.bits(31, 25) << 5) | 
            instruction.bits (11, 7)
        )
        .sign_extend_from_bit_number(11).u
    ),
    m_imm_B (
        (
            (instruction.bit (31)       << 12)  |
            (instruction.bit (7)        << 11)  | 
            (instruction.bits(30, 25)   << 5)   | 
            (instruction.bits(11, 8)    << 1)   |
            0b0
        )
        .sign_extend_from_bit_number(12).u
    ),
    m_imm_U (instruction & 0b11111111111111111111000000000000),//Just zero out the lower 12 bits (keep the upper 20)
    m_imm_J (
        (
            (instruction.bit (31)       << 20)  | 
            (instruction.bits(19, 12)   << 12)  | 
            (instruction.bit (20)       << 11)  | 
            (instruction.bits(30, 21)   << 1)   |
            0b0
        )
        .sign_extend_from_bit_number(20).u
    )
{
    //These are defined invalid RISC-V instructions
    //In addition, we don't support compressed instructions
    if (!instruction || (instruction == 0xFFFFFFFF) || ((instruction & 0b11) != 0b11)) {
        throw std::invalid_argument("Illegal instruction");
    }

    switch (this->m_opcode) {
        //R-type
        case Opcode::OP:
        case Opcode::CUSTOM_0://We implement this opcode with some custom instructions!
        case Opcode::AMO:
            this->m_format = InstFormat::R_TYPE;
            break;
        //I-type
        case Opcode::LOAD:
        case Opcode::OP_IMM:
        case Opcode::JALR:
        case Opcode::SYSTEM:
        case Opcode::MISC_MEM:
            this->m_format = InstFormat::I_TYPE;
            break;
        //S-type
        case Opcode::STORE:
            this->m_format = InstFormat::S_TYPE;
            break;
        //B-type
        case Opcode::BRANCH:
            this->m_format = InstFormat::B_TYPE;
            break;
        //U-type
        case Opcode::LUI:
        case Opcode::AUIPC:
            this->m_format = InstFormat::U_TYPE;
            break;
        //J-type
        case Opcode::JAL:
            this->m_format = InstFormat::J_TYPE;
            break;
        default:
            throw std::invalid_argument("Illegal instruction");
            break;
    }
}

void DecodedInst::log() const {
    switch (this->get_format()) {
        case InstFormat::R_TYPE:
            printf("type   = R\n");
            printf("opcode = 0x%X\n", this->get_opcode());
            printf("fullop = 0x%X\n", this->get_full_opcode());
            printf("funct3 = 0x%X\n", this->get_funct3());
            printf("funct7 = 0x%X\n", this->get_funct7());
            printf("rd     = x%u\n", this->get_rd());
            printf("rs1    = x%u\n", this->get_rs1());
            printf("rs2    = x%u\n", this->get_rs2());
            break;
        case InstFormat::I_TYPE:
            printf("type   = I\n");
            printf("opcode = 0x%X\n", this->get_opcode());
            printf("fullop = 0x%X\n", this->get_full_opcode());
            printf("funct3 = 0x%X\n", this->get_funct3());
            printf("rd     = x%u\n", this->get_rd());
            printf("rs1    = x%u\n", this->get_rs1());
            printf("imm    = 0x%X\n", this->get_imm());
            break;
        case InstFormat::S_TYPE:
            printf("type   = S\n");
            printf("opcode = 0x%X\n", this->get_opcode());
            printf("fullop = 0x%X\n", this->get_full_opcode());
            printf("funct3 = 0x%X\n", this->get_funct3());
            printf("rs1    = x%u\n", this->get_rs1());
            printf("rs2    = x%u\n", this->get_rs2());
            printf("imm    = 0x%X\n", this->get_imm());
            break;
        case InstFormat::B_TYPE:
            printf("type   = B\n");
            printf("opcode = 0x%X\n", this->get_opcode());
            printf("fullop = 0x%X\n", this->get_full_opcode());
            printf("funct3 = 0x%X\n", this->get_funct3());
            printf("rs1    = x%u\n", this->get_rs1());
            printf("rs2    = x%u\n", this->get_rs2());
            printf("imm    = 0x%X\n", this->get_imm());
            break;
        case InstFormat::U_TYPE:
            printf("type   = U\n");
            printf("opcode = 0x%X\n", this->get_opcode());
            printf("fullop = 0x%X\n", this->get_full_opcode());
            printf("rd     = x%u\n", this->get_rd());
            printf("imm    = 0x%X\n", this->get_imm());
            break;
        case InstFormat::J_TYPE:
            printf("type   = J\n");
            printf("opcode = 0x%X\n", this->get_opcode());
            printf("fullop = 0x%X\n", this->get_full_opcode());
            printf("rd     = x%u\n", this->get_rd());
            printf("imm    = 0x%X\n", this->get_imm());
            break;
        default:
            assert(false && "We should never get here");
            break;
    }
}

InstFormat DecodedInst::get_format() const {
    return this->m_format;
}

Opcode DecodedInst::get_opcode() const {
    return this->m_opcode;
}

uint8_t DecodedInst::get_full_opcode() const {
    return this->m_full_opcode;
}

//FIXME all these assertions cause problems for the SYSTEM instructions

uint8_t DecodedInst::get_funct3() const {
    /*
    assert((this->get_format() != InstFormat::U_TYPE) &&
            "Attempt to get funct3 of U-type instruction!");
    assert((this->get_format() != InstFormat::J_TYPE) &&
            "Attempt to get funct3 of J-type instruction!");
    */
    return this->m_funct3;
}

uint8_t DecodedInst::get_funct5() const {
    assert((this->get_opcode() == Opcode::AMO) &&
            "Attempt to get funct5 of non-AMO instruction!");
    return this->m_funct5;
}

uint8_t DecodedInst::get_funct7() const {
    //FIXME this assertion causes problems for the SYSTEM instructions (need to add an exception for them)
    //assert((this->get_format() == InstFormat::R_TYPE) && "Attempt to get funct7 of non-R-type instruction!");
    return this->m_funct7;
}

uint8_t DecodedInst::get_rd() const {
    /*
    assert((this->get_format() != InstFormat::S_TYPE) &&
            "Attempt to get rd of S-type instruction!");
    assert((this->get_format() != InstFormat::B_TYPE) &&
            "Attempt to get rd of B-type instruction!");
    */
    return this->m_rd;
}

uint8_t DecodedInst::get_rs1() const {
    /*
    assert((this->get_format() != InstFormat::U_TYPE) &&
            "Attempt to get rs1 of U-type instruction!");
    assert((this->get_format() != InstFormat::J_TYPE) &&
            "Attempt to get rs1 of J-type instruction!");
    */
    return this->m_rs1;
}

uint8_t DecodedInst::get_rs2() const {
    //FIXME these assertions cause problems for the SYSTEM instructions (need to add an exception for them)
    //assert((this->get_format() != InstFormat::I_TYPE) && "Attempt to get rs2 of I-type instruction!");
    //assert((this->get_format() != InstFormat::U_TYPE) && "Attempt to get rs2 of U-type instruction!");
    //assert((this->get_format() != InstFormat::J_TYPE) && "Attempt to get rs2 of J-type instruction!");
    return this->m_rs2;
}

Word DecodedInst::get_imm() const {
    switch (this->get_format()) {
        case InstFormat::R_TYPE:
            assert(false && "Attempt to get imm of R-type instruction!");
            break;
        case InstFormat::I_TYPE:
            return this->m_imm_I;
            break;
        case InstFormat::S_TYPE:
            return this->m_imm_S;
            break;
        case InstFormat::B_TYPE:
            return this->m_imm_B;
            break;
        case InstFormat::U_TYPE:
            return this->m_imm_U;
            break;
        case InstFormat::J_TYPE:
            return this->m_imm_J;
            break;
    }

    assert(false && "We should never get here");
}
