use crate::App;

pub fn run() {
    let app = App::new();
    let mut total = 0;
    for map in app.input {
        // check if map has a horizontal mirror
        if let Some(pos) = map_horiz_mirror_pos(map.clone()) {
            total += pos * 100;
        }
        // check if map has a vertial mirror
        let rmap = rotate(map);
        if let Some(pos) = map_horiz_mirror_pos(rmap.clone()) {
            total += pos
        }
    }
    println!("Part B: {}", total)
}

// rotate 90 degrees
fn rotate(map: Vec<String>) -> Vec<String> {
    let mut map2 = vec![];
    for i in 0..map[0].len() {
        let mut line = String::new();
        for j in (0..map.len()).rev() {
            line.push(map[j].chars().nth(i).unwrap());
        }
        map2.push(line);
    }
    map2
}

fn map_horiz_mirror_pos(map: Vec<String>) -> Option<usize> {
    // cant start at 0 and cant end at last row
    for p1 in 1..map.len() {
        if is_reflection(&map[p1..], &map[..p1]) {
            return Some(p1);
        }
    }
    None
}

fn is_reflection(bottom_map: &[String], top_map: &[String]) -> bool {
    let mut i = 0;
    let mut diffs = 0;
    while i < bottom_map.len() && i < top_map.len() {
        diffs += diff(
            bottom_map[i].clone(),
            top_map[top_map.len() - i - 1].clone(),
        );
        if diffs > 1 {
            return false;
        }
        i += 1;
    }
    if diffs != 1 {
        return false;
    }
    true
}

fn diff(s1: String, s2: String) -> usize {
    let mut diffs = 0;
    for i in 0..s1.len() {
        if s1.chars().nth(i) != s2.chars().nth(i) {
            diffs += 1;
        }
    }
    diffs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let map = vec![
            String::from("abc"),
            String::from("def"),
            String::from("ghi"),
        ];
        let map2 = vec![
            String::from("adg"),
            String::from("beh"),
            String::from("cfi"),
        ];
        assert_eq!(rotate(map), map2);
    }

    #[test]
    fn test_map_horiz_mirror_pos() {
        let map = vec![
            String::from(".#.##.."),
            String::from(".##.###"),
            String::from("##.##.."),
            String::from("##....."),
            String::from("..#####"),
            String::from("..#.###"),
            String::from("###.#.."),
            String::from(".#.#..."),
            String::from("....#.."),
            String::from("#..#.##"),
            String::from("#....##"),
            String::from("..#.###"),
            String::from("..#.###"),
            String::from("#.#..##"),
            String::from("#..#.##"),
            String::from("....#.."),
            String::from(".#.#..."),
        ];
        let rmap = rotate(map);
        for r in &rmap {
            println!("{}", r);
        }
        assert_eq!(map_horiz_mirror_pos(rmap), Some(0));
    }
}
