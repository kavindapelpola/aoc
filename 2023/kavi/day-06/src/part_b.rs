pub fn run() {
    let mut mult = 1;
    let races: Vec<(i64, i64)> = vec![(41777096, 249136211271011)];
    for (time, dist) in races {
        let discriminant = ((time.pow(2) - 4 * dist) as f64).sqrt();
        let lower = ((time as f64 - discriminant) / 2.0).ceil() as i32;
        let upper = ((time as f64 + discriminant) / 2.0).floor() as i32;
        let count = upper - lower + 1;
        mult *= count;
    }
    println!("Part B: {}", mult);
}
