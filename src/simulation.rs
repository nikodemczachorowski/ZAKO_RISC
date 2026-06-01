use crate::{instruction::Instruction, memory::Memory, register_bank::RegisterBank};
use crate::jump_prediction::Jump_Prediction;

const PIPELINE_LENGTH: usize = 5;

pub struct Simulation {
    memory: Memory,
    register_bank: RegisterBank,
    jump_prediction: Jump_Prediction,
    pipeline: [Instruction; PIPELINE_LENGTH],
    ip: u32,
    head: usize,
}

impl Simulation {
    pub fn new(mem_size: u32, reg_count: u32) -> Self {
        Self {
            memory: Memory::new(mem_size),
            jump_prediction: Jump_Prediction::new(),
            register_bank: RegisterBank::new(reg_count),
            pipeline: [Instruction::new(); PIPELINE_LENGTH],
            ip: 0,
            head: 0,
        }
    }

    pub fn tick(&mut self) {
        self.pipeline[self.head].fetch(&mut self.ip, &self.memory, &mut self.jump_prediction);
        self.pipeline[(self.head + 1) % PIPELINE_LENGTH].decode(&self.register_bank);
        self.pipeline[(self.head + 2) % PIPELINE_LENGTH].execute(&mut self.jump_prediction, &mut self.ip);
        self.pipeline[(self.head + 3) % PIPELINE_LENGTH].memory(&mut self.memory);
        self.pipeline[(self.head + 4) % PIPELINE_LENGTH].write_back(&mut self.register_bank);
        self.ip += 4;
        self.head = (self.head + 1) % PIPELINE_LENGTH;
    }
}
