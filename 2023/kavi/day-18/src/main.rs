mod load;
mod part_a;
mod part_b;

struct App {
    input: Vec<(char, usize, String)>,
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
