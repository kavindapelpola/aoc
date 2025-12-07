pub fn load() -> Vec<String> {
    let input = include_str!("../input.txt");
    input.lines().map(|s| s.to_string()).collect()
}
