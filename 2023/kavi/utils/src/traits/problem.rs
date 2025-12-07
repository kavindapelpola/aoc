use std::fmt::Debug;

// neighbours returns a list of neighbours for a given state.
pub trait Neighbours<S: Clone + Debug> {
    fn neighbours(&self, state: &S) -> Vec<S>;

    fn recurse(&self, seed: &S) -> Vec<S> {
        let neighbours = self.neighbours(seed);
        if neighbours.len() == 0 {
            return vec![seed.clone()];
        }
        let mut end_states = vec![];
        for neighbour in neighbours {
            let ends = self.recurse(&neighbour);
            end_states.extend(ends);
        }
        end_states
    }

    fn debug_depth(&self, seed: &S, depth: usize) {
        let mut depth_states = Vec::<(usize, Vec<S>)>::new();
        depth_states.push((0, vec![seed.clone()]));
        for i in 0..depth {
            println!("depth: {}", i);
            let mut next_states = Vec::<S>::new();
            for state in depth_states[i].1.iter() {
                println!("state: {:?}", state);
                let neighbours = self.neighbours(state);
                for neighbour in neighbours.iter() {
                    println!("neighbour: {:?}", neighbour);
                }
                println!();
                next_states.extend(neighbours);
            }
            depth_states.push((i + 1, next_states));
            println!("---\n");
        }
    }
}

pub trait Score<S: Clone> {
    fn score(&self) -> u32 {
        0
    }
}

pub fn best_state<S: Clone + Score<S>>(states: &Vec<S>) -> S {
    let mut best = states.first().unwrap().clone();
    for state in states {
        if state.score() > best.score() {
            best = state.clone();
        }
    }
    best.clone()
}

pub trait IsGoal<S> {
    fn is_goal(&self, state: &S) -> bool;
}
