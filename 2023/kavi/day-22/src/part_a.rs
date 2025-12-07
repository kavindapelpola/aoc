use crate::{App, Coord};
use std::collections::HashSet;

type Brick = HashSet<Coord>;

pub fn run() {
    let app = App::new();

    let bricks = get_bricks(&app.input);

    let (_, supported_matrix) = settle_bricks(&bricks);

    let total = (0..bricks.len())
        .collect::<Vec<_>>()
        .iter()
        .filter(|&i| {
            !supported_matrix
                .iter()
                .any(|(_, v)| v.contains(i) && v.len() == 1)
        })
        .count();
    println!("Part A: {}", total);
}

pub fn get_bricks(input: &Vec<(Coord, Coord)>) -> Vec<Brick> {
    let mut bricks = vec![];
    for line in input {
        let mut brick = HashSet::new();
        for x in line.0 .0..=line.1 .0 {
            for y in line.0 .1..=line.1 .1 {
                for z in line.0 .2..=line.1 .2 {
                    brick.insert((x, y, z));
                }
            }
        }
        bricks.push(brick);
    }
    bricks
}

fn height(brick: &Brick) -> usize {
    *brick.iter().map(|(_, _, z)| z).min().unwrap()
}

pub fn settle_bricks(bricks: &Vec<Brick>) -> (Vec<Brick>, Vec<(usize, Vec<usize>)>) {
    let mut settled_bricks = vec![];
    let mut supported_matrix = vec![];
    let mut falling_bricks = bricks.clone();

    while !falling_bricks.is_empty() {
        move_down(
            &mut falling_bricks,
            &mut settled_bricks,
            &mut supported_matrix,
        );
    }
    (settled_bricks, supported_matrix)
}

fn move_down(
    falling_bricks: &mut Vec<Brick>,
    settled_bricks: &mut Vec<Brick>,
    supported_matrix: &mut Vec<(usize, Vec<usize>)>,
) {
    // try move each brick as far down as possible
    let mut brick_no = 0;
    while brick_no < falling_bricks.len() {
        // try move brick down as much as possible
        loop {
            let height = height(&falling_bricks[brick_no]);
            if height == 1 {
                let brick = falling_bricks.remove(brick_no);
                settled_bricks.push(brick);
                break;
            }
            let mut new_brick = HashSet::new();
            for coord in &falling_bricks[brick_no] {
                new_brick.insert((coord.0, coord.1, coord.2 - 1));
            }
            // add the brick if new_brick intersected a settled brick and stop
            let intersected_settled = settled_bricks
                .iter()
                .enumerate()
                .filter(|(_, b)| b.intersection(&new_brick).count() > 0)
                .map(|(i, _)| i)
                .collect::<Vec<_>>();
            if intersected_settled.len() > 0 {
                let brick = falling_bricks.remove(brick_no);
                settled_bricks.push(brick);
                supported_matrix.push((settled_bricks.len() - 1, intersected_settled));
                break;
            }
            // if it intersected a falling brick, then move on to the next brick
            if falling_bricks
                .iter()
                .enumerate()
                .any(|(i, b)| i != brick_no && b.intersection(&new_brick).count() > 0)
            {
                brick_no += 1;
                break;
            }
            falling_bricks[brick_no] = new_brick;
        }
    }
}
