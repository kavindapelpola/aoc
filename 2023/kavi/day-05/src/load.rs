pub fn load() -> (Vec<u64>, Vec<Vec<(u64, u64, u64)>>) {
    let input = include_str!("../input.txt");
    let mut line_iter = input.lines();
    let mut seeds: Vec<&str> = line_iter.next().unwrap().split_whitespace().collect();
    seeds.remove(0);
    let seeds_uint = seeds
        .iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut maps = vec![];

    line_iter.next();
    for _ in 0..7 {
        line_iter.next();
        let mut map = vec![];
        while let Some(line) = line_iter.next() {
            if line.is_empty() {
                break;
            }
            let parts = line
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            map.push((parts[0], parts[1], parts[2]));
        }
        maps.push(map);
    }

    (seeds_uint, maps)
}

#[cfg(test)]
mod tests {
    use super::*;
}
