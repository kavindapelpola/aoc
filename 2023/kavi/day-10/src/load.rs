pub fn load() -> Vec<String> {
    let input = include_str!("../input.txt");
    input.lines().map(|line| line.to_string()).collect()
}
