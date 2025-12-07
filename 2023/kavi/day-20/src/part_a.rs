use crate::{App, ModuleType};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
enum PulseType {
    High,
    Low,
}

#[derive(Debug, Clone)]
pub enum StateType {
    Broadcast,
    FlipFlop(bool),                     // true = on, false = off
    Conjunction(HashMap<String, bool>), // true = high, false = low
}

pub fn run() {
    let app = App::new();

    let mut state = HashMap::new();
    for line in &app.input {
        let (key, (module_type, _)) = line;
        match module_type {
            ModuleType::Broadcast => {
                state.insert(key.clone(), StateType::Broadcast);
            }
            ModuleType::FlipFlop => {
                state.insert(key.clone(), StateType::FlipFlop(false));
            }
            ModuleType::Conjunction => {
                let mut connected = HashMap::new();
                app.input
                    .iter()
                    .filter(|(_, v)| v.1.contains(key))
                    .for_each(|(k, _)| {
                        connected.insert(k.clone(), false);
                    });
                state.insert(key.clone(), StateType::Conjunction(connected)); // true = high, false = low
            }
        }
    }

    let mut count_high = 0;
    let mut count_low = 0;
    // push the button once
    for _ in 0..1000 {
        let (high, low) = push_broadcast(&mut state, &app.input);
        count_high += high;
        count_low += low;
    }

    println!("Part A {}", count_high * count_low);
}

pub fn push_broadcast(
    state: &mut HashMap<String, StateType>,
    input: &HashMap<String, (ModuleType, Vec<String>)>,
) -> (u32, u32) {
    let mut pulses = vec![];
    let mut count_high = 0;
    let mut count_low = 1;

    // broadcaster send low pulse to every connected module
    for module in &input.get(&"broadcaster".to_string()).unwrap().1 {
        pulses.push((
            "broadcaster".to_string(), // source
            module.clone(),            // dest
            PulseType::Low,
        ));
        count_low += 1;
    }

    while !pulses.is_empty() {
        let to_process = pulses.clone();
        pulses.clear();
        for pulse in to_process {
            let (source_module, dest_module, pulse_type) = pulse;

            if input.get(&dest_module).is_none() {
                // dummy outputs
                continue;
            }

            let (module_type, connected_modules) = input.get(&dest_module).unwrap();

            // flip flops ignore high pulses
            if module_type == &ModuleType::FlipFlop && pulse_type == PulseType::High {
                continue;
            }

            match module_type {
                ModuleType::FlipFlop => {
                    // only processes low pulses
                    if let StateType::FlipFlop(s) = state.get_mut(&dest_module).unwrap() {
                        for connected_module in connected_modules {
                            // if off it sends a high
                            if !(*s) {
                                pulses.push((
                                    dest_module.clone(),      // source
                                    connected_module.clone(), //dest
                                    PulseType::High,
                                ));
                                count_high += 1;
                            }
                            // if on it sends a low
                            if *s {
                                pulses.push((
                                    dest_module.clone(),      // source
                                    connected_module.clone(), // dest
                                    PulseType::Low,
                                ));
                                count_low += 1;
                            }
                        }
                        // flip the state
                        *s = !(*s);
                    } else {
                        panic!("invalid, flipflop must have state type flipflop");
                    }
                }

                ModuleType::Conjunction => {
                    if let StateType::Conjunction(s) = state.get_mut(&dest_module).unwrap() {
                        // first update memory for input
                        if pulse_type == PulseType::High {
                            s.insert(source_module.clone(), true);
                        } else {
                            s.insert(source_module.clone(), false);
                        }
                        let send_pulse = if s.values().all(|&v| v) {
                            PulseType::Low
                        } else {
                            PulseType::High
                        };
                        for connected_module in connected_modules {
                            pulses.push((
                                dest_module.clone(),      // source
                                connected_module.clone(), // dest
                                send_pulse.clone(),
                            ));
                            if send_pulse == PulseType::High {
                                count_high += 1;
                            } else {
                                count_low += 1;
                            }
                        }
                    } else {
                        panic!("invalid conjuction must have state type conjunction");
                    }
                }

                _ => panic!("invalid destination"),
            }
        }
    }
    (count_high, count_low)
}
