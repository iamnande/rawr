use criterion::{Criterion, criterion_group, criterion_main};
use rawr_resource_name::ResourceName;
use std::hint::black_box;

const VALID_RESOURCE_NAME: &str =
    "mrn:tycho:opa:sol-belt-1:36UeVtK7fIxhHyD9Dd5gc1XSd77:member/anderson-dawes";

const VALID_RESOURCE_NAME_EMPTY_REGION_AND_ACCOUNT_ID: &str =
    "mrn:tycho:opa:::member/anderson-dawes";

const VALID_RESOURCE_NAME_WITH_MULTI_SEGMENT_RESOURCE_PATH: &str =
    "mrn:tycho:opa:sol-belt-1:36UeVtK7fIxhHyD9Dd5gc1XSd77:station/ceres/bay-12";

fn bench_valid_resource_name(c: &mut Criterion) {
    c.bench_function("parse resource name", |b| {
        b.iter(|| ResourceName::parse(black_box(VALID_RESOURCE_NAME)))
    });
}

fn bench_valid_resource_name_with_empty_region_and_account_id(c: &mut Criterion) {
    c.bench_function(
        "parse resource name with empty region and account id",
        |b| {
            b.iter(|| {
                ResourceName::parse(black_box(VALID_RESOURCE_NAME_EMPTY_REGION_AND_ACCOUNT_ID))
            })
        },
    );
}

fn bench_valid_resource_name_with_multi_segment_resource_path(c: &mut Criterion) {
    c.bench_function(
        "parse resource name with multi-segment resource path",
        |b| {
            b.iter(|| {
                ResourceName::parse(black_box(
                    VALID_RESOURCE_NAME_WITH_MULTI_SEGMENT_RESOURCE_PATH,
                ))
            })
        },
    );
}

criterion_group!(
    benches,
    bench_valid_resource_name,
    bench_valid_resource_name_with_empty_region_and_account_id,
    bench_valid_resource_name_with_multi_segment_resource_path
);
criterion_main!(benches);
