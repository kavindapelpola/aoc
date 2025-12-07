use crate::App;

pub fn run() {
    let app = App::new();
    let mut sum_ratios = 0;
    let gears = app
        .symbols
        .iter()
        .filter(|p| p.0 == '*')
        .collect::<Vec<_>>();
    for gear in gears {
        let mut seen = vec![];
        for part in &app.parts {
            if App::touches_part(&gear.1, part) {
                seen.push(part);
                continue;
            }
        }
        if seen.len() == 2 {
            sum_ratios += seen[0].0 * seen[1].0;
        }
    }
    println!("Part B: {}", sum_ratios);
}
