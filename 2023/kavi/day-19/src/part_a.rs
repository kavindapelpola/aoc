use std::collections::HashMap;

use crate::App;

pub fn run() {
    let app = App::new();
    let mut total = 0;
    let (parts, workflows) = app.input;
    for part in parts {
        if is_accepted(part, &workflows) {
            total += part.0 + part.1 + part.2 + part.3;
        }
    }
    println!("Part A: {}", total);
}

fn is_accepted(part: (u32, u32, u32, u32), workflows: &HashMap<String, Vec<String>>) -> bool {
    let mut workflow = "in".to_string();

    loop {
        if workflow == "A" {
            return true;
        }
        if workflow == "R" {
            return false;
        }
        let mut rules_stack = workflows.get(&workflow).unwrap().clone();
        rules_stack.reverse();
        while !rules_stack.is_empty() {
            let rule = rules_stack.pop().unwrap();
            if rule == "A" {
                return true;
            }
            if rule == "R" {
                return false;
            }
            if rules_stack.is_empty() {
                workflow = rule;
                break;
            }
            let rule_parts = rule.split(":").collect::<Vec<&str>>();
            match rule_parts[0][..1].to_string().as_str() {
                "x" => {
                    if check_rule(rule_parts[0].to_string(), part.0) {
                        workflow = rule_parts[1].to_string();
                        break;
                    }
                }
                "m" => {
                    if check_rule(rule_parts[0].to_string(), part.1) {
                        workflow = rule_parts[1].to_string();
                        break;
                    }
                }
                "a" => {
                    if check_rule(rule_parts[0].to_string(), part.2) {
                        workflow = rule_parts[1].to_string();
                        break;
                    }
                }
                "s" => {
                    if check_rule(rule_parts[0].to_string(), part.3) {
                        workflow = rule_parts[1].to_string();
                        break;
                    }
                }
                _ => {
                    panic!("Unknown rule type");
                }
            }
        }
    }
}

fn check_rule(rule: String, val: u32) -> bool {
    match rule[1..2].to_string().as_str() {
        ">" => {
            if val > rule[2..].parse::<u32>().unwrap() {
                return true;
            }
        }
        "<" => {
            if val < rule[2..].parse::<u32>().unwrap() {
                return true;
            }
        }
        _ => {
            panic!("Unknown comparison");
        }
    }
    false
}
