use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut heads = vec![lines[0].find("S").unwrap()];
    let mut split = 0;
    for row in 0..lines.len() - 1 {
        let mut new_heads = vec![];
        heads.iter().for_each(|h| {
            // try to move down, if theres a splitter then split
            if lines[row + 1].chars().nth(*h) == Some('.') {
                new_heads.push(*h);
            } else {
                split += 1;
                new_heads.push(h - 1);
                new_heads.push(h + 1);
            }
        });
        new_heads.dedup();
        heads = new_heads;
    }
    println!("{}", split);
}

fn part2(input: &str) {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut heads: HashMap<usize, u64> = HashMap::new();
    heads.insert(lines[0].find("S").unwrap(), 1);
    let mut timelines: u64 = 1;
    for row in 0..lines.len() - 1 {
        let mut new_heads: HashMap<usize, u64> = HashMap::new();
        heads.iter().for_each(|(col, count)| {
            // try to move down, if theres a splitter then split
            if lines[row + 1].chars().nth(*col) == Some('.') {
                new_heads
                    .entry(*col)
                    .and_modify(|e| *e += *count)
                    .or_insert(*count);
            } else {
                timelines += count;
                new_heads
                    .entry(col - 1)
                    .and_modify(|e| *e += *count)
                    .or_insert(*count);
                new_heads
                    .entry(col + 1)
                    .and_modify(|e| *e += *count)
                    .or_insert(*count);
            }
        });
        heads = new_heads;
    }
    println!("{}", timelines);
}
