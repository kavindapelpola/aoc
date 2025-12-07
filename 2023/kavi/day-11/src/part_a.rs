use crate::App;

pub fn run() {
    let app = App::new();
    let mut gal = vec![];
    for (y, line) in app.input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                gal.push((y as i32, x as i32));
            }
        }
    }
    let mut total = 0;
    for i in 0..gal.len() - 1 {
        for j in i + 1..gal.len() {
            let (y1, x1) = gal[i];
            let (y2, x2) = gal[j];
            let d = (y1 - y2).abs() + (x1 - x2).abs();
            total += d;
        }
    }
    println!("Part A: {}", total)
}
