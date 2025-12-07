use crate::traits::problem::{best_state, Neighbours, Score};
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

// recurse all possible paths and return the best one
pub fn best_recursed<S: Clone + Debug + Score<S>>(problem: &impl Neighbours<S>, seed: &S) -> S {
    best_state(&problem.recurse(seed))
}

// explore the whole tree overwriting frontier nodes with the best nodes found and return the best one
pub fn best_explored<S>(problem: &impl Neighbours<S>, seed: &S) -> S
where
    S: Clone + Hash + PartialEq + Eq + Debug + Score<S>,
{
    let mut frontier = HashSet::<S>::new();
    let mut explored = HashSet::<S>::new();
    let mut best = seed.clone();

    // add seed states to frontier
    frontier.insert(seed.clone());

    // main search loop
    while frontier.len() > 0 {
        let current = frontier.iter().next().unwrap().clone();
        frontier.remove(&current);
        explored.insert(current.clone());

        let neighbours = problem.neighbours(&current);

        // get the best end state
        if neighbours.len() == 0 && current.score() > best.score() {
            best = current.clone();
            continue;
        }

        for neighbour in neighbours {
            if !frontier.contains(&neighbour) && !explored.contains(&neighbour) {
                frontier.insert(neighbour.clone());
            }
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;

    //          Graph A
    //
    //              0
    //            /   \
    //           1     2*
    //          / \   / \
    //         3    4    5
    //            /   \
    //           6     7*
    //
    // * = score of 2, rest is 1

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Path {
        path: Vec<i32>,
    }

    impl Score<Path> for Path {
        fn score(&self) -> u32 {
            let mut score = 0;
            for pos in &self.path {
                let pos_score = match pos {
                    0 => 1,
                    1 => 1,
                    2 => 2,
                    3 => 1,
                    4 => 1,
                    5 => 1,
                    6 => 1,
                    7 => 2,
                    _ => 0,
                };
                score += pos_score;
            }
            score
        }
    }

    fn generate_states(state: &Path, nums_to_add: Vec<i32>) -> Vec<Path> {
        let mut states = Vec::new();
        for num in nums_to_add {
            let mut new_path = state.path.clone();
            new_path.push(num);
            states.push(Path { path: new_path });
        }
        states
    }

    struct UninformedSearchProblem {}
    impl Neighbours<Path> for UninformedSearchProblem {
        fn neighbours(&self, state: &Path) -> Vec<Path> {
            match state.path.last().unwrap() {
                0 => generate_states(state, vec![1, 2]),
                1 => generate_states(state, vec![3, 4]),
                2 => generate_states(state, vec![4, 5]),
                3 => generate_states(state, vec![]),
                4 => generate_states(state, vec![6, 7]),
                5 => generate_states(state, vec![]),
                6 => generate_states(state, vec![]),
                7 => generate_states(state, vec![]),
                _ => generate_states(state, vec![]),
            }
        }
    }

    #[test]
    fn best_recursed_returns_best_state() {
        let a = UninformedSearchProblem {};
        let best_state = best_recursed(&a, &Path { path: vec![0] });
        assert_eq!(best_state.path, vec![0, 2, 4, 7]);
    }

    #[test]
    fn best_explored_returns_best_state() {
        let a = UninformedSearchProblem {};
        let best_state = best_explored(&a, &Path { path: vec![0] });
        assert_eq!(best_state.path, vec![0, 2, 4, 7]);
    }
}
