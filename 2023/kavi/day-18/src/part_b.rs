use crate::App;

pub fn run() {
    let app = App::new();
    let mut new_input = vec![];
    for (_, _, col) in app.input {
        let size = usize::from_str_radix(col[2..7].to_string().as_str(), 16).unwrap();
        match col[7..8].to_string().as_str() {
            "0" => new_input.push(('R', size)),
            "1" => new_input.push(('D', size)),
            "2" => new_input.push(('L', size)),
            "3" => new_input.push(('U', size)),
            _ => panic!(),
        }
    }

    let outside = new_input.iter().map(|(_, len)| len).sum::<usize>();

    // shoelace formula
    let points = dir_to_points(new_input);
    let mut sum = 0;
    for i in 0..points.len() - 1 {
        let (x0, y0) = points[i];
        let (x1, y1) = points[i + 1];
        sum += x0 * y1 - y0 * x1;
    }

    println!("Part B: {}", ((sum + outside) / 2) + 1);
}

fn dir_to_points(input: Vec<(char, usize)>) -> Vec<(usize, usize)> {
    let mut points = vec![];
    let mut x = 0;
    let mut y = 0;
    for (dir, len) in input {
        match dir {
            'R' => x += len,
            'L' => x -= len,
            'U' => y -= len,
            'D' => y += len,
            _ => panic!("Unknown direction"),
        }
        points.push((x, y));
    }
    points
}
