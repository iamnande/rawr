use criterion::{Criterion, criterion_group, criterion_main};
use rawr_pdp::{Decider, RawrDecider};
use rawr_pip::JsonPolicyLoader;
use std::hint::black_box;
use std::str::FromStr;
use svix_ksuid::Ksuid;

const PRINCIPAL_KSUID: &str = "35zt2LD8MyWdngVgdr4Qaqcpesb";

fn criterion_benchmark(c: &mut Criterion) {
    let loader = JsonPolicyLoader::default();
    let decider = RawrDecider::new(Box::new(loader));
    let principal_ksuid = Ksuid::from_str(PRINCIPAL_KSUID).unwrap();

    c.bench_function("authorize", |b| {
        b.iter(|| {
            let result = decider
                .decide(
                    black_box(&principal_ksuid),
                    black_box("michael:Bolten"),
                    black_box("that/no/talent/*/clown"),
                )
                .unwrap();
            assert!(!result);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
