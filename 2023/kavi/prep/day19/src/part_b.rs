use crate::{
    model::{Blueprint, Units},
    App, State,
};
use utils::algorithms::optimise::exhaustive::best_explored;

pub fn run() {
    // let mut tot = 0;
    // for i in 1..=3 {
    //     let app = App::new();
    //     let initial_state = State::new(i - 1, 32);
    //     let result = best_explored(&app, &initial_state);
    //     tot += i * result.materials.geode as usize;
    //     println!("{}: {} {}", i, result.materials.geode, tot);
    // }

    let app = App {
        blueprints: vec![Blueprint::new(
            1,
            Units::new(2, 0, 0, 0),
            Units::new(3, 0, 0, 0),
            Units::new(3, 8, 0, 0),
            Units::new(3, 0, 12, 0),
        )],
    };

    let initial_state = State::new(0, 32);
    let result = best_explored(&app, &initial_state);

    println!("Part B : {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Blueprint, Units};

    #[test]
    fn example_from_web_explored() {
        let app = App {
            blueprints: vec![Blueprint::new(
                1,
                Units::new(4, 0, 0, 0),
                Units::new(2, 0, 0, 0),
                Units::new(3, 14, 0, 0),
                Units::new(2, 0, 7, 0),
            )],
        };

        let initial_state = State::new(0, 24);
        let result = best_explored(&app, &initial_state);
        assert_eq!(result.materials.geode, 9);
    }
}
