use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");

    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    part1(&grid);
    part2(&grid);
}

fn adjacent(grid: &Vec<Vec<char>>, x: i32, y: i32) -> u32 {
    let width = grid[0].len();
    let height = grid.len();
    let check = vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ];
    let mut count = 0;
    for (x, y) in check {
        if (x >= 0)
            && (x < width as i32)
            && (y >= 0)
            && (y < height as i32)
            && (grid[y as usize][x as usize] == '@')
        {
            count += 1;
        }
    }
    return count;
}

fn part1(grid: &Vec<Vec<char>>) {
    let mut count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '@' && adjacent(&grid, x as i32, y as i32) < 4 {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

fn part2(grid: &Vec<Vec<char>>) {
    let mut grid_mut = grid.clone();
    let mut count = 0;
    loop {
        let mut remove = vec![];
        for y in 0..grid_mut.len() {
            for x in 0..grid_mut[0].len() {
                if grid_mut[y][x] == '@' && adjacent(&grid_mut, x as i32, y as i32) < 4 {
                    count += 1;
                    remove.push((x, y));
                }
            }
        }
        if remove.is_empty() {
            break;
        }
        for (x, y) in remove {
            grid_mut[y][x] = '.';
        }
    }
    println!("{}", count);
}
