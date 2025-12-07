use crate::traits::problem::{IsGoal, Neighbours, Score};
use std::fmt::Debug;
use std::vec;

pub fn search_astar<S: Clone + Debug + PartialEq + Eq + Score<S>>(
    problem: &(impl Neighbours<S> + IsGoal<S>),
    seed: &S,
) -> Option<S> {
    let mut frontier = vec![];
    let mut explored = vec![];

    // add seed states to frontier
    frontier.push(seed.clone());

    // main search loop
    while frontier.len() > 0 {
        let index = {
            let mut best_index = 0;
            let mut best_score = 0;
            for (i, s) in frontier.iter().enumerate() {
                let score = s.score();
                if score > best_score {
                    best_score = score;
                    best_index = i;
                }
            }
            best_index
        };
        let current = frontier.remove(index);
        explored.push(current.clone());

        for neighbour in problem.neighbours(&current) {
            if problem.is_goal(&neighbour) {
                // found a path
                return Some(neighbour);
            }
            if !explored.contains(&neighbour) && !frontier.contains(&neighbour) {
                frontier.push(neighbour);
            }
        }
    }

    // could not find a path
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    type Position = (usize, usize);

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct State {
        path: Vec<Position>,
        cost: u32,
    }

    impl Score<State> for State {
        fn score(&self) -> u32 {
            // Manghattan distance
            let (x, y) = self.path.last().unwrap();
            (11 - x + y) as u32
        }
    }

    struct InformedSearchProblem {
        maze: Vec<Vec<char>>,
    }

    impl InformedSearchProblem {
        fn new() -> InformedSearchProblem {
            let input = vec![
                "X...........", // 0
                "X.XXXXXXXXX.", // 1
                "X.X.......X.", // 2
                "X.X.XXXXX.X.", // 3
                "X.,.X.....X.", // 4
                "XXX.X.XXXXX.", // 5
                "....X.......", // 6
            ];
            InformedSearchProblem {
                maze: input.iter().map(|s| s.chars().collect()).collect(),
            }
        }
    }

    impl Neighbours<State> for InformedSearchProblem {
        fn neighbours(&self, current: &State) -> Vec<State> {
            let mut neighbours = Vec::new();
            let (x, y) = current.path.last().unwrap();
            if *x > 0 && self.maze[*y][*x - 1] != 'X' && !current.path.contains(&(*x - 1, *y)) {
                let mut new_path = current.path.clone();
                new_path.push((*x - 1, *y));
                neighbours.push(State {
                    path: new_path,
                    cost: current.cost + 1,
                });
            }
            if *x < self.maze[*y].len() - 1
                && self.maze[*y][x + 1] != 'X'
                && !current.path.contains(&(*x + 1, *y))
            {
                let mut new_path = current.path.clone();
                new_path.push((*x + 1, *y));
                neighbours.push(State {
                    path: new_path,
                    cost: current.cost + 1,
                });
            }
            if *y > 0 && self.maze[*y - 1][*x] != 'X' && !current.path.contains(&(*x, *y - 1)) {
                let mut new_path = current.path.clone();
                new_path.push((*x, *y - 1));
                neighbours.push(State {
                    path: new_path,
                    cost: current.cost + 1,
                });
            }
            if *y < self.maze.len() - 1
                && self.maze[*y + 1][*x] != 'X'
                && !current.path.contains(&(*x, *y + 1))
            {
                let mut new_path = current.path.clone();
                new_path.push((*x, *y + 1));
                neighbours.push(State {
                    path: new_path,
                    cost: current.cost + 1,
                });
            }
            neighbours
        }
    }

    impl IsGoal<State> for InformedSearchProblem {
        fn is_goal(&self, state: &State) -> bool {
            state.path.last().unwrap() == &(11, 0)
        }
    }

    #[test]
    fn a_star_search_with_single_seed_finds_best_path() {
        let a = InformedSearchProblem::new();
        let solution = search_astar(
            &a,
            &State {
                path: vec![(0, 6)],
                cost: 0,
            },
        );
        assert_eq!(
            solution,
            Some(State {
                path: vec![
                    (0, 6),
                    (1, 6),
                    (2, 6),
                    (3, 6),
                    (3, 5),
                    (3, 4),
                    (2, 4),
                    (1, 4),
                    (1, 3),
                    (1, 2),
                    (1, 1),
                    (1, 0),
                    (2, 0),
                    (3, 0),
                    (4, 0),
                    (5, 0),
                    (6, 0),
                    (7, 0),
                    (8, 0),
                    (9, 0),
                    (10, 0),
                    (11, 0)
                ],
                cost: 21
            },)
        );
    }
}
