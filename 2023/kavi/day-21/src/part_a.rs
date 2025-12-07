use std::collections::HashSet;

use crate::{App, Coord};

pub fn run() {
    let app = App::new();
    println!(
        "Part A: {}",
        search(64, app.start, &app.rocks, app.size).len()
    );
}

pub fn search(
    max_steps: usize,
    start: Coord,
    rocks: &HashSet<Coord>,
    size: usize,
) -> HashSet<Coord> {
    let mut frontier = HashSet::new();
    frontier.insert(start);
    for _ in 0..max_steps {
        let mut new_frontier = HashSet::new();
        for pos in frontier {
            for new_pos in get_moves(pos, rocks, size) {
                new_frontier.insert(new_pos);
            }
        }
        frontier = new_frontier;
    }
    frontier
}

fn get_moves(pos: Coord, rocks: &HashSet<Coord>, size: usize) -> Vec<Coord> {
    let mut moves = vec![];
    let (y, x) = pos;
    if y > 0 && !rocks.contains(&(y - 1, x)) {
        moves.push((y - 1, x));
    }
    if y < size as i64 - 1 && !rocks.contains(&(y + 1, x)) {
        moves.push((y + 1, x));
    }
    if x > 0 && !rocks.contains(&(y, x - 1)) {
        moves.push((y, x - 1));
    }
    if x < size as i64 - 1 && !rocks.contains(&(y, x + 1)) {
        moves.push((y, x + 1));
    }

    moves
}
