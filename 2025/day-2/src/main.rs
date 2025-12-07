use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut total = 0;
    input.split(",").for_each(|range| {
        let bounds: Vec<&str> = range.split("-").collect();
        let lower: i64 = bounds[0].trim().parse().expect("Failed to parse value");
        let upper: i64 = bounds[1].trim().parse().expect("Failed to parse value");

        for i in lower..=upper {
            let s = i.to_string();
            let len_s = s.len();
            // must be an even length string
            if len_s % 2 != 0 {
                continue;
            }
            if s[..len_s / 2] == s[len_s / 2..] {
                total += i;
            }
        }
    });
    println!("{}", total);
}

fn part2(input: &str) {
    let mut total: u64 = 0;
    input.split(",").for_each(|range| {
        let bounds: Vec<&str> = range.split("-").collect();
        let lower: i64 = bounds[0].trim().parse().expect("Failed to parse value");
        let upper: i64 = bounds[1].trim().parse().expect("Failed to parse value");

        for i in lower..=upper {
            let s = i.to_string();
            let len_s = s.len();

            let mut found = vec![];
            for j in 1..=(len_s / 2) {
                let repeat_count = len_s / j;
                // cant be a repeated pattern if it doent fit
                if len_s % repeat_count != 0 {
                    continue;
                }
                if s == s[..j].repeat(repeat_count) {
                    found.push(s.clone());
                }
            }
            found.dedup();
            found.iter().for_each(|f| {
                total += f.parse::<u64>().expect("Failed to parse value");
            });
        }
    });
    println!("{}", total);
}
