use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;

use rawr_acm::Acm;

struct TestPolicy<'a> {
    effect: &'a str,
    action: &'a str,
    resource: &'a str,
}

struct TestCase<'a> {
    name: &'a str,
    enforce_action: &'a str,
    enforce_resource: &'a str,
    policies: Vec<TestPolicy<'a>>,
}

fn bench(c: &mut Criterion) {
    let test_cases = vec![
        TestCase {
            name: "simple_allow",
            enforce_action: "action:Get",
            enforce_resource: "resource/path",
            policies: vec![TestPolicy {
                effect: "allow",
                action: "action:Get",
                resource: "resource/path",
            }],
        },
        TestCase {
            name: "deny_override",
            enforce_action: "action:Delete",
            enforce_resource: "resource/sensitive",
            policies: vec![
                TestPolicy {
                    effect: "allow",
                    action: "action:*",
                    resource: "resource/*",
                },
                TestPolicy {
                    effect: "deny",
                    action: "action:Delete",
                    resource: "resource/sensitive",
                },
            ],
        },
        TestCase {
            name: "wildcard_glob",
            enforce_action: "action:Get",
            enforce_resource: "resource/123",
            policies: vec![TestPolicy {
                effect: "allow",
                action: "action:*",
                resource: "resource/*",
            }],
        },
        TestCase {
            name: "no_matching_policy",
            enforce_action: "action:Update",
            enforce_resource: "resource/unknown",
            policies: vec![TestPolicy {
                effect: "allow",
                action: "action:Get",
                resource: "resource/path",
            }],
        },
        TestCase {
            name: "deep_path",
            enforce_action: "action:Get",
            enforce_resource: "a/b/c/d/e/f/g/h/i",
            policies: vec![TestPolicy {
                effect: "allow",
                action: "action:Get",
                resource: "a/b/c/d/e/f/g/h/i",
            }],
        },
    ];

    let mut group = c.benchmark_group("Acm::enforce");
    for case in test_cases {
        let mut acm = Acm::new();
        for policy in &case.policies {
            match policy.effect {
                "allow" => acm.allow(policy.action, policy.resource),
                "deny" => acm.deny(policy.action, policy.resource),
                _ => panic!("Unknown effect: {}", policy.effect),
            }
        }

        group.throughput(Throughput::Elements(1));
        group.bench_function(case.name, |b| {
            b.iter(|| {
                acm.enforce(
                    black_box(case.enforce_action),
                    black_box(case.enforce_resource),
                );
            })
        });
    }
    group.finish();
}

criterion_group!(benches, bench);

criterion_main!(benches);
