use crate::App;

pub fn run() {
    let app = App::new();
    let mut card_count = vec![1; app.input.len()];
    for (i, (win, my)) in app.input.iter().enumerate() {
        let count = my.iter().filter(|&x| win.contains(x)).count();
        for j in (i + 1)..(i + 1 + count) {
            if j >= app.input.len() {
                break;
            }
            card_count[j] += card_count[i];
        }
    }
    println!("Part B: {}", card_count.iter().sum::<usize>());
}
