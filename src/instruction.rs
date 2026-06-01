use crate::memory::*;
use crate::register_bank::*;

#[derive(Copy, Clone)]
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
}

#[derive(Copy, Clone)]
pub struct Instruction {
    in_code: u32,
    operation: ALU,
    dest: u8,
    res: i32,
}

impl Instruction {
    pub fn new() -> Self {
        Self {
            in_code: 0,
            operation: ALU::NOP,
            dest: 0,
            res: 0,
        }
    }
    pub fn fetch(&mut self, addr: u32, mem: &Memory) {
        self.in_code = mem.read(addr);
    }

    pub fn decode(&mut self, regs: &RegisterBank) {
        let mut opcode: u8 = (self.in_code >> 26) as u8;
        self.dest = ((self.in_code >> 21) & 0b11111) as u8;
        let reg1 = ((self.in_code >> 16) & 0b11111) as u8;
        let val1 = regs.get_register_value(reg1);

        let val2 = if opcode & 0x10 == 0 {
            let reg2 = ((self.in_code >> 11) & 0b11111) as u8;
            regs.get_register_value(reg2)
        } else {
            (self.in_code & 0xFFFF) as i32
        };

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
            0x08 => ALU::LOAD((val1 + val2) as u32),
            0x09 => {
                self.res = regs.get_register_value(self.dest);
                ALU::STORE(self.res, (val1 + val2) as u32)
            }
            _ => ALU::NOP,
        }
    }

    pub fn execute(&mut self) {
        match self.operation {
            ALU::ADD(op1, op2) => self.res = op1 + op2,
            ALU::SUB(op1, op2) => self.res = op1 - op2,
            ALU::MUL(op1, op2) => self.res = op1 * op2,
            ALU::DIV(op1, op2) => self.res = op1 / op2,
            ALU::AND(op1, op2) => self.res = op1 & op2,
            ALU::OR(op1, op2) => self.res = op1 | op2,
            ALU::XOR(op1, op2) => self.res = op1 ^ op2,
            _ => (),
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
