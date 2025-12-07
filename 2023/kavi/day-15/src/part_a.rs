use crate::App;

pub fn run() {
    let app = App::new();
    let mut tot = 0;
    for i in app.input {
        tot += hash(i);
    }
    println!("Part A: {}", tot);
}

pub fn hash(s: String) -> u64 {
    let mut res = 0;
    for c in s.chars() {
        res += c as u64;
        res *= 17;
        res = res % 256;
    }
    res
}
