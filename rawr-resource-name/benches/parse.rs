use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use std::{collections::HashMap, hint::black_box};

use rawr_resource_name::ResourceName;

const VALID_RESOURCE_NAME: &str =
    "mrn:tycho:opa:sol-belt-1:36UeVtK7fIxhHyD9Dd5gc1XSd77:member/anderson-dawes";

const VALID_RESOURCE_NAME_EMPTY_REGION_AND_ACCOUNT_ID: &str =
    "mrn:tycho:opa:::member/anderson-dawes";

const VALID_RESOURCE_NAME_WITH_MULTI_SEGMENT_RESOURCE_PATH: &str =
    "mrn:tycho:opa:sol-belt-1:36UeVtK7fIxhHyD9Dd5gc1XSd77:station/ceres/bay-12";

fn bench(c: &mut Criterion) {
    // TODO(nick): make this a map so we can have human friendly names...
    let test_cases: HashMap<&str, &str> = HashMap::from([
        ("valid_resource_name", VALID_RESOURCE_NAME),
        (
            "valid_resource_name_empty_region_and_account_id",
            VALID_RESOURCE_NAME_EMPTY_REGION_AND_ACCOUNT_ID,
        ),
        (
            "valid_resource_name_with_multi_segment_resource_path",
            VALID_RESOURCE_NAME_WITH_MULTI_SEGMENT_RESOURCE_PATH,
        ),
    ]);

    let mut group = c.benchmark_group("ResourceName::parse");
    for (name, case) in test_cases {
        group.throughput(Throughput::Elements(1));
        group.bench_function(name, |b| b.iter(|| ResourceName::parse(black_box(case))));
    }
    group.finish();
}

criterion_group!(benches, bench,);
criterion_main!(benches);
