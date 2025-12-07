use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut cols = vec![];
    lines[0].chars().enumerate().for_each(|(p, c)| {
        if c == ' '
            && lines[1].chars().nth(p) == Some(' ')
            && lines[2].chars().nth(p) == Some(' ')
            && lines[3].chars().nth(p) == Some(' ')
        {
            cols.push(p);
        }
    });
    cols.push(lines[0].len());
    let mut total = 0;
    let mut last_col = 0;
    for col in cols {
        let s1 = lines[0][last_col..col].to_string();
        let s2 = lines[1][last_col..col].to_string();
        let s3 = lines[2][last_col..col].to_string();
        let s4 = lines[3][last_col..col].to_string();
        let op = lines[4][last_col..col].to_string();
        let n1 = s1.trim().parse::<u64>().unwrap();
        let n2 = s2.trim().parse::<u64>().unwrap();
        let n3 = s3.trim().parse::<u64>().unwrap();
        let n4 = s4.trim().parse::<u64>().unwrap();

        if op.trim() == "*" {
            total += n1 * n2 * n3 * n4;
        } else {
            total += n1 + n2 + n3 + n4;
        }
        last_col = col;
    }
    println!("{}", total);
}

fn part2(input: &str) {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut cols = vec![];
    lines[0].chars().enumerate().for_each(|(p, c)| {
        if c == ' '
            && lines[1].chars().nth(p) == Some(' ')
            && lines[2].chars().nth(p) == Some(' ')
            && lines[3].chars().nth(p) == Some(' ')
        {
            cols.push(p);
        }
    });
    cols.push(lines[0].len());
    let mut total = 0;
    let mut last_col = 0;
    for col in cols {
        let op = lines[4][last_col..col].to_string();

        let mut subtotal = 0;
        for i in (last_col..col).rev() {
            let s = format!(
                "{}{}{}{}",
                lines[0].chars().nth(i).unwrap(),
                lines[1].chars().nth(i).unwrap(),
                lines[2].chars().nth(i).unwrap(),
                lines[3].chars().nth(i).unwrap()
            );
            if s.trim().is_empty() {
                continue;
            }
            let n = s.trim().parse::<u64>().unwrap();
            if i == col - 1 {
                subtotal = n;
            } else {
                if op.trim() == "*" {
                    subtotal *= n;
                } else {
                    subtotal += n;
                }
            }
        }
        total += subtotal;
        last_col = col;
    }
    println!("{}", total);
}
