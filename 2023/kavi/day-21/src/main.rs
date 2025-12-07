use std::collections::HashSet;

mod load;
mod part_a;
mod part_b;

pub type Coord = (i64, i64);

struct App {
    start: Coord,
    rocks: HashSet<Coord>,
    size: usize,
}

impl App {
    fn new() -> App {
        let (start, rocks, size) = load::load();
        App { start, rocks, size }
    }
}

fn main() {
    part_a::run();
    part_b::run();
}
