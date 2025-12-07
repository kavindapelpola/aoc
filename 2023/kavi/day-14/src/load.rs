pub fn load() -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let input = include_str!("../input.txt");
    let mut rocks = vec![];
    let mut hashes = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'O' => rocks.push((y, x)),
                '#' => hashes.push((y, x)),
                _ => (),
            }
        }
    }
    // rocks already sorted by top first
    (rocks, hashes)
}
