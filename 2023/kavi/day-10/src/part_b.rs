use crate::App;

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

pub fn run() {
    let app = App::new();
    let mut map = app.input.clone();
    let start_pos = find_s(&app.input);
    let mut path = vec!['S', '|'];
    let mut pos = (start_pos.0 - 1, start_pos.1);
    map[pos.0].replace_range(pos.1..pos.1 + 1, "X");
    let mut dir = Direction::North;
    while *path.last().unwrap() != 'S' {
        pos = mv(pos, &dir);
        let c = app.input[pos.0].chars().nth(pos.1).unwrap();
        path.push(c);
        if c != 'S' {
            dir = turn(c, &dir);
        }
        map[pos.0].replace_range(pos.1..pos.1 + 1, "X");
    }

    let mut area = 0;
    for (i, line) in map.iter().enumerate() {
        let mut inside = false;
        let mut flat_up = false;
        let mut flat_down = false;
        for (j, c) in line.chars().enumerate() {
            let orig = app.input[i].chars().nth(j).unwrap();
            if c == 'X' && orig == '-' {
                continue;
            }
            if c == 'X' && orig == '|' {
                inside = !inside;
                continue;
            };
            if c == 'X' && orig == 'F' {
                flat_up = true;
                continue;
            }
            if c == 'X' && (orig == 'J' || orig == 'S') && flat_up {
                inside = !inside;
                flat_up = false;
                continue;
            }
            if c == 'X' && orig == '7' && flat_up {
                flat_up = false;
                continue;
            }

            if c == 'X' && orig == 'L' {
                flat_down = true;
                continue;
            }
            if c == 'X' && orig == '7' && flat_down {
                inside = !inside;
                flat_down = false;
                continue;
            }
            if c == 'X' && (orig == 'J' || orig == 'S') && flat_down {
                flat_down = false;
                continue;
            }

            if inside {
                area += 1;
            }
        }
    }
    println!("Part B: {}", area);
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
