use crate::App;

pub fn run() {
    let app = App::new();
    let mut sum_id = 0;
    for (id, games) in app.input.iter().enumerate() {
        let mut impossible = false;
        for game in games {
            if game.0 > 12 {
                // red
                impossible = true;
                break;
            }
            if game.1 > 14 {
                // blue
                impossible = true;
                break;
            }
            if game.2 > 13 {
                // green
                impossible = true;
                break;
            }
        }
        if !impossible {
            sum_id += id + 1;
        }
    }
    println!("Part A: {}", sum_id);
}
