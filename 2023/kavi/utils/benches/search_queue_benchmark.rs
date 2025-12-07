use criterion::{criterion_group, criterion_main, Criterion};
use utils::algorithms::search::queue::search_bfs;
use utils::traits::problem::{IsGoal, Neighbours};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Path {
    path: Vec<String>,
}

pub struct UninformedSearchProblem {}
impl Neighbours<Path> for UninformedSearchProblem {
    fn neighbours(&self, state: &Path) -> Vec<Path> {
        let last = state.path.last().unwrap();
        let mut new_0 = state.path.clone();
        new_0.push(last.clone() + ".0");
        let mut new_1 = state.path.clone();
        new_1.push(last.clone() + ".1");
        vec![Path { path: new_0 }, Path { path: new_1 }]
    }
}

impl IsGoal<Path> for UninformedSearchProblem {
    fn is_goal(&self, state: &Path) -> bool {
        state.path.last().unwrap() == "0.1.1.0.0.1.1.0.1.1"
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let p = UninformedSearchProblem {};
    c.bench_function("search bfs", |b| {
        b.iter(|| {
            search_bfs(
                &p,
                &Path {
                    path: vec!["0".to_string()],
                },
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
