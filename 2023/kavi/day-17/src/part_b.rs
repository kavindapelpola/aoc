use crate::{
    part_a::{neighbours, Dir, State},
    App,
};
use std::collections::HashMap;

pub fn run() {
    let app = App::new();
    println!("Part B: {}", find_least(&app.input));
}

pub fn find_least(input: &Vec<Vec<u32>>) -> u32 {
    let mut frontier = vec![];
    let mut lowest_map: HashMap<(usize, usize, Dir), u32> = HashMap::new();
    lowest_map.insert((0, 0, Dir::Right), 0);
    frontier.push(State {
        pos: (0, 0),
        dir: Dir::Right,
        total: 0,
    });
    while !frontier.is_empty() {
        let cur = frontier.pop().unwrap();
        let neighbours = neighbours(&cur, input, 1..=10, 4);
        for n in neighbours {
            let key = (n.pos.0.clone(), n.pos.1.clone(), n.dir.clone());
            if lowest_map.contains_key(&key) {
                if lowest_map[&key] > n.total {
                    lowest_map.insert(key, n.total);
                    frontier.push(n);
                }
            } else {
                lowest_map.insert(key, n.total);
                frontier.push(n);
            }
        }
    }

    lowest_map
        .iter()
        .filter(|i| i.0 .0 == input.len() - 1 && i.0 .1 == input.len() - 1)
        .min_by(|a, b| a.1.cmp(b.1))
        .map(|i| i.1)
        .cloned()
        .unwrap()
}
