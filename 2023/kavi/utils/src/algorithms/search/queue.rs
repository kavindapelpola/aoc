use crate::{
    structures::searchable_ordered_list::SearchableOrderedList,
    traits::problem::{IsGoal, Neighbours},
};
use std::fmt::Debug;
use std::hash::Hash;

enum DequeAlgorithm {
    BFS,
    DFS,
}

// breadth first search
pub fn search_bfs<S: Hash + Clone + Debug + PartialEq + Eq>(
    problem: &(impl Neighbours<S> + IsGoal<S>),
    seed: &S,
) -> Option<S> {
    search_queue(problem, seed, DequeAlgorithm::BFS)
}

// depth first search
pub fn search_dfs<S: Hash + Clone + Debug + PartialEq + Eq>(
    problem: &(impl Neighbours<S> + IsGoal<S>),
    seed: &S,
) -> Option<S> {
    search_queue(problem, seed, DequeAlgorithm::DFS)
}

fn search_queue<S: Hash + Clone + Debug + PartialEq + Eq>(
    problem: &(impl Neighbours<S> + IsGoal<S>),
    seed: &S,
    alg: DequeAlgorithm,
) -> Option<S> {
    let mut frontier = SearchableOrderedList::<S>::new();
    let mut explored = vec![];

    // add seed states to frontier
    frontier.push_front(seed.clone());

    // main search loop
    while frontier.len() > 0 {
        let current = match alg {
            DequeAlgorithm::BFS => frontier.pop_front(),
            DequeAlgorithm::DFS => frontier.pop_back(),
        }
        .unwrap();
        explored.push(current.clone());

        for neighbour in problem.neighbours(&current) {
            if problem.is_goal(&neighbour) {
                // found a path
                return Some(neighbour);
            }
            if !explored.contains(&neighbour) && !frontier.contains(&neighbour) {
                frontier.push_back(neighbour);
            }
        }
    }

    // could not find a path
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    //          Graph A
    //
    //              0
    //            /   \
    //           1     2
    //          / \   / \
    //         3   4 5   6
    //            /   \
    //           7 -9- 8

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Path {
        path: Vec<i32>,
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
                1 => generate_states(state, vec![0, 3, 4]),
                2 => generate_states(state, vec![0, 5, 6]),
                3 => generate_states(state, vec![1]),
                4 => generate_states(state, vec![1, 7]),
                5 => generate_states(state, vec![2, 8]),
                6 => generate_states(state, vec![2]),
                7 => generate_states(state, vec![9]),
                8 => generate_states(state, vec![5]),
                9 => generate_states(state, vec![8]),
                _ => generate_states(state, vec![]),
            }
        }
    }

    impl IsGoal<Path> for UninformedSearchProblem {
        fn is_goal(&self, current: &Path) -> bool {
            *current.path.last().unwrap() == 8
        }
    }

    #[test]
    fn bfs_search_with_seed_finds_best_path() {
        let a = UninformedSearchProblem {};
        let solution = search_bfs(&a, &Path { path: vec![0] });
        assert_eq!(
            solution,
            Some(Path {
                path: vec![0, 2, 5, 8]
            })
        );
    }
}
