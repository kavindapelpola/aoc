use crate::App;

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

pub fn run() {
    let app = App::new();
    let start_pos = find_s(&app.input);
    let mut path = vec!['S', '|'];
    let mut pos = (start_pos.0 - 1, start_pos.1);
    let mut dir = Direction::North;
    while *path.last().unwrap() != 'S' {
        pos = mv(pos, &dir);
        let c = app.input[pos.0].chars().nth(pos.1).unwrap();
        path.push(c);
        if c != 'S' {
            dir = turn(c, &dir);
        }
    }
    println!("Part A: {:?}", path.len() / 2);
}

fn mv(pos: (usize, usize), dir: &Direction) -> (usize, usize) {
    match dir {
        Direction::North => (pos.0 - 1, pos.1),
        Direction::South => (pos.0 + 1, pos.1),
        Direction::West => (pos.0, pos.1 - 1),
        Direction::East => (pos.0, pos.1 + 1),
    }
}

fn turn(c: char, dir: &Direction) -> Direction {
    match c {
        '|' => match dir {
            Direction::North => Direction::North,
            Direction::South => Direction::South,
            _ => panic!("Invalid direction {:?} for {}", dir, c),
        },
        '-' => match dir {
            Direction::West => Direction::West,
            Direction::East => Direction::East,
            _ => panic!("Invalid direction {:?} for {}", dir, c),
        },
        'L' => match dir {
            // north and east
            Direction::West => Direction::North,
            Direction::South => Direction::East,
            _ => panic!("Invalid direction {:?} for {}", dir, c),
        },
        'J' => match dir {
            // north and west
            Direction::East => Direction::North,
            Direction::South => Direction::West,
            _ => panic!("Invalid direction {:?} for {}", dir, c),
        },
        '7' => match dir {
            // south and west
            Direction::East => Direction::South,
            Direction::North => Direction::West,
            _ => panic!("Invalid direction {:?} for {}", dir, c),
        },
        'F' => match dir {
            // south and east
            Direction::West => Direction::South,
            Direction::North => Direction::East,
            _ => panic!("Invalid direction {:?} for {}", dir, c),
        },
        _ => panic!("Invalid character {}", c),
    }
}

fn find_s(map: &Vec<String>) -> (usize, usize) {
    for (r, row) in map.iter().enumerate() {
        for (c, col) in row.chars().enumerate() {
            if col == 'S' {
                return (r, c);
            }
        }
    }
    (0, 0)
}
