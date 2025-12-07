use std::collections::HashSet;

use crate::{part_a::move_rocks, part_a::SIZE, App};

pub fn run() {
    let app = App::new();
    let (mut rocks, mut hashes) = app.input;

    rocks.sort_by(|a, b| a.0.cmp(&b.0));
    rocks.sort_by(|a, b| a.1.cmp(&b.1));

    let mut rock_memory = vec![];
    let mut load_memory = vec![];
    let repeat_start;
    loop {
        for _ in 0..4 {
            rocks = move_rocks(&rocks, &hashes);
            rocks = rotate_rocks_clockwise(&rocks);
            hashes = rotate_rocks_clockwise(&hashes);
        }
        rocks.sort_by(|a, b| a.0.cmp(&b.0));
        rocks.sort_by(|a, b| a.1.cmp(&b.1));

        if rock_memory.contains(&rocks) {
            repeat_start = rock_memory.iter().position(|x| *x == rocks).unwrap();
            break;
        }
        rock_memory.push(rocks.clone());
        let mut tot = 0;
        for (y, _) in &rocks {
            tot += SIZE - y;
        }
        load_memory.push(tot);
    }
    let repeat_len = load_memory.len() - repeat_start;
    println!(
        "Part B: {}",
        load_memory[repeat_start + (1_000_000_000 - repeat_start) % repeat_len - 1]
    );
}

fn rotate_rocks_clockwise(stuff: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut new_stuff = vec![];
    for stuff in stuff {
        let (y, x) = stuff;
        new_stuff.push((*x, SIZE - *y - 1));
    }
    new_stuff
}
