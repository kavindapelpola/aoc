use crate::App;

pub fn run() {
    let app = App::new();
    let mut sum_parts = 0;
    for symbol in &app.symbols {
        for part in &app.parts {
            if App::touches_part(&symbol.1, part) {
                sum_parts += part.0;
                continue;
            }
        }
    }
    println!("Part A: {}", sum_parts);
}
