use std::collections::HashSet;

use crate::Coord;

pub fn load() -> (Coord, HashSet<Coord>, usize) {
    let input = include_str!("../input.txt");
    let mut rocks = HashSet::new();
    let mut start = (0, 0);
    let mut size = 0;
    for (y, line) in input.lines().enumerate() {
        size = line.len();
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                rocks.insert((y as i64, x as i64));
            }
            if c == 'S' {
                start = (y as i64, x as i64);
            }
        }
    }
    (start, rocks, size)
}
