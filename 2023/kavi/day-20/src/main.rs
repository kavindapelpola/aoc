use std::collections::HashMap;

mod load;
mod part_a;
mod part_b;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ModuleType {
    Broadcast,
    FlipFlop,
    Conjunction,
}

struct App {
    input: HashMap<String, (ModuleType, Vec<String>)>,
}

impl App {
    fn new() -> App {
        App {
            input: load::load(),
        }
    }
}

fn main() {
    part_a::run();
    part_b::run();
}
