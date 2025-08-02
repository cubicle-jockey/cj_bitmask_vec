use cj_bitmask_vec::prelude::*;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;

fn bench_basic_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("basic_operations");

    // Benchmark new()
    group.bench_function("new", |b| {
        b.iter(|| {
            let _vec: BitmaskVec<u8, i32> = black_box(BitmaskVec::new());
        })
    });

    // Benchmark with_capacity()
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("with_capacity", size), size, |b, &size| {
            b.iter(|| {
                let _vec: BitmaskVec<u8, i32> = black_box(BitmaskVec::with_capacity(size));
            })
        });
    }

    // Benchmark push operations
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("push", size), size, |b, &size| {
            b.iter(|| {
                let mut vec: BitmaskVec<u8, i32> = BitmaskVec::new();
                for i in 0..size {
                    vec.push(black_box(i as i32));
                }
                black_box(vec)
            })
        });

        group.bench_with_input(
            BenchmarkId::new("push_with_mask", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut vec = BitmaskVec::new();
                    for i in 0..size {
                        vec.push_with_mask(black_box((i % 256) as u8), black_box(i as i32));
                    }
                    black_box(vec)
                })
            },
        );
    }

    group.finish();
}

fn bench_indexing_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("indexing_operations");

    // Create test vectors of different sizes
    for size in [100, 1000, 10000].iter() {
        let mut vec = BitmaskVec::new();
        for i in 0..*size {
            vec.push_with_mask((i % 256) as u8, i as i32);
        }

        group.bench_with_input(BenchmarkId::new("index_access", size), &vec, |b, vec| {
            b.iter(|| {
                let idx = black_box(vec.len() / 2);
                black_box(vec[idx])
            })
        });

        group.bench_with_input(BenchmarkId::new("pop", size), size, |b, &size| {
            b.iter(|| {
                let mut vec = BitmaskVec::new();
                for i in 0..size {
                    vec.push_with_mask((i % 256) as u8, i as i32);
                }
                while !vec.is_empty() {
                    black_box(vec.pop());
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("pop_with_mask", size), size, |b, &size| {
            b.iter(|| {
                let mut vec = BitmaskVec::new();
                for i in 0..size {
                    vec.push_with_mask((i % 256) as u8, i as i32);
                }
                while !vec.is_empty() {
                    black_box(vec.pop_with_mask());
                }
            })
        });
    }

    group.finish();
}

fn bench_iteration_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("iteration_operations");

    // Benchmark iteration operations
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("iter", size), size, |b, &size| {
            b.iter(|| {
                let mut vec = BitmaskVec::new();
                for i in 0..size {
                    vec.push_with_mask((i % 256) as u8, i as i32);
                }

                let mut sum = 0i64;
                for item in vec.iter() {
                    sum += *item as i64;
                }
                black_box(sum)
            })
        });

        group.bench_with_input(
            BenchmarkId::new("iter_with_mask", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut vec = BitmaskVec::new();
                    for i in 0..size {
                        vec.push_with_mask((i % 256) as u8, i as i32);
                    }

                    let mut sum = 0i64;
                    for pair in vec.iter_with_mask() {
                        sum += pair.item as i64;
                    }
                    black_box(sum)
                })
            },
        );

        // Benchmark mutable iteration
        group.bench_with_input(BenchmarkId::new("iter_mut", size), size, |b, &size| {
            b.iter(|| {
                let mut vec = BitmaskVec::new();
                for i in 0..size {
                    vec.push_with_mask((i % 256) as u8, i as i32);
                }

                for item in vec.iter_mut() {
                    *item = black_box(*item * 2);
                }
                black_box(vec)
            })
        });

        group.bench_with_input(
            BenchmarkId::new("iter_with_mask_mut", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut vec = BitmaskVec::new();
                    for i in 0..size {
                        vec.push_with_mask((i % 256) as u8, i as i32);
                    }

                    for pair in vec.iter_with_mask_mut() {
                        pair.item = black_box(pair.item * 2);
                    }
                    black_box(vec)
                })
            },
        );
    }

    group.finish();
}

