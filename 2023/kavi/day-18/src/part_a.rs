use crate::App;

const SIZE: usize = 1000;

pub fn run() {
    let app = App::new();
    let mut map = map(&app.input, SIZE);
    map = fill(&map, SIZE);

    let mut total = 0;
    for y in 0..SIZE {
        for x in 0..SIZE {
            if map[y][x] == 'X' || map[y][x] == 'Y' {
                total += 1;
            }
        }
    }
    println!("Part A: {}", total);
}

pub fn map(input: &Vec<(char, usize, String)>, size: usize) -> Vec<Vec<char>> {
    let mut map = vec![vec!['.'; size]; size];
    let mut x = size / 2;
    let mut y = size / 2;
    map[y][x] = 'X';
    for (dir, len, _) in input {
        // println!("{} {} - {} {}", x, y, dir, len);
        for _ in 0..*len {
            match dir {
                'R' => x += 1,
                'L' => x -= 1,
                'U' => y -= 1,
                'D' => y += 1,
                _ => panic!("Unknown direction"),
            }
            map[y][x] = 'X';
        }
    }
    map
}

pub fn fill(map: &Vec<Vec<char>>, size: usize) -> Vec<Vec<char>> {
    let mut map = map.clone();
    for y in 0..size {
        let mut on = false;
        let mut start_up = false;
        for x in 0..size {
            if map[y][x] == 'X' {
                // XXXXXXX
                if map[y][x + 1] == 'X' && map[y][x - 1] == 'X' {
                    continue;
                }
                // ...X...
                if map[y][x + 1] != 'X' && map[y][x - 1] != 'X' {
                    on = !on;
                    continue;
                }
                // .X.....
                // .XXXXXX
                // .......
                if map[y][x - 1] != 'X' && map[y - 1][x] == 'X' {
                    start_up = true;
                    continue;
                }
                // .......
                // .XXXXXX
                // .X.....
                if map[y][x - 1] != 'X' && map[y + 1][x] == 'X' {
                    start_up = false;
                    continue;
                }
                // .....X.
                // .XXXXX.
                // .X.....
                if map[y][x + 1] != 'X' && map[y - 1][x] == 'X' && !start_up {
                    on = !on;
                    continue;
                }
                // .X.....
                // .XXXXX.
                // .....X.
                if map[y][x + 1] != 'X' && map[y + 1][x] == 'X' && start_up {
                    on = !on;
                    continue;
                }
            }
            if on {
                map[y][x] = 'Y';
            }
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fill() {
        let map = vec![
            vec!['.', '.', '.', '.', '.', '.'],
            vec!['.', 'X', 'X', 'X', '.', '.'],
            vec!['.', 'X', '.', 'X', 'X', '.'],
            vec!['.', 'X', '.', '.', 'X', '.'],
            vec!['.', 'X', 'X', 'X', 'X', '.'],
            vec!['.', '.', '.', '.', '.', '.'],
        ];
        let map = fill(&map, 6);
        for line in map {
            println!("{:?}", line);
        }
    }
}
