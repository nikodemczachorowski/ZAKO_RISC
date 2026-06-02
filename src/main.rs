use std::env;

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
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Invalid number of arguments");
        return;
    }
    let mem_file = &args[2];
    let cod_file = &args[1];
    let mut mem = read_memory_file(mem_file);
    read_program_file(cod_file, &mut mem);
    let mut sim = Simulation::new(mem, 32);
    let mut counter = 0;
    while sim.ip < 500 {
        sim.tick();
        counter += 1;
        // println!("{:04X}", sim.ip)
    }

    print_mem(sim.get_memory());
    println!("Cycles: {}", counter);
}
