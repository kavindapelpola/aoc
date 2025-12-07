use crate::App;

pub fn run() {
    let app = App::new();

    let mut sum = 0;
    for (j, (lhs, rhs)) in app.input.iter().enumerate() {
        if lhs <= rhs {
            sum += j + 1;
        }
    }
    println!("Part A: {}", sum);
}
