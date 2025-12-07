use crate::App;

pub fn run() {
    let app = App::new();
    let mut sum = 0;
    let base: i32 = 2;
    for (win, my) in app.input {
        let count = my.iter().filter(|&x| win.contains(x)).count();
        if count == 0 {
            continue;
        }
        sum += base.pow(count as u32 - 1);
    }
    println!("Part A: {}", sum);
}
