use polyfit_rs::polyfit_rs::polyfit;
use std::collections::HashSet;

use crate::{App, Coord};

pub fn run() {
    let app = App::new();
    let x = vec![65, 65 + 131, 65 + 131 * 2];
    let y = x
        .iter()
        .map(|x| search(*x, app.start, &app.rocks, app.size).len())
        .collect::<Vec<usize>>();

    let xf = x.iter().map(|x| *x as f64).collect::<Vec<f64>>();
    let yf = y.iter().map(|y| *y as f64).collect::<Vec<f64>>();
    let ans = polyfit(&xf, &yf, 2).unwrap();
    let n = 26_501_365.;
    println!("Part B: {}", n * n * ans[2] + n * ans[1] + ans[0]);
    // 621289922886149.1
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

fn get_moves(pos: Coord, rocks: &HashSet<Coord>, size: usize) -> HashSet<Coord> {
    let mut moves = HashSet::new();
    let (actual_y, actual_x) = pos;
    let mut y = actual_y % size as i64;
    let mut x = actual_x % size as i64;
    if y < 0 {
        y = size as i64 + y;
    }
    if x < 0 {
        x = size as i64 + x;
    }
    if !rocks.contains(&(y - 1, x)) {
        moves.insert((actual_y - 1, actual_x));
    }
    if !rocks.contains(&(y + 1, x)) {
        moves.insert((actual_y + 1, actual_x));
    }
    if !rocks.contains(&(y, x - 1)) {
        moves.insert((actual_y, actual_x - 1));
    }
    if !rocks.contains(&(y, x + 1)) {
        moves.insert((actual_y, actual_x + 1));
    }

    moves
}
