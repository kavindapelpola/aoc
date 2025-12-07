pub fn load() -> Vec<(String, u32)> {
    let input = include_str!("../input.txt");
    input
        .lines()
        .map(|l| {
            let p = l.split_whitespace().collect::<Vec<&str>>();
            (p[0].to_string(), p[1].to_string().parse::<u32>().unwrap())
        })
        .collect::<Vec<(String, u32)>>()
}
