pub fn load() -> Vec<(String, Vec<usize>)> {
    let input = include_str!("../input.txt");
    let mut res = vec![];
    for line in input.lines() {
        let parts = line.split(" ").collect::<Vec<&str>>();
        res.push((
            parts[0].to_string(),
            parts[1].split(",").map(|x| x.parse().unwrap()).collect(),
        ));
    }
    res
}
