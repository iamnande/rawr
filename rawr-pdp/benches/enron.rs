use criterion::{Criterion, criterion_group, criterion_main};
use rawr_pdp::{Decider, RawrDecider};
use rawr_pip::JsonPolicyLoader;
use std::hint::black_box;
use std::str::FromStr;
use svix_ksuid::Ksuid;

const ACCOUNT_KSUID: &str = "365SmM3pnQ5Flegvk5nA7leI5KL";
const PRINCIPAL_KSUID: &str = "35zt2LD8MyWdngVgdr4Qaqcpesb";

fn criterion_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let loader = JsonPolicyLoader::default();
    let decider = RawrDecider::new(loader);
    let account_ksuid = Ksuid::from_str(ACCOUNT_KSUID).unwrap();
    let principal_ksuid = Ksuid::from_str(PRINCIPAL_KSUID).unwrap();

    c.bench_function("authorize", |b| {
        b.iter(|| {
            let result = rt
                .block_on(decider.decide(
                    black_box(&account_ksuid),
                    black_box(&principal_ksuid),
                    black_box("michael:Bolten"),
                    black_box("that/no/talent/*/clown"),
                ))
                .unwrap();
            assert!(!result);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
