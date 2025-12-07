use crate::App;

pub const SIZE: usize = 100;

pub fn run() {
    let app = App::new();
    let (mut rocks, hashes) = app.input;
    rocks = move_rocks(&rocks, &hashes);
    let mut tot = 0;
    for (y, _) in rocks {
        tot += SIZE - y;
    }
    println!("Part A: {}", tot);
}

pub fn move_rocks(
    rocks: &Vec<(usize, usize)>,
    hashes: &Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut sorted_rocks = rocks.clone();
    sorted_rocks.sort_by(|a, b| a.0.cmp(&b.0));
    let mut new_rocks = vec![];
    for rock in sorted_rocks {
        let mut found = false;
        let (mut y, x) = rock;
        while y > 0 && !found {
            for (hy, hx) in hashes {
                if *hy == y - 1 && *hx == x {
                    found = true;
                    break;
                }
            }
            for (ry, rx) in &new_rocks {
                if *ry == y - 1 && *rx == x {
                    found = true;
                    break;
                }
            }
            if !found {
                y -= 1;
            }
        }
        new_rocks.push((y, x));
    }
    new_rocks
}
