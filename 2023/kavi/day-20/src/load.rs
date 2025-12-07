use std::collections::HashMap;

use crate::ModuleType;

pub fn load() -> HashMap<String, (ModuleType, Vec<String>)> {
    let input = include_str!("../input.txt");
    let mut r = HashMap::new();
    for line in input.lines() {
        let p = line.split(" -> ").collect::<Vec<&str>>();
        let key;
        let module_type;
        match p[0][..1].to_string().as_str() {
            "b" => {
                key = p[0].to_string();
                module_type = ModuleType::Broadcast;
            }
            "%" => {
                key = p[0][1..].to_string();
                module_type = ModuleType::FlipFlop;
            }
            "&" => {
                key = p[0][1..].to_string();
                module_type = ModuleType::Conjunction;
            }
            _ => panic!("Unknown module type"),
        }
        let dest = p[1]
            .split(",")
            .map(|s| s.to_string().trim().to_string())
            .collect::<Vec<String>>();
        r.insert(key, (module_type, dest));
    }
    r
}
