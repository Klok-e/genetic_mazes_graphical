use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("iterator vs for loop");
    for inp in [(300, 300)].iter() {
        group.bench_with_input(
            BenchmarkId::new("loop", format!("{:?}", inp)),
            inp,
            |b, inp| {
                b.iter(|| {
                    let mut v = (0, 0);
                    for x in 0..inp.0 {
                        for y in 0..inp.1 {
                            v = (x, y);
                        }
                    }
                    v
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("iterator", format!("{:?}", inp)),
            inp,
            |b, inp| {
                b.iter(|| {
                    let mut v = (0, 0);
                    for (x, y) in (0..inp.0).flat_map(|x| (0..inp.0).map(move |y| (x, y))) {
                        v = (x, y);
                    }
                    v
                })
            },
        );
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
