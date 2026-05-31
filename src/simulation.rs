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
    pub fn new(mem_size: u32, reg_count: u32) -> Self {
        Self {
            memory: Memory::new(mem_size),
            register_bank: RegisterBank::new(reg_count),
            pipeline: [Instruction::new(); PIPELINE_LENGTH],
            ip: 0,
            head: 0,
        }
    }

    pub fn tick(&mut self) {}
}
