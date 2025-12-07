use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut dial = 50;
    let mut count = 0;
    input.lines().for_each(|line| {
        let (direction, value_str) = line.split_at(1);
        let value: i32 = value_str.trim().parse().expect("Failed to parse value");

        match direction {
            "L" => dial += value,
            "R" => dial -= value,
            _ => panic!("Invalid direction"),
        }
        dial = dial % 100;
        if dial == 0 {
            count += 1;
        }
    });
    println!("part 1: {}", count);
}

fn part2(input: &str) {
    let mut dial = 50;
    let mut count = 0;
    input.lines().for_each(|line| {
        let (direction, value_str) = line.split_at(1);
        let value: i32 = value_str.trim().parse().expect("Failed to parse value");

        let old_dial = dial;
        match direction {
            "L" => dial -= value,
            "R" => dial += value,
            _ => panic!("Invalid direction"),
        }
        if old_dial * dial < 0 {
            count += 1;
        }
        if dial == 0 {
            count += 1;
        } else {
            count += dial.abs() / 100;
        }
        dial = dial % 100;
    });
    println!("part 2: {}", count);
}
