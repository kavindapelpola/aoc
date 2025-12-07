pub fn load() -> Vec<(Vec<String>, Vec<String>)> {
    let input = include_str!("../input.txt");
    let mut res = vec![];
    for line in input.lines() {
        let win: Vec<String> = line[10..40]
            .trim()
            .split(" ")
            .filter(|x| !x.is_empty() && x != &" ")
            .map(|x| x.to_string())
            .collect();
        let my: Vec<String> = line[42..]
            .trim()
            .split(" ")
            .filter(|x| !x.is_empty() && x != &" ")
            .map(|x| x.to_string())
            .collect();
        res.push((win, my));
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
}
