use crate::Coord;

pub fn load() -> Vec<(Coord, Coord)> {
    let input = include_str!("../i.txt");
    let mut coords = vec![];
    for line in input.lines() {
        let s = line.split("~").collect::<Vec<&str>>();
        coords.push((get_coord(&s[0]), get_coord(&s[1])));
    }
    coords
}

fn get_coord(s: &str) -> Coord {
    let s = s.split(",").collect::<Vec<&str>>();
    let x = s[0].parse::<usize>().unwrap();
    let y = s[1].parse::<usize>().unwrap();
    let z = s[2].parse::<usize>().unwrap();
    (x, y, z)
}
