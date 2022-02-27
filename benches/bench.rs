use benchmark::{mega_match, phf};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

pub fn bench_all(c: &mut Criterion) {
    let input = [
        "AD",                                                                  // first item in the match
        "PK",           // middle item in the match
        "J07BX03",      // last item in the match
        "MISSING_ITEM", // default branch in the match
        "PI",           // a middle miss in the match
        "1119349007",   // second last with a first char matches
        "AH",           // early miss
        "AHHHHHHHHHHHHHHHHHHHHHHHA-Alan-Patridige-knowing-me-knowing-you-AHA", // early miss
    ];

    let mut group = c.benchmark_group("match_vs_phf");
    for s in input.iter() {
        // mega_match
        group.bench_with_input(BenchmarkId::new("mega_match", s), s, |b, s| {
            b.iter(|| mega_match(s))
        });

        // phf
        group.bench_with_input(BenchmarkId::new("phf", s), s, |b, s| b.iter(|| phf(s)));
    }
}

criterion_group!(benches, bench_all);
criterion_main!(benches);
