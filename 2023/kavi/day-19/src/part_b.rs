use std::collections::HashMap;

use crate::App;

pub fn run() {
    let app = App::new();
    let mut tot: u64 = 0;
    let (_, workflows) = app.input;
    let paths = accepted_paths(&workflows);

    let mut included_ranges = vec![];

    for path_step in &paths {
        let mut x_low = 1;
        let mut x_high = 4000;
        let mut m_low = 1;
        let mut m_high = 4000;
        let mut a_low = 1;
        let mut a_high = 4000;
        let mut s_low = 1;
        let mut s_high = 4000;

        for (_, rules) in path_step {
            if rules.is_empty() {
                continue;
            }
            let mut rule_stack = rules.clone();
            rule_stack.reverse();
            while rule_stack.len() > 1 {
                let not_rule = rule_stack.pop().unwrap();
                if !not_rule.contains(":") {
                    continue;
                }
                match not_rule[0..1].to_string().as_str() {
                    "x" => match not_rule[1..2].to_string().as_str() {
                        ">" => {
                            x_high = min(x_high, rule_val(&not_rule));
                        }
                        "<" => {
                            x_low = max(x_low, rule_val(&not_rule));
                        }
                        _ => {
                            panic!("Unknown comparison");
                        }
                    },
                    "m" => match not_rule[1..2].to_string().as_str() {
                        ">" => {
                            m_high = min(m_high, rule_val(&not_rule));
                        }
                        "<" => {
                            m_low = max(m_low, rule_val(&not_rule));
                        }
                        _ => {
                            panic!("Unknown comparison");
                        }
                    },
                    "a" => match not_rule[1..2].to_string().as_str() {
                        ">" => {
                            a_high = min(a_high, rule_val(&not_rule));
                        }
                        "<" => {
                            a_low = max(a_low, rule_val(&not_rule));
                        }
                        _ => {
                            panic!("Unknown comparison");
                        }
                    },
                    "s" => match not_rule[1..2].to_string().as_str() {
                        ">" => {
                            s_high = min(s_high, rule_val(&not_rule));
                        }
                        "<" => {
                            s_low = max(s_low, rule_val(&not_rule));
                        }
                        _ => {
                            panic!("Unknown comparison");
                        }
                    },
                    _ => {
                        panic!("here");
                    }
                }
            }
            let yes_rule = rule_stack.pop().unwrap();
            if yes_rule.contains(":") {
                match yes_rule[0..1].to_string().as_str() {
                    "x" => match yes_rule[1..2].to_string().as_str() {
                        ">" => {
                            x_low = max(x_low, rule_val(&yes_rule));
                        }
                        "<" => {
                            x_high = min(x_high, rule_val(&yes_rule));
                        }
                        _ => {
                            panic!("Unknown comparison");
                        }
                    },
                    "m" => match yes_rule[1..2].to_string().as_str() {
                        ">" => {
                            m_low = max(m_low, rule_val(&yes_rule));
                        }
                        "<" => {
                            m_high = min(m_high, rule_val(&yes_rule));
                        }
                        _ => {
                            panic!("Unknown comparison");
                        }
                    },
                    "a" => match yes_rule[1..2].to_string().as_str() {
                        ">" => {
                            a_low = max(a_low, rule_val(&yes_rule));
                        }
                        "<" => {
                            a_high = min(a_high, rule_val(&yes_rule));
                        }
                        _ => {
                            panic!("Unknown comparison");
                        }
                    },
                    "s" => match yes_rule[1..2].to_string().as_str() {
                        ">" => {
                            s_low = max(s_low, rule_val(&yes_rule));
                        }
                        "<" => {
                            s_high = min(s_high, rule_val(&yes_rule));
                        }
                        _ => {
                            panic!("Unknown comparison");
                        }
                    },
                    _ => {
                        panic!("here");
                    }
                }
            }
        }

        included_ranges.push((x_low, x_high, m_low, m_high, a_low, a_high, s_low, s_high));

        println!();
    }

    let mut done_x = vec![(included_ranges[0].0, included_ranges[0].1)];
    let mut done_m = vec![(included_ranges[0].2, included_ranges[0].3)];
    let mut done_a = vec![(included_ranges[0].4, included_ranges[0].5)];
    let mut done_s = vec![(included_ranges[0].6, included_ranges[0].7)];
    tot = (done_x[0].1 - done_x[0].0) as u64
        * (done_m[0].1 - done_m[0].0) as u64
        * (done_a[0].1 - done_a[0].0) as u64
        * (done_s[0].1 - done_s[0].0) as u64;
    for (x_low, x_high, m_low, m_high, a_low, a_high, s_low, s_high) in
        included_ranges[1..].to_vec()
    {
        println!(
            "{} {} {} {} {} {} {} {}",
            x_low, x_high, m_low, m_high, a_low, a_high, s_low, s_high
        );
        let mut this_done_x: Vec<(u32, u32)> = vec![(x_low, x_high)];
        for r in done_x.clone() {
            let y = this_done_x.clone();
            this_done_x.clear();
            for q in y {
                this_done_x.extend(unique_range(q.0, q.1, r.0, r.1));
            }
        }
        done_x.extend(this_done_x.clone());

        let mut this_done_m: Vec<(u32, u32)> = vec![(m_low, m_high)];
        for r in done_m.clone() {
            let y = this_done_m.clone();
            this_done_m.clear();
            for q in y {
                this_done_m.extend(unique_range(q.0, q.1, r.0, r.1));
            }
        }
        done_m.extend(this_done_m.clone());

        let mut this_done_a: Vec<(u32, u32)> = vec![(a_low, a_high)];
        for r in done_a.clone() {
            let y = this_done_a.clone();
            this_done_a.clear();
            for q in y {
                this_done_a.extend(unique_range(q.0, q.1, r.0, r.1));
            }
        }
        done_a.extend(this_done_a.clone());

        let mut this_done_s: Vec<(u32, u32)> = vec![(s_low, s_high)];
        for r in done_s.clone() {
            let y = this_done_s.clone();
            this_done_s.clear();
            for q in y {
                this_done_s.extend(unique_range(q.0, q.1, r.0, r.1));
            }
        }
        done_s.extend(this_done_s.clone());

        println!("tx - {:?}", this_done_x);
        println!("tm - {:?}", this_done_m);
        println!("ta - {:?}", this_done_a);
        println!("ts - {:?}", this_done_s);

        println!("x - {:?}", done_x);
        println!("m - {:?}", done_m);
        println!("a - {:?}", done_a);
        println!("s - {:?}", done_s);

        if this_done_x.is_empty()
            && this_done_a.is_empty()
            && this_done_m.is_empty()
            && this_done_s.is_empty()
        {
            continue;
        }

        let mut to_add = 1;

        to_add *= (x_high - x_low) as u64;
        to_add *= (m_high - m_low) as u64;
        to_add *= (a_high - a_low) as u64;
        to_add *= (s_high - s_low) as u64;

        // if this_done_x.is_empty() {
        //     to_add *= (x_high - x_low) as u64;
        // } else {
        //     to_add *= (this_done_x[0].1 - this_done_x[0].0) as u64;
        // }

        // if this_done_m.is_empty() {
        //     to_add *= (m_high - m_low) as u64;
        // } else {
        //     to_add *= (this_done_m[0].1 - this_done_m[0].0) as u64;
        // }

        // if this_done_a.is_empty() {
        //     to_add *= (a_high - a_low) as u64;
        // } else {
        //     to_add *= (this_done_a[0].1 - this_done_a[0].0) as u64;
        // }

        // if this_done_s.is_empty() {
        //     to_add *= (s_high - s_low) as u64;
        // } else {
        //     to_add *= (this_done_s[0].1 - this_done_s[0].0) as u64;
        // }

        tot += to_add;
        println!("to_add -\n{}\n{}", to_add, tot);

        println!();
    }

    println!("{}", 4000 as u64 * 4000 * 4000 * 4000);
    println!("167409079868000");
    println!("{}", tot);
}

