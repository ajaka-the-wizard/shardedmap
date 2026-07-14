# Benchmarks

All benchmarks run with [Criterion](https://github.com/bheisler/criterion.rs). Each concurrent benchmark spawns N threads, each performing 1,000 insert+get pairs on disjoint key ranges, and measures total wall-clock time for all threads to complete. Raw reports (HTML + JSON) are in `criterion/`; run `cargo bench` to reproduce. Criterion also generates interactive HTML reports with violin plots and regression lines for each benchmark - after running `cargo bench`, open `criterion/report/index.html` in a browser for the full visual breakdown

## ShardedMap vs. DashMap vs. Mutex\<HashMap\>

ShardedMap (16 shards) benchmarked against [DashMap](https://github.com/xacrimon/dashmap) and a naive `Mutex<HashMap>` baseline, across 1–16 threads:

| Threads | ShardedMap (ms) | DashMap (ms) | Mutex\<HashMap\> (ms) |
|--------:|-----------------:|--------------:|------------------------:|
| 1       | 0.72             | 0.58          | 0.63                   |
| 2       | 1.32             | 0.83          | 1.10                   |
| 4       | 1.47             | 1.13          | 3.11                   |
| 8       | 2.81             | 2.07          | 6.24                   |
| 16      | 5.42             | 3.91          | 12.70                  |

**Takeaways:**
- `Mutex<HashMap>` scales linearly with thread count (time roughly doubles every doubling of threads) — the expected signature of a single global lock fully serializing all access.
- ShardedMap and DashMap both scale sub-linearly: per-thread cost drops as thread count rises, since work spreads across independent locks instead of queuing on one.
- ShardedMap delivers **~2.3x higher throughput than the `Mutex<HashMap>` baseline at 16 threads** (5.42ms vs. 12.70ms).
- ShardedMap trails DashMap by **~24% single-threaded, widening to ~39% at 16 threads**. DashMap is a heavily-optimized, industry-standard crate with hand-tuned internals; ShardedMap's gap is consistent with the generic `ShardableMap`/`ShardLock` trait-dispatch overhead it pays on every operation in exchange for pluggable backends.

## Shard count tuning (`concurrent_mixed`)

Isolating shard count as an independent variable (1/4/16/64 shards) across thread counts, holding everything else fixed:

| Threads | 1 shard (ms) | 4 shards (ms) | 16 shards (ms) | 64 shards (ms) |
|--------:|--------------:|----------------:|------------------:|------------------:|
| 1       | 0.62         | 0.64            | 0.71              | 0.76              |
| 2       | 1.49         | 1.10            | 0.95              | 0.93              |
| 4       | 4.09         | 2.09            | 1.51              | 1.29              |
| 8       | 7.87         | 3.78            | 2.88              | 2.49              |

**Takeaways:**
- At **1 thread (no contention possible)**, more shards only cost overhead: 64 shards is ~22% slower than 1 shard (0.76ms vs. 0.62ms), from the added cost of hashing to a shard index and holding more lock instances with nothing to parallelize against.
- At **2+ threads**, the picture flips: going from 1→2 threads, the 1-shard config nearly doubles (1.49ms, consistent with degenerating to a single global lock), while the 64-shard config barely moves (0.93ms, +22%) — the shards are absorbing the concurrent load as designed.
- This confirms the core design tradeoff directly: shard count is a real dial between single-threaded overhead and multi-threaded scalability, not just a knob in theory.

## Single-threaded insert overhead

| Shard count | Mean insert time (ns) |
|------------:|------------------------:|
| 1           | 324.5                  |
| 8           | 313.3                  |
| 64          | 323.6                  |

No meaningful difference (within noise) — sharding overhead is negligible for single-threaded workloads at this key/value size, as expected.

## HashMap vs. BTreeMap backend (single-key get)

| Backend   | Mean get time (ns) |
|-----------|----------------------:|
| HashMap   | 103.6                |
| BTreeMap  | 72.8                 |

BTreeMap's `get` outperforms HashMap's here — but this is an artifact of the benchmark using a single stored key (n=1), not a general result. Rust's default `HashMap` hasher (SipHash) pays a fixed, DoS-resistant hashing cost on every lookup regardless of map size, while BTreeMap on a single-entry tree does effectively zero comparisons. At larger n, HashMap's O(1) average case would be expected to win. Included here for completeness, not as a backend recommendation.

## Reading the raw reports

- `mean` vs. `median`: if `mean` is noticeably higher than `median`, a handful of runs were slow outliers (scheduling noise, thread preemption) — look at `report/pdf.svg` (violin plot) to confirm a long right tail rather than a shifted distribution.
- Coefficient of variation (`std_dev / mean`) above ~10% on these benchmarks generally indicates environmental noise rather than a real property of the code, particularly at low thread counts where total work per run is small.
