use crate::App;

pub fn run() {
    let app = App::new();
    println!(
        "Part B: {}",
        app.input.iter().map(|l| App::word_digits(&l)).sum::<u32>()
    );
}
