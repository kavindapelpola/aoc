use criterion::{criterion_group, criterion_main, Criterion};
use utils::algorithms::optimise::exhaustive::best_recursed;
use utils::traits::problem::{Neighbours, Score};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Path {
    path: Vec<String>,
}

impl Score<Path> for Path {
    fn score(&self) -> u32 {
        let last = self.path.last();
        if last.is_none() {
            return 0;
        }
        last.unwrap().chars().filter(|c| *c == '1').count() as u32
    }
}

pub struct UninformedSearchProblem {}
impl Neighbours<Path> for UninformedSearchProblem {
    fn neighbours(&self, state: &Path) -> Vec<Path> {
        if state.path.len() > 10 {
            return vec![];
        }
        let last = state.path.last().unwrap();
        let mut new_0 = state.path.clone();
        new_0.push(last.clone() + ".0");
        let mut new_1 = state.path.clone();
        new_1.push(last.clone() + ".1");
        vec![Path { path: new_0 }, Path { path: new_1 }]
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let p = UninformedSearchProblem {};
    c.bench_function("search exhaustive", |b| {
        b.iter(|| {
            best_recursed(
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
