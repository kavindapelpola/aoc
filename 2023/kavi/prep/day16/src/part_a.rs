use crate::{App, State};
use utils::algorithms::optimise::exhaustive::best_recursed;
use utils::structures::graph::Graph;

pub fn run() {
    let mut graph = Graph::<String, u32>::new();
    include_str!("../input.txt")
        .split("\n")
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let r = super::load::parse_valve(line).unwrap().1;
            graph.add_node(r.0 .0.to_string(), r.0 .1 as u32);
            graph.add_unit_edges(
                r.0 .0.to_string(),
                r.1.iter().map(|v| v.to_string()).collect(),
            );
        });

    let app = App::new(graph);

    let best = best_recursed(
        &app,
        &State {
            cur_valve: "AA".to_string(),
            open_valves: vec![],
            time_left: 30,
            lifetime_flow: 0,
        },
    );

    println!("{:?}", best);
}

#[cfg(test)]
mod tests {
    use utils::traits::problem::Neighbours;

    use super::*;

    const RAW_STR: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    fn get_app() -> App {
        let mut graph = Graph::<String, u32>::new();
        RAW_STR
            .split("\n")
            .filter(|line| !line.is_empty())
            .for_each(|line| {
                let r = super::super::load::parse_valve(line).unwrap().1;
                graph.add_node(r.0 .0.to_string(), r.0 .1 as u32);
                graph.add_unit_edges(
                    r.0 .0.to_string(),
                    r.1.iter().map(|v| v.to_string()).collect(),
                );
            });

        App::new(graph)
    }

    fn initial_state() -> State {
        State {
            cur_valve: "AA".to_string(),
            open_valves: vec![],
            time_left: 30,
            lifetime_flow: 0,
        }
    }

    #[test]
    fn good_neighbours_returns_neighbouring_good_valves_in_one_time_unit() {
        let a = get_app();
        let neighbours = a.neighbours(&initial_state());
        assert!(neighbours.contains(&State {
            cur_valve: "DD".to_string(),
            open_valves: vec!["DD".to_string()],
            time_left: 28,
            lifetime_flow: 560,
        }));
        assert!(neighbours.contains(&State {
            cur_valve: "BB".to_string(),
            open_valves: vec!["BB".to_string()],
            time_left: 28,
            lifetime_flow: 364,
        }));
    }

    #[test]
    fn good_neighbours_does_not_return_neighbouring_broken_vales() {
        let a = get_app();
        let neighbours = a.neighbours(&initial_state());
        let found_ii = neighbours.iter().find(|s| s.cur_valve == "II");
        let found_gg = neighbours.iter().find(|s| s.cur_valve == "GG");
        let found_ff = neighbours.iter().find(|s| s.cur_valve == "FF");
        assert!(found_ii.is_none());
        assert!(found_gg.is_none());
        assert!(found_ff.is_none());
    }

    #[test]
    fn good_neighbours_does_not_return_self() {
        let a = get_app();
        let neighbours = a.neighbours(&initial_state());
        let found_aa = neighbours.iter().find(|s| s.cur_valve == "AA");
        assert!(found_aa.is_none());
    }

    #[test]
    fn good_neighbours_returns_multi_time_units() {
        let a = get_app();
        let neighbours = a.neighbours(&initial_state());
        assert!(neighbours.contains(&State {
            cur_valve: "HH".to_string(),
            open_valves: vec!["HH".to_string()],
            time_left: 24,
            lifetime_flow: 528,
        }));
        assert!(neighbours.contains(&State {
            cur_valve: "CC".to_string(),
            open_valves: vec!["CC".to_string()],
            time_left: 27,
            lifetime_flow: 54,
        }));
        assert!(neighbours.contains(&State {
            cur_valve: "JJ".to_string(),
            open_valves: vec!["JJ".to_string()],
            time_left: 27,
            lifetime_flow: 567,
        }));
        assert!(neighbours.contains(&State {
            cur_valve: "EE".to_string(),
            open_valves: vec!["EE".to_string()],
            time_left: 27,
            lifetime_flow: 81,
        }));
    }

    #[test]
    fn search_returns_best_path() {
        let a = get_app();

        let best = best_recursed(&a, &initial_state());

        assert_eq!(best.lifetime_flow, 1651);
    }
}
