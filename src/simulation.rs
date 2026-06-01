use crate::jump_prediction::Jump_Prediction;
use crate::{instruction::Instruction, memory::Memory, register_bank::RegisterBank};

const PIPELINE_LENGTH: usize = 5;

pub struct Simulation {
    memory: Memory,
    register_bank: RegisterBank,
    jump_prediction: Jump_Prediction,
    pipeline: [Instruction; PIPELINE_LENGTH],
    pub ip: u32,
    head: usize,
}

impl Simulation {
    pub fn new(mem: Memory, reg_count: u32) -> Self {
        Self {
            memory: mem,
            jump_prediction: Jump_Prediction::new(),
            register_bank: RegisterBank::new(reg_count),
            pipeline: [Instruction::new(); PIPELINE_LENGTH],
            ip: 0,
            head: 0,
        }
    }

    pub fn tick(&mut self) {
        let former_ip = self.ip;
        self.pipeline[self.head].fetch(&mut self.ip, &self.memory, &mut self.jump_prediction);
        self.pipeline[(self.head + 1) % PIPELINE_LENGTH].decode(&self.register_bank);
        self.pipeline[(self.head + 2) % PIPELINE_LENGTH]
            .execute(&mut self.jump_prediction, &mut self.ip);
        self.pipeline[(self.head + 3) % PIPELINE_LENGTH].memory(&mut self.memory);
        self.pipeline[(self.head + 4) % PIPELINE_LENGTH].write_back(&mut self.register_bank);
        if former_ip == self.ip {
            self.ip += 4
        }
        self.head = (self.head + 1) % PIPELINE_LENGTH;
    }

    pub fn get_memory(&self) -> &Memory {
        &self.memory
    }
}
