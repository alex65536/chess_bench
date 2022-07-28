use chess_bench::{
    impls,
    perft::{self, Case},
    Perft,
};
use criterion::{criterion_group, criterion_main, Criterion};

fn do_perft(
    c: &mut Criterion,
    name: &'static str,
    perfter: impl Fn(&Case, &(dyn Perft + 'static)),
) {
    let perfts = impls::all_perft();
    for case in &perft::CASES {
        let mut group = c.benchmark_group(format!("{}/{}", name, case.name));
        for p in &perfts {
            group.bench_function(p.name(), |b| b.iter(|| perfter(case, &**p)));
        }
    }
}

fn perft(c: &mut Criterion) {
    do_perft(c, "perft", Case::run_perft);
}

fn hperft(c: &mut Criterion) {
    do_perft(c, "hperft", Case::run_hperft);
}

criterion_group!(benches, perft, hperft);
criterion_main!(benches);
