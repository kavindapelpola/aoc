mod load;
mod part_a;
mod part_b;

struct App {
    symbols: Vec<(char, (usize, usize))>,
    parts: Vec<(u32, Vec<(usize, usize)>)>,
}

impl App {
    fn new() -> App {
        let l = load::load();
        App {
            symbols: l.0,
            parts: l.1,
        }
    }

    fn touches_part(pos: &(usize, usize), part: &(u32, Vec<(usize, usize)>)) -> bool {
        if pos.0 > 0 && pos.1 > 0 && part.1.contains(&(pos.0 - 1, pos.1 - 1)) {
            return true;
        }
        if pos.0 > 0 && part.1.contains(&(pos.0 - 1, pos.1)) {
            return true;
        }
        if pos.0 > 0 && part.1.contains(&(pos.0 - 1, pos.1 + 1)) {
            return true;
        }
        if pos.1 > 0 && part.1.contains(&(pos.0, pos.1 - 1)) {
            return true;
        }
        if pos.1 > 0 && part.1.contains(&(pos.0, pos.1 + 1)) {
            return true;
        }
        if pos.1 > 0 && part.1.contains(&(pos.0 + 1, pos.1 - 1)) {
            return true;
        }
        if part.1.contains(&(pos.0 + 1, pos.1)) {
            return true;
        }
        if part.1.contains(&(pos.0 + 1, pos.1 + 1)) {
            return true;
        }
        false
    }
}

fn main() {
    part_a::run();
    part_b::run();
}