fn unique_range(
    this_lower: u32,
    this_upper: u32,
    other_lower: u32,
    other_upper: u32,
) -> Vec<(u32, u32)> {
    //   ---------------------------------->
    //                     |<---this---->|
    //    |<---other---->|
    if this_lower >= other_upper {
        return vec![(this_lower, this_upper)];
    }

    //   ---------------------------------->
    //     |<---this---->|
    //                     |<---other---->|
    if this_upper <= other_lower {
        return vec![(this_lower, this_upper)];
    }

    //   ---------------------------------->
    //                |<---this---->|
    //             |<------other------>|
    if this_lower >= other_lower && this_upper <= other_upper {
        return vec![];
    }

    //   ---------------------------------->
    //             |<------this--------->|
    //                 |<---other---->|
    if this_lower < other_lower && this_upper > other_upper {
        return vec![(this_lower, other_lower), (other_upper, this_upper)];
    }

    //   ---------------------------------->
    //            |<---this---->|
    //                     |<---other---->|
    if this_lower < other_lower && this_upper > other_lower {
        return vec![(this_lower, other_lower)];
    }

    //   ---------------------------------->
    //                   |<---this---->|
    //           |<---other---->|
    if this_upper > other_upper && this_lower < other_upper {
        return vec![(other_upper, this_upper)];
    }

    println!(
        "{} {} {} {}",
        this_lower, this_upper, other_lower, other_upper
    );
    panic!("here");
}

fn min(x: u32, y: u32) -> u32 {
    if x < y {
        x
    } else {
        y
    }
}

fn max(x: u32, y: u32) -> u32 {
    if x > y {
        x
    } else {
        y
    }
}

fn rule_val(rule: &String) -> u32 {
    let x = rule.split(":").collect::<Vec<&str>>()[0].to_string();
    x[2..].to_string().parse().unwrap()
}

struct State {
    path: Vec<(String, Vec<String>)>,
}

fn accepted_paths(workflows: &HashMap<String, Vec<String>>) -> Vec<Vec<(String, Vec<String>)>> {
    let mut accepted = vec![];

    let mut frontier = vec![State {
        path: vec![("in".to_string(), vec![])],
    }];

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();
        let rules_stack = workflows
            .get(&current.path.last().unwrap().0)
            .unwrap()
            .clone();
        let mut combined_rules = vec![];
        for rule in rules_stack {
            combined_rules.push(rule.clone());
            if rule == "A" {
                let mut new_path = current.path.clone();
                new_path.push(("".to_string(), combined_rules.clone()));
                accepted.push(new_path);
            } else if rule == "R" {
                continue;
            } else {
                let mut new_path = current.path.clone();
                if rule.contains(":") {
                    let x = rule.split(":").collect::<Vec<&str>>()[1].to_string();
                    if x == "A" {
                        new_path.push((x, combined_rules.clone()));
                        accepted.push(new_path);
                        continue;
                    }
                    if x == "R" {
                        continue;
                    }
                    new_path.push((x, combined_rules.clone()));
                } else {
                    new_path.push((rule, combined_rules.clone()));
                }
                frontier.push(State { path: new_path });
            }
        }
    }

    accepted.dedup();
    accepted
}
