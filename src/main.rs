use crate::simulation::Simulation;

mod instruction;
mod memory;
mod register_bank;
mod simulation;

fn main() {
    let mut sim = Simulation::new(2000, 32);
    println!("Hello, world!");
}
