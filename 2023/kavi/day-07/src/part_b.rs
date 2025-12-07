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
        "Part B: {}",
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
        'J' => 1,
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
    let mut has_two_pair = false;
    let wilds = chars.get(&'J').unwrap_or(&0).clone();
    if wilds == 5 {
        return 6; // five of a kind
    }
    for (c, v) in chars {
        if c == 'J' {
            continue;
        }
        if v + wilds == 5 {
            return 6; // five = 6
        }
        if v + wilds == 4 {
            return 5; // four = 5
        }
        if v == 3 {
            has_three = true;
        }
        if v == 2 && has_pair {
            has_two_pair = true;
        }
        if v == 2 && !has_pair {
            has_pair = true;
        }
    }
    if has_three && has_pair {
        return 4; // full house = 4
    }
    if has_two_pair && wilds == 1 {
        return 4; // full house = 4
    }
    if has_three {
        return 3; // three = 3
    }
    if has_pair && wilds == 1 {
        return 3; // three = 3
    }
    if wilds == 2 {
        return 3; // three = 3
    }
    if has_two_pair {
        return 2; // two pair = 2
    }
    if has_pair {
        return 1; // pair = 1
    }
    if wilds > 0 {
        return 1; // pair = 1
    }
    0 // nothing
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_type() {
        assert_eq!(hand_type("22222".to_string()), 6); // five of a kind
        assert_eq!(hand_type("2222J".to_string()), 6); // five of a kind
        assert_eq!(hand_type("222JJ".to_string()), 6); // five of a kind
        assert_eq!(hand_type("22JJJ".to_string()), 6); // five of a kind
        assert_eq!(hand_type("2JJJJ".to_string()), 6); // five of a kind
        assert_eq!(hand_type("JJJJJ".to_string()), 6); // five of a kind

        assert_eq!(hand_type("22223".to_string()), 5); // four of a kind
        assert_eq!(hand_type("222J3".to_string()), 5); // four of a kind
        assert_eq!(hand_type("22JJ3".to_string()), 5); // four of a kind
        assert_eq!(hand_type("2JJJ3".to_string()), 5); // four of a kind

        assert_eq!(hand_type("22233".to_string()), 4); // full house
        assert_eq!(hand_type("22J33".to_string()), 4); // full house

        assert_eq!(hand_type("22234".to_string()), 3); // three of a kind
        assert_eq!(hand_type("22J34".to_string()), 3); // three of a kind

        assert_eq!(hand_type("22335".to_string()), 2); // two pair

        assert_eq!(hand_type("22345".to_string()), 1); // pair
        assert_eq!(hand_type("J2345".to_string()), 1); // pair

        assert_eq!(hand_type("23456".to_string()), 0); // nothing
    }
}
