use crate::App;

pub fn run() {
    let app = App::new();
    let mut sum = 0;
    for i in app.input {
        sum += next_value(&i);
    }
    println!("Part A: {}", sum);
}

fn next_value(seq: &Vec<i64>) -> i64 {
    let mut rows = vec![seq.clone()];
    loop {
        let mut next_row = vec![];
        let mut all_zero = true;
        let r = rows.last().unwrap();
        for i in 0..r.len() - 1 {
            let val = r[i + 1] - r[i];
            if val != 0 {
                all_zero = false;
            }
            next_row.push(val);
        }
        rows.push(next_row);
        if all_zero {
            break;
        }
    }
    let mut last_val = 0;
    for i in (0..rows.len()).rev() {
        let l = *rows[i].last().unwrap();
        last_val += l;
    }
    last_val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_value() {
        assert_eq!(next_value(&vec![1, 3, 6, 10, 15, 21]), 28);
    }
}
