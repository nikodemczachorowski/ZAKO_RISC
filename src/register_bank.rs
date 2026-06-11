#[derive(Clone)]
struct Register {
    pub val: i32,
    pub busy: bool,
}

pub struct RegisterBank {
    registers: Vec<Register>,
}

impl RegisterBank {
    pub fn new(reg_count: u32) -> Self {
        RegisterBank {
            registers: vec![
                Register {
                    val: 0,
                    busy: false,
                };
                reg_count as usize
            ],
        }
    }
    pub fn get_register_value(&self, reg: u8) -> i32 {
        self.registers[reg as usize].val
    }

    pub fn update_register_value(&mut self, reg: u8, val: i32) {
        self.registers[reg as usize].val = val;
    }

    pub fn is_register_busy(&self, reg: u8) -> bool {
        self.registers[reg as usize].busy
    }

    pub fn mark_as_busy(&mut self, reg: u8) {
        self.registers[reg as usize].busy = true;
    }

    pub fn unmark_as_busy(&mut self, reg: u8) {
        self.registers[reg as usize].busy = false;
    }
}
