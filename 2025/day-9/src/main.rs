use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");

    let coords = input
        .lines()
        .map(|s| {
            let mut parts = s.split(',');
            let x: u64 = parts.next().unwrap().parse().unwrap();
            let y: u64 = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect::<Vec<_>>();

    let mut areas = part1(&coords);
    part2(&coords, &mut areas);
}

fn part1(coords: &Vec<(u64, u64)>) -> Vec<((u64, u64), (u64, u64), u64)> {
    let mut areas = Vec::new();
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            if i == j {
                continue;
            }
            let (x1, y1) = coords[i];
            let (x2, y2) = coords[j];
            let area = (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1);
            areas.push(((x1, y1), (x2, y2), area));
        }
    }
    areas.sort_by(|a, b| a.2.cmp(&b.2));
    println!("{:?}", areas.last());
    areas
}

fn is_left((x1, y1): &(u64, u64), (x2, y2): &(u64, u64), (x, y): &(u64, u64)) -> bool {
    let cross_product = ((*x2 as i64 - *x1 as i64) * (*y as i64 - *y1 as i64))
        - ((*y2 as i64 - *y1 as i64) * (*x as i64 - *x1 as i64));

    if cross_product == 0 {
        // its on the edge
        if x <= x1.max(x2) {
            return true;
        }
        return false;
    }
    if y2 > y1 {
        if cross_product >= 0 {
            return true;
        }
    } else {
        if cross_product < 0 {
            return true;
        }
    }
    false
}

fn inside(coords: &Vec<(u64, u64)>, (x, y): (u64, u64)) -> bool {
    let mut intersect = 0;
    for i in 0..coords.len() {
        let (x1, y1) = coords[i];
        let (x2, y2) = coords[(i + 1) % coords.len()];

        // Check if point is on a horizontal edge
        if y1 == y2 {
            if y == y1 && x >= x1.min(x2) && x <= x1.max(x2) {
                return true; // on the boundary
            }
            continue;
        }

        // Check if point is on a vertical edge
        if x1 == x2 {
            if x == x1 && y >= y1.min(y2) && y <= y1.max(y2) {
                return true; // on the boundary
            }
        }

        // Check if ray at height y intersects this edge
        // Include lower endpoint, exclude upper endpoint to avoid double-counting vertices
        if !((y1 <= y && y < y2) || (y2 <= y && y < y1)) {
            continue;
        }

        // Check if intersection point is to the right of (x, y)
        if is_left(&(x1, y1), &(x2, y2), &(x, y)) {
            intersect += 1;
        }
    }
    intersect % 2 == 1
}

fn part2(coords: &Vec<(u64, u64)>, areas: &mut Vec<((u64, u64), (u64, u64), u64)>) {
    let max_y = coords.iter().map(|(_, y)| *y).max().unwrap();
    let max_x = coords.iter().map(|(x, _)| *x).max().unwrap();
    let mut remembered = vec![vec![false; max_y as usize + 1]; max_x as usize + 1];
    loop {
        if let Some(((x1, y1), (x2, y2), area)) = areas.pop() {
            let mut has_outside = false;
            for x in x1.min(x2)..=x1.max(x2) {
                for y in y1.min(y2)..=y1.max(y2) {
                    if remembered[x as usize][y as usize] {
                        continue;
                    }
                    if inside(coords, (x, y)) {
                        remembered[x as usize][y as usize] = true;
                    } else {
                        has_outside = true;
                        areas.retain(|((x1, y1), (x2, y2), _)| {
                            if x >= *x1.min(x2)
                                && x <= *x1.max(x2)
                                && y >= *y1.min(y2)
                                && y <= *y1.max(y2)
                            {
                                return false;
                            }
                            true
                        });
                        break;
                    }
                }
                if has_outside {
                    break;
                }
            }
            if !has_outside {
                println!(
                    "Considering rectangle: ({},{}) to ({},{}), area: {}, not_inside: {}",
                    x1, y1, x2, y2, area, has_outside
                );
                break;
            }
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_left_returns_true_if_point_is_left() {
        assert!(is_left(&(5, 0), &(5, 5), &(0, 1)));
    }

    #[test]
    fn is_left_returns_true_if_point_is_just_left() {
        assert!(is_left(&(5, 0), &(5, 5), &(4, 1)));
    }

    #[test]
    fn is_left_returns_false_if_point_is_right() {
        assert!(!is_left(&(5, 0), &(5, 5), &(10, 1)));
    }

    #[test]
    fn is_left_returns_false_if_point_is_just_right() {
        assert!(!is_left(&(5, 0), &(5, 5), &(6, 1)));
    }

    #[test]
    fn is_left_returns_true_if_point_is_on_line() {
        assert!(is_left(&(5, 0), &(5, 5), &(5, 1)));
    }

    #[test]
    fn is_left_returns_true_if_point_is_on_start() {
        assert!(is_left(&(5, 0), &(5, 5), &(5, 0)));
    }

    #[test]
    fn is_left_returns_true_if_point_is_on_end() {
        assert!(is_left(&(5, 0), &(5, 5), &(5, 5)));
    }

    #[test]
    fn is_left_returns_true_if_point_is_on_start_of_horizontal_line() {
        assert!(is_left(&(5, 1), &(10, 1), &(5, 1)));
    }

    #[test]
    fn is_left_returns_true_if_point_is_on_end_of_horizontal_line() {
        assert!(is_left(&(5, 1), &(10, 1), &(10, 1)));
    }

    #[test]
    fn is_left_returns_true_if_point_is_left_end_of_horizontal_line() {
        assert!(is_left(&(5, 1), &(10, 1), &(2, 1)));
    }

    #[test]
    fn is_left_returns_false_if_point_is_right_end_of_horizontal_line() {
        assert!(!is_left(&(5, 1), &(10, 1), &(11, 1)));
    }
}
