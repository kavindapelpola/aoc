use std::{cmp::max, ops::AddAssign};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Units {
    pub ore: u8,
    pub clay: u8,
    pub obsidian: u8,
    pub geode: u8,
}

impl Units {
    pub fn new(ore: u8, clay: u8, obsidian: u8, geode: u8) -> Self {
        Self {
            ore,
            clay,
            obsidian,
            geode,
        }
    }
}

impl AddAssign for Units {
    fn add_assign(&mut self, other: Self) {
        self.ore += other.ore;
        self.clay += other.clay;
        self.obsidian += other.obsidian;
        self.geode += other.geode;
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Costs {
    pub ore_robot_cost: Units,
    pub clay_robot_cost: Units,
    pub obsidian_robot_cost: Units,
    pub geode_robot_cost: Units,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Blueprint {
    pub id: u32,
    pub costs: Costs,
    pub max_material_cost: Units,
}

impl Blueprint {
    pub fn new(
        id: u32,
        ore_robot_cost: Units,
        clay_robot_cost: Units,
        obsidian_robot_cost: Units,
        geode_robot_cost: Units,
    ) -> Self {
        Self {
            id,
            costs: Costs {
                ore_robot_cost: ore_robot_cost.clone(),
                clay_robot_cost: clay_robot_cost.clone(),
                obsidian_robot_cost: obsidian_robot_cost.clone(),
                geode_robot_cost: geode_robot_cost.clone(),
            },
            max_material_cost: Units::new(
                max(
                    ore_robot_cost.ore,
                    max(
                        clay_robot_cost.ore,
                        max(obsidian_robot_cost.ore, geode_robot_cost.ore),
                    ),
                ),
                obsidian_robot_cost.clay,
                geode_robot_cost.obsidian,
                0, // geo is not used in the max material cost
            ),
        }
    }
}
