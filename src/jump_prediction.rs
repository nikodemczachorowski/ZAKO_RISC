use std::collections::HashMap;
use std::io;

pub struct Jump_Prediction {
    algorithms: Vec<(fn(&mut Jump_Prediction, u32) -> (u32, bool), String)>,
    BTB: HashMap<u32, (u32, bool)>,
    GHB: u32,
    taken: HashMap<u32, bool>,
    chosen: String,
}
fn static_jump(predictor: &mut Jump_Prediction, addr: u32) -> (u32, bool) {
    if predictor.chosen == "STATIC JUMP TRUE" {
        predictor.taken.insert(addr, true);
        (0, true)
    } else {
        predictor.taken.insert(addr, false);
        (0, false)
    }
}

fn memory_jump(predictor: &mut Jump_Prediction, addr: u32) -> (u32, bool) {
    if predictor.BTB.contains_key(&addr) {
        let (dest, result) = predictor.BTB.get(&addr).unwrap();
        predictor.taken.insert(addr, *result);
        return (*dest, *result);
    } else {
        predictor.BTB.insert(addr, (0, false));
        predictor.taken.insert(addr, false);
        return (0, false);
    }
}

fn GHB_jump(predictor: &mut Jump_Prediction, addr: u32) -> bool {
    true
}

impl Jump_Prediction {
    pub fn new() -> Jump_Prediction {
        let mut prediction = Jump_Prediction {
            algorithms: Vec::new(),
            BTB: HashMap::new(),
            chosen: String::new(),
            taken: HashMap::new(),
            GHB: 0,
        };
        prediction.build();
        prediction.choose();
        prediction
    }
    fn build(&mut self) -> () {
        self.algorithms
            .push((static_jump, String::from("STATIC JUMP TRUE")));
        self.algorithms
            .push((static_jump, String::from("STATIC JUMP FALSE")));
        self.algorithms
            .push((memory_jump, String::from("MEMORY JUMP")));
    }
    pub fn change(&mut self, addr: u32, ip: &mut u32, result: bool, dest: i32) {
        if !result && *self.taken.get(&addr).unwrap_or(&false) {
            *ip = addr + 4;
            println!("unsuccesful jump taken");
        } else if result && !*self.taken.get(&addr).unwrap_or(&false) {
            *ip = dest as u32;
            println!("not taken when should have");
        } else {
            println!("Prediction successfull");
        }
        self.BTB.insert(addr, (dest as u32, result));
    }
    pub fn choose(&mut self) -> () {
        let mut i: u32 = 1;
        for (_, name) in self.algorithms.iter() {
            println!("{}. {}", i, name);
            i = i + 1;
        }
        let algorithm_number: u32 = loop {
            let mut algorithm_number_to_parse: String = String::new();
            io::stdin()
                .read_line(&mut algorithm_number_to_parse)
                .expect("FAILED READING ALGORITHM");

            match algorithm_number_to_parse.trim().parse::<u32>() {
                Ok(number) if number < self.algorithms.len() as u32 => {
                    break number;
                }
                _ => {
                    println!("It is not correct number. Try again:");
                    continue;
                }
            }
        };
        self.chosen = self.algorithms[(algorithm_number - 1) as usize].1.clone();
    }

    pub fn predict(&mut self, addr: u32) -> (u32, bool) {
        for i in 0..self.algorithms.len() {
            if self.chosen == self.algorithms[i].1 {
                let function = self.algorithms[i].0;
                return function(self, addr);
            }
        }
        println!("You did not choose jump prediction algorithm.");
        (0, false)
    }
}
