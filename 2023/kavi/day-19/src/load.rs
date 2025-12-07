use std::collections::HashMap;

pub fn load() -> (Vec<(u32, u32, u32, u32)>, HashMap<String, Vec<String>>) {
    let input = include_str!("../i.txt");

    let mut workflows = HashMap::new();
    let mut parts = vec![];

    let sections = input.split("\n\n").collect::<Vec<&str>>();
    for line in sections[0].lines() {
        let mut parts_line = line.split("{");
        workflows.insert(
            parts_line.next().unwrap().to_string(),
            parts_line
                .next()
                .unwrap()
                .trim_end_matches("}")
                .trim_start_matches("{")
                .split(",")
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        );
    }
    for line in sections[1].lines() {
        let mut parts_line = line.split(",");
        parts.push((
            parts_line.next().unwrap().split("=").collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .unwrap(),
            parts_line.next().unwrap().split("=").collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .unwrap(),
            parts_line.next().unwrap().split("=").collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .unwrap(),
            parts_line
                .next()
                .unwrap()
                .trim_end_matches("}")
                .split("=")
                .collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .unwrap(),
        ));
    }

    (parts, workflows)
}
