use std::mem::swap;

use crate::jump_prediction::Jump_Prediction;
use crate::memory::*;
use crate::register_bank::*;

#[derive(Copy, Clone, Debug)]
enum ALU {
    NOP,
    ADD(i32, i32),
    SUB(i32, i32),
    MUL(i32, i32),
    DIV(i32, i32),
    AND(i32, i32),
    OR(i32, i32),
    XOR(i32, i32),
    LOAD(u32),
    STORE(i32, u32),
    BRZ(i32, i32),
    BRNZ(i32, i32),
    BRGT(i32, i32),
    BRGE(i32, i32),
    BRLT(i32, i32),
    BRLE(i32, i32),
}

#[derive(Copy, Clone, Debug)]
pub struct Instruction {
    in_code: u32,
    operation: ALU,
    dest: u8,
    res: i32,
    addr: u32,
}

impl Instruction {
    pub fn new() -> Self {
        Self {
            in_code: 0,
            operation: ALU::NOP,
            dest: 0,
            res: 0,
            addr: 0x00,
        }
    }
    pub fn fetch(&mut self, addr: &mut u32, mem: &Memory, jump_prediction: &mut Jump_Prediction) {
        self.in_code = mem.read(*addr);
        self.addr = *addr;
        if (self.in_code & (1 << 31) == 1) {
            let (dest, result) = jump_prediction.predict(*addr);

            if dest == 0 && result == true {
                *addr = self.in_code & 0xFFFF;
            } else if dest != 0 && result == true {
                *addr = dest;
            }
        }
    }

    pub fn decode(&mut self, regs: &RegisterBank) {
        let mut opcode: u8 = (self.in_code >> 26) as u8;
        self.dest = ((self.in_code >> 21) & 0b11111) as u8;
        let mut reg1 = ((self.in_code >> 16) & 0b11111) as u8;
        let mut val1 = regs.get_register_value(reg1);

        let val2 = if opcode & 0x10 == 0 {
            let reg2 = ((self.in_code >> 11) & 0b11111) as u8;
            regs.get_register_value(reg2)
        } else if opcode & 0x20 == 1 {
            (self.in_code & 0xFFFF) as i32
        } else {
            (self.in_code & 0xFFFF) as i32
        };
        dbg!(&self);
        opcode &= 0b1111;
        self.operation = match opcode {
            0x00 => ALU::NOP,
            0x01 => ALU::ADD(val1, val2),
            0x02 => ALU::SUB(val1, val2),
            0x03 => ALU::MUL(val1, val2),
            0x04 => ALU::DIV(val1, val2),
            0x05 => ALU::AND(val1, val2),
            0x06 => ALU::OR(val1, val2),
            0x07 => ALU::XOR(val1, val2),
            0x08 => {
                swap(&mut self.dest, &mut reg1);
                val1 = regs.get_register_value(reg1);
                ALU::LOAD((val1 + val2) as u32)
            }
            0x09 => {
                swap(&mut self.dest, &mut reg1);
                val1 = regs.get_register_value(reg1);
                self.res = regs.get_register_value(self.dest);
                ALU::STORE(self.res, (val1 + val2) as u32)
            }
            0x20 => ALU::BRZ(val1, val2),
            0x21 => ALU::BRNZ(val1, val2),
            0x22 => ALU::BRGT(val1, val2),
            0x23 => ALU::BRGE(val1, val2),
            0x24 => ALU::BRLT(val1, val2),
            0x25 => ALU::BRLE(val1, val2),
            _ => ALU::NOP,
        }
    }

    pub fn execute(&mut self, jump_prediction: &mut Jump_Prediction, ip: &mut u32) {
        match self.operation {
            ALU::NOP => (),
            ALU::LOAD(_) => (),
            ALU::STORE(_, _) => (),
            ALU::ADD(op1, op2) => self.res = op1 + op2,
            ALU::SUB(op1, op2) => self.res = op1 - op2,
            ALU::MUL(op1, op2) => self.res = op1 * op2,
            ALU::DIV(op1, op2) => self.res = op1 / op2,
            ALU::AND(op1, op2) => self.res = op1 & op2,
            ALU::OR(op1, op2) => self.res = op1 | op2,
            ALU::XOR(op1, op2) => self.res = op1 ^ op2,
            ALU::BRZ(op1, op2) => {
                self.res = (op1 == 0) as i32;
                jump_prediction.change(self.addr, op1 != 0, op2);
                *ip = op2 as u32;
            }
            ALU::BRNZ(op1, op2) => {
                self.res = (op1 != 0) as i32;
                jump_prediction.change(self.addr, op1 != 0, op2);
                *ip = op2 as u32;
            }
            ALU::BRGT(op1, op2) => {
                self.res = (op1 > 0) as i32;
                jump_prediction.change(self.addr, op1 != 0, op2);
                *ip = op2 as u32;
            }
            ALU::BRGE(op1, op2) => {
                self.res = (op1 >= 0) as i32;
                jump_prediction.change(self.addr, op1 != 0, op2);
                *ip = op2 as u32;
            }
            ALU::BRLT(op1, op2) => {
                self.res = (op1 < 0) as i32;
                jump_prediction.change(self.addr, op1 != 0, op2);
                *ip = op2 as u32;
            }
            ALU::BRLE(op1, op2) => {
                self.res = (op1 <= 0) as i32;
                jump_prediction.change(self.addr, op1 != 0, op2);
                *ip = op2 as u32;
            }
            _ => println!("Unimplemented operation"),
        }
    }

    pub fn memory(&mut self, mem: &mut Memory) {
        match self.operation {
            ALU::LOAD(addr) => self.res = mem.read(addr),
            ALU::STORE(op, addr) => mem.write(addr, op),
            _ => (),
        }
    }

    pub fn write_back(&mut self, regs: &mut RegisterBank) {
        if self.dest != 0 {
            regs.update_register_value(self.dest, self.res);
            regs.unmark_as_busy(self.dest);
        }
    }
}
