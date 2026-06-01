use std::cell::RefCell;
use std::collections::HashMap;
use crate::instruction::Instruction;

pub struct Jump_Prediction{
    algorithms: Vec<(fn(&mut Jump_Prediction, u32) -> (u32, bool), String)>,
    BTB: HashMap<u32,(u32,bool)>,
    GHB: u32,
    chosen: String,
}
fn static_jump(predictor: &mut Jump_Prediction, addr: u32) -> (u32,bool)
{
    if predictor.chosen == "STATIC JUMP TRUE" {
        (0, true)
    }
    else {
        (0, false)
    }
}

fn memory_jump(predictor: &mut Jump_Prediction, addr: u32) -> (u32,bool)
{
    if predictor.BTB.contains_key(&addr) {
        let (dest, result) = predictor.BTB.get(&addr).unwrap();
        return (*dest, *result);
    }
    else {
        predictor.BTB.insert(addr, (0,false));
        return (0,false);
    }
}

fn GHB_jump(predictor: &mut Jump_Prediction, addr: u32) -> bool
{
    true
}


impl Jump_Prediction {
    pub fn new() -> Jump_Prediction {
        let mut prediction = Jump_Prediction {
            algorithms: Vec::new(),
            BTB: HashMap::new(),
            chosen: String::new(),
            GHB: 0,
        };
        prediction.build();
        prediction
    }
    fn build(&mut self) -> ()
    {
        self.algorithms.push((static_jump, String::from("STATIC JUMP TRUE")));
        self.algorithms.push((static_jump, String::from("STATIC JUMP FALSE")));
        self.algorithms.push((memory_jump, String::from("MEMORY JUMP")));
    }
    pub fn change(&mut self, addr: u32, result: bool, dest: i32) {
        self.BTB.insert(addr, (dest as u32,result));
    }
    pub fn choose(&mut self, algorithm_number: u8) -> ()
    {
        self.chosen = self.algorithms[algorithm_number as usize].1.clone();
    }

    pub fn predict(&mut self, addr: u32) -> (u32, bool)
    {
        let chosen_algo = self.chosen.clone();
        for i in 0..self.algorithms.len() {
            if self.chosen == self.algorithms[i].1
            {
                let function = self.algorithms[i].0;
                function(self, addr);
            }
        }
        println!("You did not choose jump prediction algorithm.");
        (0, false)
    }
}