use criterion::{Criterion, criterion_group, criterion_main};
use rawr_acm::Acm;

fn bench_simple_allow(c: &mut Criterion) {
    let mut acm = Acm::new();
    acm.allow("action:Get", "resource/path");

    c.bench_function("acm enforce simple allow", |b| {
        b.iter(|| {
            assert!(acm.enforce("action:Get", "resource/path"));
        })
    });
}

fn bench_deny_override(c: &mut Criterion) {
    let mut acm = Acm::new();
    acm.allow("action:*", "resource/*");
    acm.deny("action:Delete", "resource/sensitive");

    c.bench_function("acm enforce deny override", |b| {
        b.iter(|| {
            assert!(!acm.enforce("action:Delete", "resource/sensitive"));
        })
    });
}

fn bench_wildcard_glob(c: &mut Criterion) {
    let mut acm = Acm::new();
    acm.allow("action:*", "resource/*");

    c.bench_function("acm enforce wildcard glob", |b| {
        b.iter(|| {
            assert!(acm.enforce("action:Get", "resource/123"));
        })
    });
}

fn bench_deep_path(c: &mut Criterion) {
    let mut acm = Acm::new();
    acm.allow("action:Get", "a/b/c/d/e/f/g/h/i");

    c.bench_function("acm deep path match", |b| {
        b.iter(|| {
            assert!(acm.enforce("action:Get", "a/b/c/d/e/f/g/h/i"));
        })
    });
}

criterion_group!(
    benches,
    bench_simple_allow,
    bench_deny_override,
    bench_wildcard_glob,
    bench_deep_path
);

criterion_main!(benches);
