use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};
use fyrstikker::fyrstikk_tal_kombinasjonar;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("fyrstikker");

    group.sampling_mode(SamplingMode::Flat).sample_size(10);

    group.bench_function("fyrstikker 33", |b| {
        b.iter(|| fyrstikk_tal_kombinasjonar(33))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
