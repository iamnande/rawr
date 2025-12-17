use rawr_resource_name::ResourceName;
use std::env;

const DEFAULT_PROFILE_CATEGORY: &str = "baseline";

fn main() {
    let args: Vec<String> = env::args().collect();
    let category = args
        .get(1)
        .map(|s| s.as_str())
        .unwrap_or(DEFAULT_PROFILE_CATEGORY);

    match category {
        "baseline" => {
            println!("running baseline ResourceName profile benchmark...");
            bench_baseline_resource_name();
        }
        _ => {
            eprintln!("unrecognized profile category: {}", category);
            eprintln!("available categories: baseline, detailed, surgical, rounded");
            std::process::exit(1);
        }
    }
}

fn bench_baseline_resource_name() {
    const ITERATIONS: usize = 1_000_000;
    const RESOURCE_NAME: &str =
        "mrn:tycho:opa:sol-belt-1:36UeVtK7fIxhHyD9Dd5gc1XSd77:member/anderson-dawes";

    for _idx in 0..ITERATIONS {
        ResourceName::parse(RESOURCE_NAME).unwrap();
    }
}
