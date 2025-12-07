pub fn load() -> Vec<String> {
    let input = include_str!("../input.txt");
    let mut res = vec![];
    for line in input.lines() {
        let parts = line.split(",").collect::<Vec<&str>>();
        for part in parts {
            res.push(part.to_string());
        }
    }
    res
}
