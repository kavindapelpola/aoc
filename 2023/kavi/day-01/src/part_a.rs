use crate::App;

pub fn run() {
    let app = App::new();
    println!(
        "Part A: {}",
        app.input.iter().map(|l| App::digits(l)).sum::<u32>()
    );
}
