pub fn load() -> Vec<(usize, usize, char)> {
    let input = include_str!("../input.txt");
    let mut res = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            res.push((y, x, c));
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
}
