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
    let mut total: u64 = 0;
    for i in 0..gal.len() - 1 {
        for j in i + 1..gal.len() {
            let (y1, x1) = gal[i];
            let (y2, x2) = gal[j];
            let mut spacers = 0;
            if y1 > y2 {
                for a in y2..y1 {
                    if app.input[a as usize].chars().nth(x1 as usize) == Some('X') {
                        spacers += 1;
                    }
                }
            } else {
                for a in y1..y2 {
                    if app.input[a as usize].chars().nth(x1 as usize) == Some('X') {
                        spacers += 1;
                    }
                }
            }
            if x1 > x2 {
                for a in x2..x1 {
                    if app.input[y2 as usize].chars().nth(a as usize) == Some('X') {
                        spacers += 1;
                    }
                }
            } else {
                for a in x1..x2 {
                    if app.input[y1 as usize].chars().nth(a as usize) == Some('X') {
                        spacers += 1;
                    }
                }
            }
            let d = (y1 - y2).abs() as u64 + (x1 - x2).abs() as u64 - spacers
                + spacers * (1_000_000 - 1);
            total += d;
        }
    }
    println!("Part B: {}", total)
}
