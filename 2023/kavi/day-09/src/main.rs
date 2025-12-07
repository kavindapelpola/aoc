mod load;
mod part_a;
mod part_b;

struct App {
    input: Vec<Vec<i64>>,
}

impl App {
    fn new() -> App {
        App {
            input: load::load(),
        }
    }
}

fn main() {
    part_a::run();
    part_b::run();
}
