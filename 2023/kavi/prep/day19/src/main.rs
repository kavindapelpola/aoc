use itertools::iproduct;
use model::{Blueprint, Units};
use std::{cmp::max, hash::Hash};
use utils::traits::problem::{Neighbours, Score};

mod load;
mod model;
mod part_a;
mod part_b;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct State {
    blueprint_index: usize,
    time_left: usize,
    robots: Units,
    materials: Units,
}

impl State {
    fn new(blueprint_index: usize, time_total: usize) -> Self {
        Self {
            blueprint_index,
            time_left: time_total,
            robots: Units::new(1, 0, 0, 0),
            materials: Units::new(0, 0, 0, 0),
        }
    }
}

impl Score<State> for State {
    fn score(&self) -> u32 {
        self.materials.geode as u32
    }
}

struct App {
    blueprints: Vec<Blueprint>,
}

impl App {
    fn new() -> App {
        App {
            blueprints: load::load_blueprint(include_str!("../input.txt")),
        }
    }

    fn can_create(&self, robots: &Units, state: &State) -> bool {
        let blueprint_costs = self.blueprints[state.blueprint_index].costs.clone();
        if state.materials.obsidian < robots.geode * blueprint_costs.geode_robot_cost.obsidian {
            return false;
        }
        if state.materials.clay < robots.obsidian * blueprint_costs.obsidian_robot_cost.clay {
            return false;
        }
        if state.materials.ore
            < robots.geode * blueprint_costs.geode_robot_cost.ore
                + robots.obsidian * blueprint_costs.obsidian_robot_cost.ore
                + robots.clay * blueprint_costs.clay_robot_cost.ore
                + robots.ore * blueprint_costs.ore_robot_cost.ore
        {
            return false;
        }
        true
    }
}

impl Neighbours<State> for App {
    // all states that will result in increasing the flow
    fn neighbours(&self, state: &State) -> Vec<State> {
        let mut good_neighbour_states = vec![];

        // if we have no time we can't do anything
        if state.time_left == 0 {
            return good_neighbour_states;
        }

        let blueprint = &self.blueprints[state.blueprint_index];

        let mut valid_combinations = vec![];
        for (ore, clay, obs, geode) in iproduct!(0..=1, 0..=1, 0..=1, 0..=1) {
            if ore + clay + obs + geode > 1 {
                continue;
            }
            let u = Units::new(ore, clay, obs, geode);
            if self.can_create(&u, state) {
                // dont create too many ore robots
                if ore > 0
                    && state.materials.ore
                        > max(
                            blueprint.costs.clay_robot_cost.ore,
                            max(
                                blueprint.costs.obsidian_robot_cost.ore,
                                max(
                                    blueprint.costs.geode_robot_cost.ore,
                                    blueprint.costs.ore_robot_cost.ore,
                                ),
                            ),
                        )
                {
                    continue;
                }

                // dont create too many clay robots
                if clay > 0 && state.materials.clay > blueprint.costs.obsidian_robot_cost.clay {
                    continue;
                }

                // dont create too many obsidian robots
                if obs > 0 && state.materials.obsidian > blueprint.costs.geode_robot_cost.obsidian {
                    continue;
                }

                valid_combinations.push(u);
            }
        }

        for robots_to_create in valid_combinations {
            let mut new_state = state.clone();
            // add the robots created
            new_state.robots += robots_to_create.clone();
            // remove the materials used to create the robots
            new_state.materials.ore -= robots_to_create.ore * blueprint.costs.ore_robot_cost.ore;
            new_state.materials.ore -= robots_to_create.clay * blueprint.costs.clay_robot_cost.ore;
            new_state.materials.ore -=
                robots_to_create.obsidian * blueprint.costs.obsidian_robot_cost.ore;
            new_state.materials.ore -=
                robots_to_create.geode * blueprint.costs.geode_robot_cost.ore;
            new_state.materials.clay -=
                robots_to_create.obsidian * blueprint.costs.obsidian_robot_cost.clay;
            new_state.materials.obsidian -=
                robots_to_create.geode * blueprint.costs.geode_robot_cost.obsidian;

            // add the materials created by robots
            new_state.materials += state.robots.clone();

            // decrement time left
            new_state.time_left -= 1;

            good_neighbour_states.push(new_state);
        }

        good_neighbour_states
    }
}

fn main() {
    part_a::run();
    part_b::run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_returns_true_if_can_create() {
        let app = App {
            blueprints: vec![Blueprint::new(
                1,
                Units::new(3, 0, 0, 0),
                Units::new(4, 0, 0, 0),
                Units::new(3, 19, 0, 0),
                Units::new(2, 0, 8, 0),
            )],
        };
        let mut state = State::new(0, 100);
        state.materials = Units::new(3, 0, 0, 0);
        let result = app.can_create(&Units::new(1, 0, 0, 0), &state);
        assert!(result);
    }

    #[test]
    fn can_create_returns_false_if_cant_create() {
        let app = App {
            blueprints: vec![Blueprint::new(
                1,
                Units::new(3, 0, 0, 0),
                Units::new(4, 0, 0, 0),
                Units::new(3, 19, 0, 0),
                Units::new(2, 0, 8, 0),
            )],
        };
        let mut state = State::new(0, 100);
        state.materials = Units::new(3, 0, 0, 0);
        let result = app.can_create(&Units::new(1, 0, 0, 1), &state);
        assert!(!result);
    }

    #[test]
    fn neighbours_returns_all_neighbours() {
        let app = App {
            blueprints: vec![Blueprint::new(
                1,
                Units::new(4, 0, 0, 0),
                Units::new(2, 0, 0, 0),
                Units::new(3, 14, 0, 0),
                Units::new(2, 0, 7, 0),
            )],
        };
        let result = app.neighbours(&State::new(0, 24));
        let expected = vec![State {
            blueprint_index: 0,
            time_left: 23,
            robots: Units::new(1, 0, 0, 0),
            materials: Units::new(1, 0, 0, 0),
        }];
        assert_eq!(result, expected);
    }
}
