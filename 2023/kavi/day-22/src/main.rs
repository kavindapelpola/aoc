mod load;
mod part_a;
mod part_b;
mod x;

pub type Coord = (usize, usize, usize);

struct App {
    input: Vec<(Coord, Coord)>,
}

impl App {
    fn new() -> App {
        App {
            input: load::load(),
        }
    }
}

fn main() {
    println!("{:?}", x::run());
    //    part_a::run();
    part_b::run();
}
