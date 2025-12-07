pub fn load() -> Vec<String> {
    let input = include_str!("../input.txt");
    let mut res = vec![];
    let mut col = vec![];
    let spacer = "X".repeat(input.lines().next().unwrap().len());
    for line in input.lines() {
        let mut found = false;
        for (p, c) in line.chars().enumerate() {
            if c == '#' {
                found = true;
                col.push(p);
            }
        }
        if !found {
            res.push(line.to_string());
            res.push(spacer.clone());
        } else {
            res.push(line.to_string());
        }
    }
    let mut res2 = vec![];
    for line in res {
        let mut s = String::new();
        for (p, c) in line.chars().enumerate() {
            if col.contains(&p) {
                s.push(c);
            } else {
                if s.contains('#') || s.contains('.') {
                    s.push('.');
                } else {
                    s.push('X');
                }
                s.push('X');
            }
        }
        res2.push(s);
    }
    res2
}

#[cfg(test)]
mod tests {
    use super::*;
}
