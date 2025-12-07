use std::fs;

type Range = (u64, u64);

fn point_inside_range(point: u64, range: &Range) -> bool {
    if point >= range.0 && point <= range.1 {
        return true;
    }
    return false;
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");

    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let ranges = parts[0]
        .lines()
        .map(|line| {
            let r = line.split("-").collect::<Vec<&str>>();
            return ((r[0].parse::<u64>().unwrap(), r[1].parse::<u64>().unwrap()));
        })
        .collect::<Vec<Range>>();
    let ingredients = parts[1]
        .lines()
        .map(|ingredient| ingredient.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    part1(&ranges, &ingredients);
    part2(&ranges);
}

fn part1(ranges: &Vec<Range>, ingredients: &Vec<u64>) {
    let mut count = 0;
    ingredients.iter().for_each(|i| {
        for (lower, upper) in ranges {
            if i >= lower && i <= upper {
                count += 1;
                return;
            }
        }
    });
    println!("{}", count);
}

fn merge(range1: &Range, range2: &Range) -> Option<Range> {
    // range1 consumes range2
    if range1.0 <= range2.0 && range1.1 >= range2.1 {
        return Some(range1.clone());
    }
    // range2 consumes range1
    if range2.0 <= range1.0 && range2.1 >= range1.1 {
        return Some(range2.clone());
    }
    // range1 lower is inside range2
    if point_inside_range(range1.0, range2) {
        // if range1 upper is beyond range2 upper then merge
        if range1.1 > range2.1 {
            return Some((range2.0, range1.1));
        } else {
            // range2 consumes range1
            panic!("this shouldnt happen");
        }
    }
    // range1 upper is inside range2
    if point_inside_range(range1.1, range2) {
        // if range1 lower is before range2 lower then merge
        if range1.0 < range2.0 {
            return Some((range1.0, range2.1));
        } else {
            // range2 consumes range1
            panic!("this shouldnt happen");
        }
    }
    return None;
}

fn part2(ranges: &Vec<Range>) {
    let mut unmerged_ranges = ranges.to_vec();
    let mut merged_ranges = vec![];
    loop {
        if let Some(mut range_to_merge) = unmerged_ranges.pop() {
            if unmerged_ranges.is_empty() {
                merged_ranges.push(range_to_merge);
                break;
            }
            let mut pos = 0;
            loop {
                if let Some(result) = merge(&range_to_merge, &unmerged_ranges[pos]) {
                    // merged so remove the entry and try unmerged list again
                    range_to_merge = result;
                    unmerged_ranges.remove(pos);
                    if unmerged_ranges.len() == 0 {
                        break;
                    }
                    pos = 0;
                    continue;
                }
                pos += 1;
                if pos >= unmerged_ranges.len() {
                    break;
                }
            }
            merged_ranges.push(range_to_merge);
        } else {
            break;
        }
    }
    println!(
        "{}",
        merged_ranges.iter().map(|(l, u)| u - l + 1).sum::<u64>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_merge() {
        let result = merge(&(12, 18), &(16, 20));
        assert_eq!(result, Some((12, 20)));
    }
}
