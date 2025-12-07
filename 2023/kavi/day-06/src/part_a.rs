pub fn run() {
    let mut mult = 1;
    let races: Vec<(i32, i32)> = vec![(41, 249), (77, 1362), (70, 1127), (96, 1011)];
    for (time, dist) in races {
        let discriminant = ((time.pow(2) - 4 * dist) as f64).sqrt();
        let lower = ((time as f64 - discriminant) / 2.0).ceil() as i32;
        let upper = ((time as f64 + discriminant) / 2.0).floor() as i32;
        let count = upper - lower + 1;
        mult *= count;
    }
    println!("Part A: {}", mult);
}
