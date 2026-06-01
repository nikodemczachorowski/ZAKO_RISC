use crate::{
    files::{print_mem, read_memory_file, read_program_file},
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
    print_mem(&mem);
    let mut sim = Simulation::new(2000, 32);
    sim.tick();
}
