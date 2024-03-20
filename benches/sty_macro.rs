use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sty::{sty, style::red};

fn marcro_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Styling vs Println");

    let text = "Hello, World!";
    group.bench_function("sty macro", |b| {
        b.iter(|| sty!(black_box(text), [red]));
    });

    group.bench_function("format macro", |b| {
        b.iter(|| format!("{}", black_box(text)));
    });

    group.finish();
}

criterion_group!(benches, marcro_benchmark);
criterion_main!(benches);
