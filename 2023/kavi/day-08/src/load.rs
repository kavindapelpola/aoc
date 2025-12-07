use std::collections::HashMap;

pub fn load() -> HashMap<String, (String, String)> {
    let input = include_str!("../input.txt");
    let mut map = HashMap::new();
    for line in input.lines() {
        map.insert(
            line[0..=2].to_string(),
            (line[7..=9].to_string(), line[12..=14].to_string()),
        );
    }
    map
}
