use crate::App;
use std::{collections::HashMap, ops::RangeInclusive};

pub fn run() {
    let app = App::new();
    println!("Part A: {}", find_least(&app.input));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct State {
    pub pos: (usize, usize),
    pub dir: Dir,
    pub total: u32,
}

pub fn find_least(input: &Vec<Vec<u32>>) -> u32 {
    let mut frontier = Vec::with_capacity(1_000_000);
    let mut lowest_map: HashMap<(usize, usize, Dir), u32> = HashMap::new();
    lowest_map.insert((0, 0, Dir::Right), 0);
    frontier.push(State {
        pos: (0, 0),
        dir: Dir::Right,
        total: 0,
    });
    while !frontier.is_empty() {
        let cur = frontier.pop().unwrap();
        let neighbours = neighbours(&cur, input, 1..=3, 1);
        for n in neighbours {
            let key = (n.pos.0.clone(), n.pos.1.clone(), n.dir.clone());
            if lowest_map.contains_key(&key) {
                if lowest_map[&key] > n.total {
                    lowest_map.insert(key, n.total);
                    frontier.push(n);
                }
            } else {
                lowest_map.insert(key, n.total);
                frontier.push(n);
            }
        }
    }

    lowest_map
        .iter()
        .filter(|i| i.0 .0 == input.len() - 1 && i.0 .1 == input.len() - 1)
        .min_by(|a, b| a.1.cmp(b.1))
        .map(|i| i.1)
        .cloned()
        .unwrap()
}

pub fn neighbours(
    cur: &State,
    input: &Vec<Vec<u32>>,
    range: RangeInclusive<usize>,
    min: usize,
) -> Vec<State> {
    let mut v = Vec::with_capacity(20);
    let (y, x) = cur.pos;
    match cur.dir {
        Dir::Up => {
            let mut total = cur.total;
            for i in range {
                if y < i {
                    break;
                }
                total += input[y - i][x];
                if i < min {
                    continue;
                }
                v.push(State {
                    pos: (y - i, x),
                    dir: Dir::Left,
                    total,
                });
                v.push(State {
                    pos: (y - i, x),
                    dir: Dir::Right,
                    total,
                });
            }
        }
        Dir::Down => {
            let mut total = cur.total;
            for i in range {
                if y + i > input.len() - 1 {
                    break;
                }
                total += input[y + i][x];
                if i < min {
                    continue;
                }
                v.push(State {
                    pos: (y + i, x),
                    dir: Dir::Left,
                    total,
                });
                v.push(State {
                    pos: (y + i, x),
                    dir: Dir::Right,
                    total,
                });
            }
        }
        Dir::Right => {
            let mut total = cur.total;
            for i in range {
                if x + i > input.len() - 1 {
                    break;
                }
                total += input[y][x + i];
                if i < min {
                    continue;
                }
                v.push(State {
                    pos: (y, x + i),
                    dir: Dir::Up,
                    total,
                });
                v.push(State {
                    pos: (y, x + i),
                    dir: Dir::Down,
                    total,
                });
            }
        }
        Dir::Left => {
            let mut total = cur.total;
            for i in range {
                if i > x {
                    break;
                }
                total += input[y][x - i];
                if i < min {
                    continue;
                }
                v.push(State {
                    pos: (y, x - i),
                    dir: Dir::Up,
                    total,
                });
                v.push(State {
                    pos: (y, x - i),
                    dir: Dir::Down,
                    total,
                });
            }
        }
    }
    v
}
