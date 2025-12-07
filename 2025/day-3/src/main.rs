use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut total = 0;
    input.lines().for_each(|line| {
        // find the highest number before the last char
        let highest = line[..line.len() - 1].chars().into_iter().max().unwrap();
        // find the position of the fist occurrence highest number before the last char
        let first_highest_pos = line.find(highest).unwrap();
        let second_highest = line[first_highest_pos + 1..]
            .chars()
            .into_iter()
            .max()
            .unwrap();
        total += format!("{}{}", highest, second_highest)
            .parse::<u32>()
            .unwrap();
    });
    println!("{}", total);
}

fn part2(input: &str) {
    let mut total: u64 = 0;
    input.lines().for_each(|line| {
        let mut start = 0;
        let mut num = "".to_string();
        for i in 0..12 {
            let segment = line[start..line.len() - (11 - i)].to_string();
            let highest = segment.chars().into_iter().max().unwrap();
            start += segment.find(highest).unwrap() + 1;
            num.push(highest);
        }
        total += num.parse::<u64>().unwrap();
    });
    println!("{}", total);
}
