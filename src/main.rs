use crate::files::read_program_file;
use crate::{
    files::{print_mem, read_memory_file},
    simulation::Simulation,
};

mod files;
mod instruction;
mod jump_prediction;
mod memory;
mod register_bank;
mod simulation;

fn main() {
    let mut mem = read_memory_file("pipe2.dat".to_string());
    read_program_file("pipe2.cod".to_string(), &mut mem);
    let mut sim = Simulation::new(mem, 32);
    while sim.ip < 500 {
        sim.tick();
    }

    print_mem(sim.get_memory());
}
