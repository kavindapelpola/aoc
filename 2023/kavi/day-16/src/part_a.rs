use crate::App;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const SIZE: usize = 110;

pub fn run() {
    let app = App::new();
    let mut map = vec![vec!['.'; SIZE]; SIZE];
    let mut heads = vec![(0, 0, Direction::Right)];
    let mut visited = vec![];
    while !heads.is_empty() {
        let mut new_heads = vec![];
        for (y, x, d) in heads {
            if visited.contains(&(y, x, d.clone())) {
                continue;
            }
            visited.push((y, x, d.clone()));
            map[y][x] = '#';
            let mut hit_mirror = false;
            for (mirror_y, mirror_x, mirror) in &app.input {
                if *mirror_y == y && *mirror_x == x {
                    match (mirror, &d) {
                        ('/', Direction::Up) => {
                            if let Some((y, x)) = moved_xy(y, x, &Direction::Right) {
                                new_heads.push((y, x, Direction::Right));
                            }
                        }
                        ('/', Direction::Down) => {
                            if let Some((y, x)) = moved_xy(y, x, &Direction::Left) {
                                new_heads.push((y, x, Direction::Left));
                            }
                        }
                        ('/', Direction::Left) => {
                            if let Some((y, x)) = moved_xy(y, x, &Direction::Down) {
                                new_heads.push((y, x, Direction::Down));
                            }
                        }
                        ('/', Direction::Right) => {
                            if let Some((y, x)) = moved_xy(y, x, &Direction::Up) {
                                new_heads.push((y, x, Direction::Up));
                            }
                        }
                        ('\\', Direction::Up) => {
                            if let Some((y, x)) = moved_xy(y, x, &Direction::Left) {
                                new_heads.push((y, x, Direction::Left));
                            }
                        }
                        ('\\', Direction::Down) => {
                            if let Some((y, x)) = moved_xy(y, x, &Direction::Right) {
                                new_heads.push((y, x, Direction::Right));
                            }
                        }
                        ('\\', Direction::Left) => {
                            if let Some((y, x)) = moved_xy(y, x, &Direction::Up) {
                                new_heads.push((y, x, Direction::Up));
                            }
                        }
                        ('\\', Direction::Right) => {
                            if let Some((y, x)) = moved_xy(y, x, &Direction::Down) {
                                new_heads.push((y, x, Direction::Down));
                            }
                        }
                        ('|', Direction::Up) => {
                            if let Some((y, x)) = moved_xy(y, x, &Direction::Up) {
                                new_heads.push((y, x, Direction::Up));
                            }
                        }
                        ('|', Direction::Down) => {
                            if let Some((y, x)) = moved_xy(y, x, &Direction::Down) {
                                new_heads.push((y, x, Direction::Down));
                            }
                        }
                        ('|', Direction::Right) => {
                            if let Some((y, x)) = moved_xy(y, x, &Direction::Up) {
                                new_heads.push((y, x, Direction::Up));
                            }
                            if let Some((y, x)) = moved_xy(y, x, &Direction::Down) {
                                new_heads.push((y, x, Direction::Down));
                            }
                        }
                        ('|', Direction::Left) => {
                            if let Some((y, x)) = moved_xy(y, x, &Direction::Up) {
                                new_heads.push((y, x, Direction::Up));
                            }
                            if let Some((y, x)) = moved_xy(y, x, &Direction::Down) {
                                new_heads.push((y, x, Direction::Down));
                            }
                        }
                        ('-', Direction::Up) => {
                            if let Some((y, x)) = moved_xy(y, x, &Direction::Left) {
                                new_heads.push((y, x, Direction::Left));
                            }
                            if let Some((y, x)) = moved_xy(y, x, &Direction::Right) {
                                new_heads.push((y, x, Direction::Right));
                            }
                        }
                        ('-', Direction::Down) => {
                            if let Some((y, x)) = moved_xy(y, x, &Direction::Left) {
                                new_heads.push((y, x, Direction::Left));
                            }
                            if let Some((y, x)) = moved_xy(y, x, &Direction::Right) {
                                new_heads.push((y, x, Direction::Right));
                            }
                        }
                        ('-', Direction::Left) => {
                            if let Some((y, x)) = moved_xy(y, x, &Direction::Left) {
                                new_heads.push((y, x, Direction::Left));
                            }
                        }
                        ('-', Direction::Right) => {
                            if let Some((y, x)) = moved_xy(y, x, &Direction::Right) {
                                new_heads.push((y, x, Direction::Right));
                            }
                        }
                        _ => (),
                    };
                    hit_mirror = true;
                    break;
                }
            }
            if !hit_mirror {
                if let Some((y, x)) = moved_xy(y, x, &d) {
                    new_heads.push((y, x, d.clone()));
                }
            }
        }
        heads = new_heads;
    }
    let mut total = 0;
    for line in &map {
        total += line.iter().filter(|c| **c == '#').count();
    }
    println!("Part A: {}\n", total);
}

fn moved_xy(y: usize, x: usize, d: &Direction) -> Option<(usize, usize)> {
    match d {
        Direction::Up => {
            if y > 0 {
                return Some((y - 1, x));
            }
        }
        Direction::Down => {
            if y < SIZE - 1 {
                return Some((y + 1, x));
            }
        }
        Direction::Left => {
            if x > 0 {
                return Some((y, x - 1));
            }
        }
        Direction::Right => {
            if x < SIZE - 1 {
                return Some((y, x + 1));
            }
        }
    }
    None
}
