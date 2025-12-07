use crate::App;
use rayon::prelude::*;
use std::{collections::HashSet, sync::Mutex};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const SIZE: usize = 110;

pub fn run() {
    let app = App::new();
    let largest = Mutex::new(0);
    (0..SIZE).collect::<Vec<usize>>().par_iter().for_each(|i| {
        let energised = parta((*i, 0, Direction::Right), &app.input);
        if energised > largest.lock().unwrap().clone() {
            *largest.lock().unwrap() = energised;
        }
    });
    (0..SIZE).collect::<Vec<usize>>().par_iter().for_each(|i| {
        let energised = parta((*i, SIZE - 1, Direction::Left), &app.input);
        if energised > largest.lock().unwrap().clone() {
            *largest.lock().unwrap() = energised;
        }
    });
    (0..SIZE).collect::<Vec<usize>>().par_iter().for_each(|i| {
        let energised = parta((0, *i, Direction::Down), &app.input);
        if energised > largest.lock().unwrap().clone() {
            *largest.lock().unwrap() = energised;
        }
    });
    (0..SIZE).collect::<Vec<usize>>().par_iter().for_each(|i| {
        let energised = parta((SIZE - 1, *i, Direction::Up), &app.input);
        if energised > largest.lock().unwrap().clone() {
            *largest.lock().unwrap() = energised;
        }
    });
    println!("Part B: {}", largest.lock().unwrap());
}

fn parta(start: (usize, usize, Direction), mirrors: &Vec<(usize, usize, char)>) -> usize {
    let mut map = vec![vec!['.'; SIZE]; SIZE];
    let mut heads = vec![start];
    let mut visited = HashSet::new();
    while !heads.is_empty() {
        let mut new_heads = vec![];
        for (y, x, d) in heads {
            if visited.contains(&(y, x, d.clone())) {
                continue;
            }
            visited.insert((y, x, d.clone()));
            map[y][x] = '#';
            let mut hit_mirror = false;
            for (mirror_y, mirror_x, mirror) in mirrors {
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
    total
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
