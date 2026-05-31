use crate::{instruction::Instruction, memory::Memory, register_bank::RegisterBank};

const PIPELINE_LENGTH: usize = 5;

pub struct Simulation {
    memory: Memory,
    register_bank: RegisterBank,
    pipeline: [Instruction; PIPELINE_LENGTH],
    ip: u32,
    head: usize,
}

impl Simulation {
    pub fn new(memSize: u32, regCount: u32) -> Self {
        Self {
            memory: Memory::new(memSize),
            register_bank: RegisterBank::new(regCount),
            pipeline: 
            ip: 0,
            head: 0,
        }
    }

    pub fn tick(&mut self) {}
}
