pub fn load() -> Vec<Vec<i64>> {
    let input = include_str!("../input.txt");
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}
