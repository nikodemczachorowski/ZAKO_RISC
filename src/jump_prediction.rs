use std::collections::{HashMap, VecDeque};
use std::{io};
use rand::random;

pub struct Jump_Prediction {
    algorithms: Vec<(fn(&mut Jump_Prediction, u32) -> (u32, bool), String)>,
    BTB: HashMap<u32, (u32, bool)>,
    GHR: u16,
    PHT: [u16; 2_u32.pow(16) as usize],
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

fn GHR_jump(predictor: &mut Jump_Prediction, addr: u32) -> (u32, bool) {
    let index = predictor.PHT[predictor.GHR as usize];
    let x: f32 = random();
    let (mut dest,mut result) = (0, false);
    if predictor.BTB.contains_key(&addr) {
        if let Some(&(d,r)) = predictor.BTB.get(&addr) {
            dest = d;
            result = r;
        }
    }
    else {
        predictor.BTB.insert(addr, (0, false));
    }

   if 0.2 + (index as f32)*0.2 >= x
   {
       predictor.GHR = predictor.GHR << 1;
       predictor.GHR |= 1;
       predictor.taken.insert(addr,true);
       (dest, true)
   } else {
       predictor.GHR = predictor.GHR << 1; predictor.GHR |= 0;
       predictor.taken.insert(addr,false);
        (dest, false)
    }
    /*if index >= 2
    {
        predictor.GHR = predictor.GHR << 1; predictor.GHR |= 1;
        predictor.taken.insert(addr,true);
        (dest, true)
    }
    else {
        predictor.GHR = predictor.GHR << 1; predictor.GHR |= 0;
        predictor.taken.insert(addr, false);
        (dest, false)
    }*/
}

impl Jump_Prediction {
    pub fn new() -> Jump_Prediction {
        let mut prediction = Jump_Prediction {
            algorithms: Vec::new(),
            BTB: HashMap::new(),
            chosen: String::new(),
            taken: HashMap::new(),
            GHR: 0,
            PHT: [0; 2_u32.pow(16) as usize],
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
        self.algorithms
            .push((GHR_jump, String::from("GHR JUMP")));
    }
    pub fn change(&mut self, addr: u32, ip: &mut u32, result: bool, dest: i32) {
        let mut index = self.PHT[self.GHR as usize];
        if result {
            if index < 3
            {
                index += 1;
            }
        }
        else {
            if index > 0
            {
                index -= 1;
            }
        }
        self.PHT[self.GHR as usize] = index;
        self.GHR = self.GHR << 1; self.GHR |= result as u16;

        if !result && *self.taken.get(&addr).unwrap_or(&false) {
            *ip = addr + 4;
           // println!("unsuccesful jump taken");
        } else if result && !*self.taken.get(&addr).unwrap_or(&false) {
            *ip = dest as u32;
            println!("not taken when should have");
        } else {
            //println!("Prediction successfull");
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
                Ok(number) if number-1 < self.algorithms.len() as u32 => {
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
