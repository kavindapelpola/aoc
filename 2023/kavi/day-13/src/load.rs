pub fn load() -> Vec<Vec<String>> {
    let input = include_str!("../input.txt");
    let maps = input.split("\n\n");
    let mut mapvecs = vec![];
    for blob in maps {
        let lines = blob.split("\n");
        mapvecs.push(
            lines
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        );
    }
    mapvecs
}
