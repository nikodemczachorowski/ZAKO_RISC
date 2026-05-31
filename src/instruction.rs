use crate::memory::*;
use crate::register_bank::*;

#[derive(Copy, Clone)]
enum PipelineStage {
    IF,
    ID,
    EX,
    MEM,
    WB,
}

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
    stage: PipelineStage,
    in_str: u32,
    operation: ALU,
    reg1: u8,
    reg2: u8,
    imm: i32,
    dest: u8,
    res: i32,
}

impl Instruction {
    pub fn new() -> Self {
        Self {
            stage: PipelineStage::IF,
            in_str: 0,
            operation: ALU::NOP,
            reg1: 0,
            reg2: 0,
            imm: 0,
            dest: 0,
            res: 0,
        }
    }
    pub fn fetch(&mut self, addr: u32, mem: Memory) {
        self.in_str = mem.read(addr);
    }

    pub fn decode(&mut self, regs: RegisterBank) {}

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
