use crate::App;

pub fn run() {
    let app = App::new();
    let mut sum_power = 0;
    for games in app.input {
        let mut max_red = 1;
        let mut max_blue = 1;
        let mut max_green = 1;
        for game in games {
            if game.0 > max_red {
                max_red = game.0;
            }
            if game.1 > max_blue {
                max_blue = game.1;
            }
            if game.2 > max_green {
                max_green = game.2;
            }
        }
        sum_power += max_red * max_blue * max_green;
    }
    println!("Part B: {}", sum_power);
}
