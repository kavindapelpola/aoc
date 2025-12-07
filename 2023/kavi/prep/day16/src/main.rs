use std::collections::HashMap;
use utils::structures::graph::Graph;
use utils::traits::problem::{Neighbours, Score};

mod load;
mod part_a;
mod part_b;

#[derive(Clone, PartialEq, Eq, Debug)]
struct State {
    cur_valve: String,
    open_valves: Vec<String>,
    time_left: u32,
    lifetime_flow: u32,
}

impl Score<State> for State {
    fn score(&self) -> u32 {
        self.lifetime_flow
    }
}

struct App {
    graph: Graph<String, u32>,
    shortest_paths: HashMap<String, HashMap<String, i32>>,
}

impl App {
    fn new(graph: Graph<String, u32>) -> App {
        let all_shortest_paths = graph.shortest_paths();
        let mut shortest_paths = HashMap::<String, HashMap<String, i32>>::new();
        for (valve, all_neighbours) in all_shortest_paths {
            let mut neighbours = HashMap::<String, i32>::new();
            for (neighbour, dist) in all_neighbours {
                // remove paths to broken valves and self
                if dist > 0 && graph.nodes.get(&neighbour).unwrap() > &0 {
                    neighbours.insert(neighbour, dist);
                }
            }
            shortest_paths.insert(valve, neighbours);
        }
        App {
            graph,
            shortest_paths,
        }
    }
}

impl Neighbours<State> for App {
    // all states that will result in increasing the flow
    fn neighbours(&self, state: &State) -> Vec<State> {
        // good neighbouts increase the flow vs. bad neighbouts are moves we can make that dont increate the flow
        let mut good_neighbour_states = vec![];

        // if we have one second or less, we can't do anything
        if state.time_left <= 1 {
            return good_neighbour_states;
        }

        let all_neighbours = self.shortest_paths.get(&state.cur_valve).unwrap();
        for (neighbour, dist) in all_neighbours {
            // skip if there is not enough time to get to this valve, open it and get some flow
            if state.time_left < *dist as u32 + 2 {
                continue;
            }
            // skip if we have already opened this valve
            if state.open_valves.contains(neighbour) {
                continue;
            }
            // passed all tests so add to good neighbours
            let mut open_valves = state.open_valves.clone();
            open_valves.push(neighbour.clone());
            let time_left = state.time_left - *dist as u32 - 1; // travel to it and switch it on
            good_neighbour_states.push(State {
                cur_valve: neighbour.clone(),
                open_valves: open_valves,
                time_left: time_left,
                lifetime_flow: state.lifetime_flow
                    + *self.graph.nodes.get(neighbour).unwrap() * time_left,
            });
        }
        good_neighbour_states
    }
}

fn main() {
    part_a::run();
    part_b::run();
}
