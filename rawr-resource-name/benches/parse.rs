use criterion::{Criterion, criterion_group, criterion_main};
use rawr_resource_name::ResourceName;
use std::hint::black_box;

const VALID_RESOURCE_NAME: &str =
    "mrn:tycho:opa:sol-belt-1:36UeVtK7fIxhHyD9Dd5gc1XSd77:member/anderson-dawes";

const VALID_RESOURCE_NAME_EMPTY_REGION_AND_ACCOUNT_ID: &str =
    "mrn:tycho:opa:::member/anderson-dawes";

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse resource name", |b| {
        b.iter(|| ResourceName::parse(black_box(VALID_RESOURCE_NAME)))
    });

    c.bench_function(
        "parse resource name with empty region and account id",
        |b| {
            b.iter(|| {
                ResourceName::parse(black_box(VALID_RESOURCE_NAME_EMPTY_REGION_AND_ACCOUNT_ID))
            })
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
