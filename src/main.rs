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
    let mem = read_memory_file("pipe2.dat".to_string());
    print_mem(&mem);
    let mut sim = Simulation::new(2000, 32);
    sim.tick();
}
