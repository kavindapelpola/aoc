pub fn load() -> (Vec<(char, (usize, usize))>, Vec<(u32, Vec<(usize, usize)>)>) {
    let input = include_str!("../input.txt");
    let mut symbols = vec![];
    let mut serials = vec![];
    for (l, line) in input.lines().enumerate() {
        let mut p = 0;
        while p < line.len() {
            let c = line.chars().nth(p).unwrap();
            if c == '.' {
                p += 1;
                continue;
            }
            if !c.is_digit(10) {
                symbols.push((c, (l, p)));
                p += 1;
                continue;
            }
            let mut num = 0;
            let mut ps = vec![];
            while p < line.len() && line.chars().nth(p).unwrap().is_digit(10) {
                ps.push((l, p));
                num = num * 10 + line.chars().nth(p).unwrap().to_digit(10).unwrap();
                p += 1;
            }
            serials.push((num, ps));
        }
    }
    (symbols, serials)
}
