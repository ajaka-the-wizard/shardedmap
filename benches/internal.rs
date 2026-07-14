use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use shardedmap::Builder;
use std::sync::Arc;
use std::thread;

fn bench_single_threaded(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_threaded");
    for shards in [1, 8, 64] {
        let map = Builder::new_default_lock_with_hashmap::<u64, u64>(shards);
        group.bench_with_input(BenchmarkId::new("insert", shards), &shards, |b, _| {
            let mut i = 0u64;
            b.iter(|| {
                map.insert(black_box(i), black_box(i));
                i += 1;
            });
        });
    }
    group.finish();
}

fn bench_concurrent(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_mixed");
    for shards in [1, 4, 16, 64] {
        for threads in [1, 2, 4, 8] {
            let id = BenchmarkId::new(format!("shards_{shards}"), threads);
            group.bench_with_input(id, &threads, |b, &threads| {
                b.iter(|| {
                    let map = Arc::new(Builder::new_default_lock_with_hashmap::<u64, u64>(shards));
                    let handles: Vec<_> = (0..threads)
                        .map(|t| {
                            let map = Arc::clone(&map);
                            thread::spawn(move || {
                                for i in 0..1000u64 {
                                    let key = (t as u64) * 1000 + i;
                                    map.insert(key, key);
                                    black_box(map.get(&key));
                                }
                            })
                        })
                        .collect();
                    for h in handles {
                        h.join().unwrap();
                    }
                });
            });
        }
    }
    group.finish();
}

// Q3: backend comparison — HashMap vs BTreeMap vs CustomMap, apples to apples
fn bench_backends(c: &mut Criterion) {
    let mut group = c.benchmark_group("backend_comparison");
    let hashmap = Builder::new_default_lock_with_hashmap::<u64, u64>(16);
    let btreemap = Builder::new_default_lock_with_btreemap::<u64, u64>(16);

    group.bench_function("hashmap_get", |b| {
        hashmap.insert(42, 42);
        b.iter(|| black_box(hashmap.get(&42)));
    });
    group.bench_function("btreemap_get", |b| {
        btreemap.insert(42, 42);
        b.iter(|| black_box(btreemap.get(&42)));
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_single_threaded,
    bench_concurrent,
    bench_backends
);
criterion_main!(benches);
