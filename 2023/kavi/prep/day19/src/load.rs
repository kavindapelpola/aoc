use crate::{Blueprint, Units};

pub fn load_blueprint(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(|line| {
            let token = line.split(' ').collect::<Vec<&str>>();
            let mut id = token[1].to_string();
            id.pop();
            let ore_robot_cost = Units::new(token[6].parse().unwrap(), 0, 0, 0);
            let clay_robot_cost = Units::new(token[12].parse().unwrap(), 0, 0, 0);
            let obsidian_robot_cost =
                Units::new(token[18].parse().unwrap(), token[21].parse().unwrap(), 0, 0);
            let geode_robot_cost =
                Units::new(token[27].parse().unwrap(), 0, token[30].parse().unwrap(), 0);
            Blueprint::new(
                id.parse().unwrap(),
                ore_robot_cost,
                clay_robot_cost,
                obsidian_robot_cost,
                geode_robot_cost,
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_blueprint() {
        let result = load_blueprint("Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 19 clay. Each geode robot costs 3 ore and 8 obsidian.
Blueprint 2: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 9 clay. Each geode robot costs 2 ore and 10 obsidian.\n");
        let expected = vec![
            Blueprint::new(
                1,
                Units::new(3, 0, 0, 0),
                Units::new(4, 0, 0, 0),
                Units::new(3, 19, 0, 0),
                Units::new(3, 0, 8, 0),
            ),
            Blueprint::new(
                2,
                Units::new(3, 0, 0, 0),
                Units::new(3, 0, 0, 0),
                Units::new(3, 9, 0, 0),
                Units::new(2, 0, 10, 0),
            ),
        ];
        assert_eq!(result, expected);
    }
}
