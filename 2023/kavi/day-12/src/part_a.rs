use crate::App;
use std::collections::HashMap;

pub fn run() {
    let app = App::new();
    let mut sum = 0;
    for i in app.input {
        let mut cache = HashMap::new();
        sum += arrangements(i.0.clone(), i.1.clone(), None, &mut cache);
    }
    println!("Part A: {}", sum);
}

pub fn arrangements(
    s: String,
    v: Vec<usize>,
    in_hash: Option<usize>,
    cache: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    if s.is_empty() {
        return match (in_hash, v.len()) {
            (None, 0) => 1,
            (Some(x), 1) if x == v[0] => 1,
            _ => 0,
        };
    }
    if in_hash.is_some() && v.is_empty() {
        return 0;
    }

    let key = (s.len(), in_hash.unwrap_or(0), v.len());
    if let Some(&x) = cache.get(&key) {
        return x;
    }

    let a = match (s.chars().nth(0).unwrap(), in_hash) {
        ('#', None) => arrangements(s[1..].to_string(), v, Some(1), cache),
        ('#', Some(x)) => arrangements(s[1..].to_string(), v, Some(x + 1), cache),
        ('.', None) => arrangements(s[1..].to_string(), v, None, cache),
        ('.', Some(x)) => {
            if x != v[0] {
                return 0;
            }
            arrangements(s[1..].to_string(), v[1..].to_vec(), None, cache)
        }
        ('?', None) => {
            // dot + hash
            arrangements(s[1..].to_string(), v.clone(), None, cache)
                + arrangements(s[1..].to_string(), v.clone(), Some(1), cache)
        }
        ('?', Some(x)) => {
            let mut a = arrangements(s[1..].to_string(), v.clone(), Some(x + 1), cache);
            if x == v[0] {
                a += arrangements(s[1..].to_string(), v[1..].to_vec(), None, cache)
            }
            a
        }
        _ => panic!(),
    };
    cache.insert(key, a);
    a
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test() {
        let s = (0..5).map(|_| "?????#?#.???????????").join("?");
        let v = (0..5)
            .flat_map(|_| vec![1, 6, 1, 1, 1, 4])
            .collect::<Vec<_>>();
        let mut cache = HashMap::new();
        println!("{:?}", arrangements(s, v, None, &mut cache));
    }
}
