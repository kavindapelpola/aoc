pub fn load() -> Vec<(char, usize, String)> {
    let input = include_str!("../input.txt");
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let dir = parts.next().unwrap().chars().next().unwrap();
            let len = parts.next().unwrap().parse().unwrap();
            let col = parts.next().unwrap().to_string();
            (dir, len, col)
        })
        .collect()
}
