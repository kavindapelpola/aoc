use crate::App;
use std::{cmp::Ordering, collections::HashMap};

pub fn run() {
    let app = App::new();

    let mut sorted = app.input.clone();
    sorted.sort_by(|a, b| {
        let a_type = hand_type(a.0.clone());
        let b_type = hand_type(b.0.clone());
        if a_type > b_type {
            return Ordering::Greater;
        }
        if a_type == b_type {
            for i in 0..(a.0.len()) {
                let a_char = char_to_i(a.0.chars().nth(i).unwrap());
                let b_char = char_to_i(b.0.chars().nth(i).unwrap());
                if a_char > b_char {
                    return Ordering::Greater;
                }
                if a_char < b_char {
                    return Ordering::Less;
                }
            }
        }
        return Ordering::Less;
    });

    println!(
        "Part A: {}",
        sorted
            .iter()
            .enumerate()
            .map(|(i, v)| (i + 1) * v.1 as usize)
            .sum::<usize>()
    );
}

fn char_to_i(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => 0,
    }
}

fn hand_type(hand: String) -> u32 {
    let mut chars: HashMap<char, u32> = HashMap::new();
    for c in hand.chars() {
        if chars.contains_key(&c) {
            *chars.get_mut(&c).unwrap() += 1;
        } else {
            chars.insert(c, 1);
        }
    }
    score(chars)
}

fn score(chars: HashMap<char, u32>) -> u32 {
    let mut has_three = false;
    let mut has_pair = false;
    for (_, v) in chars {
        if v == 5 {
            return 6; // five = 6
        }
        if v == 4 {
            return 5; // four = 5
        }
        if v == 3 {
            has_three = true;
        }
        if v == 2 && has_pair {
            return 2; // two pair = 2
        }
        if v == 2 {
            has_pair = true;
        }
    }
    if has_three && has_pair {
        return 4; // full house = 4
    }
    if has_three {
        return 3; // three = 3
    }
    if has_pair {
        return 1; // pair = 1
    }
    0 // nothing
}