fn bench_filtering_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("filtering_operations");

    // Benchmark filtering operations
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("filter_mask", size), size, |b, &size| {
            b.iter(|| {
                let mut vec = BitmaskVec::new();
                for i in 0..size {
                    vec.push_with_mask((i % 256) as u8, i as i32);
                }

                let mut count = 0;
                let mut iter = vec.iter_with_mask();
                let mask = black_box(0b00000010u8);
                while let Some(_pair) = iter.filter_mask(&mask) {
                    count += 1;
                }
                black_box(count)
            })
        });

        group.bench_with_input(
            BenchmarkId::new("matches_mask_iteration", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut vec = BitmaskVec::new();
                    for i in 0..size {
                        vec.push_with_mask((i % 256) as u8, i as i32);
                    }

                    let mut sum = 0i64;
                    let mask = black_box(0b00000010u8);
                    for pair in vec.iter_with_mask() {
                        if pair.matches_mask(&mask) {
                            sum += pair.item as i64;
                        }
                    }
                    black_box(sum)
                })
            },
        );
    }

    group.finish();
}

fn bench_collection_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("collection_operations");

    // Benchmark append operations
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("append", size), size, |b, &size| {
            b.iter(|| {
                let mut vec1 = BitmaskVec::new();
                let mut vec2 = BitmaskVec::new();

                for i in 0..size {
                    vec1.push_with_mask((i % 256) as u8, i as i32);
                    vec2.push_with_mask(((i + size) % 256) as u8, (i + size) as i32);
                }

                vec1.append(&mut vec2);
                black_box(vec1)
            })
        });

        group.bench_with_input(BenchmarkId::new("clear", size), size, |b, &size| {
            b.iter(|| {
                let mut vec = BitmaskVec::new();
                for i in 0..size {
                    vec.push_with_mask((i % 256) as u8, i as i32);
                }
                vec.clear();
                black_box(vec)
            })
        });

        group.bench_with_input(BenchmarkId::new("resize", size), size, |b, &size| {
            b.iter(|| {
                let mut vec: BitmaskVec<u8, i32> = BitmaskVec::new();
                vec.resize(size, black_box(42i32));
                black_box(vec)
            })
        });

        group.bench_with_input(
            BenchmarkId::new("resize_with_mask", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut vec = BitmaskVec::new();
                    vec.resize_with_mask(size, black_box(0b10101010u8), black_box(42i32));
                    black_box(vec)
                })
            },
        );
    }

    group.finish();
}

fn bench_different_bitmask_types(c: &mut Criterion) {
    let mut group = c.benchmark_group("different_bitmask_types");

    let size = 1000;

    // Benchmark with u8 bitmask
    group.bench_function("u8_bitmask", |b| {
        b.iter(|| {
            let mut vec: BitmaskVec<u8, i32> = BitmaskVec::new();
            for i in 0..size {
                vec.push_with_mask((i % 256) as u8, i as i32);
            }

            let mut sum = 0i64;
            for pair in vec.iter_with_mask() {
                if pair.matches_mask(&0b00000010u8) {
                    sum += pair.item as i64;
                }
            }
            black_box(sum)
        })
    });

    // Benchmark with u16 bitmask
    group.bench_function("u16_bitmask", |b| {
        b.iter(|| {
            let mut vec: BitmaskVec<u16, i32> = BitmaskVec::new();
            for i in 0..size {
                vec.push_with_mask((i % 65536) as u16, i as i32);
            }

            let mut sum = 0i64;
            for pair in vec.iter_with_mask() {
                if pair.matches_mask(&0b0000001000000000u16) {
                    sum += pair.item as i64;
                }
            }
            black_box(sum)
        })
    });

    // Benchmark with u32 bitmask
    group.bench_function("u32_bitmask", |b| {
        b.iter(|| {
            let mut vec: BitmaskVec<u32, i32> = BitmaskVec::new();
            for i in 0..size {
                vec.push_with_mask(i as u32, i as i32);
            }

            let mut sum = 0i64;
            for pair in vec.iter_with_mask() {
                if pair.matches_mask(&0b00000000000000000000001000000000u32) {
                    sum += pair.item as i64;
                }
            }
            black_box(sum)
        })
    });

    // Benchmark with u64 bitmask
    group.bench_function("u64_bitmask", |b| {
        b.iter(|| {
            let mut vec: BitmaskVec<u64, i32> = BitmaskVec::new();
            for i in 0..size {
                vec.push_with_mask(i as u64, i as i32);
            }

            let mut sum = 0i64;
            for pair in vec.iter_with_mask() {
                if pair.matches_mask(
                    &0b0000000000000000000000000000000000000000000000000000001000000000u64,
                ) {
                    sum += pair.item as i64;
                }
            }
            black_box(sum)
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_basic_operations,
    bench_indexing_operations,
    bench_iteration_operations,
    bench_filtering_operations,
    bench_collection_operations,
    bench_different_bitmask_types
);
criterion_main!(benches);
