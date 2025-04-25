use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use emap::Map;

macro_rules! bench_capacity_some {
    ($c:expr, $type:ty, $default_value:expr, $name:expr) => {
        let mut group = $c.benchmark_group(format!("Map_{}", $name));

        for el in [10, 100, 1000, 10_000, 25_000, 50_000, 75_000, 100_000].iter() {
            group.bench_with_input(BenchmarkId::new("std", el), el, |b, el| {
                b.iter(|| {
                    black_box(Map::<$type>::with_capacity_some(
                        black_box(*el),
                        black_box($default_value),
                    ));
                })
            });

            #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
            group.bench_with_input(BenchmarkId::new("sse", el), el, |b, el| {
                b.iter(|| {
                    black_box(Map::<$type>::with_capacity_some_sse(
                        black_box(*el),
                        black_box($default_value),
                    ));
                })
            });
        }

        group.finish();
    };
}

fn bench_i8(c: &mut Criterion) {
    bench_capacity_some!(c, i8, 42_i8, "i8");
}

fn bench_i16(c: &mut Criterion) {
    bench_capacity_some!(c, i16, 1234_i16, "i16");
}

fn bench_i32(c: &mut Criterion) {
    bench_capacity_some!(c, i32, 0x11223344_i32, "i32");
}

fn bench_u8(c: &mut Criterion) {
    bench_capacity_some!(c, u8, 0xFF_u8, "u8");
}

fn bench_u16(c: &mut Criterion) {
    bench_capacity_some!(c, u16, 0xABCD_u16, "u16");
}

fn bench_u32(c: &mut Criterion) {
    bench_capacity_some!(c, u32, 0xDEADBEEF_u32, "u32");
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(std::time::Duration::from_millis(500))
        .measurement_time(std::time::Duration::from_secs(2))
        .sample_size(20);
    targets = bench_i8, bench_i16, bench_i32, bench_u8, bench_u16, bench_u32
}
criterion_main!(benches);
