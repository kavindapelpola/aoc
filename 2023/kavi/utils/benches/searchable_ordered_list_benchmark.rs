use criterion::{criterion_group, criterion_main, Criterion};
use utils::structures::searchable_ordered_list::SearchableOrderedList;

fn filled_list(max: usize) -> SearchableOrderedList<usize> {
    let mut l = SearchableOrderedList::<usize>::new();
    for i in 0..max {
        l.push_back(i);
    }
    l
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut c1 = SearchableOrderedList::<usize>::new();
    c.bench_function("add item", |b| b.iter(|| c1.push_back(c1.len())));

    let mut d1 = filled_list(100_000);
    c.bench_function("add and delete item", |b| {
        b.iter(|| {
            d1.push_back(d1.len());
            d1.pop_back()
        })
    });

    let l1 = filled_list(100_000);
    c.bench_function("contains 100_000", |b| b.iter(|| l1.contains(&100_000)));

    let l2 = filled_list(1000);
    c.bench_function("contains 1000", |b| b.iter(|| l2.contains(&1000)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
