pub struct Jump_Prediction{
    algorithms: Vec<(fn() -> bool, String)>,
}
fn static_jump(jump: bool) -> bool
{
    jump
}

fn memory_

impl Jump_Prediction {
    pub fn new() -> Jump_Prediction {
        let mut prediction = Jump_Prediction {
             algorithms: Vec::new(),
        };
        prediction.build();
        prediction
    }
    fn build(&mut self) -> ()
    {
        self.algorithms.push(((|| static_jump(true)) as fn() -> bool, String::from("Static jump TRUE")));
        self.algorithms.push(((|| static_jump(false)) as fn() -> bool, String::from("Static jump TRUE")));
    }
}