pub fn load() -> Vec<Vec<u32>> {
    let input = include_str!("../input.txt");
    let mut v = Vec::new();
    for line in input.lines() {
        let mut v2 = Vec::new();
        for c in line.chars() {
            v2.push(c.to_string().parse::<u32>().unwrap());
        }
        v.push(v2);
    }
    v
}
