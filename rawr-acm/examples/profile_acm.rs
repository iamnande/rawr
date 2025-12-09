use rawr_acm::Acm;
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
            println!("running baseline ACM profile benchmark...");
            bench_baseline_acm();
        }
        _ => {
            eprintln!("unrecognized profile category: {}", category);
            eprintln!("available categories: baseline, detailed, surgical, rounded");
            std::process::exit(1);
        }
    }
}

fn bench_baseline_acm() {
    const ITERATIONS: usize = 1_000_000;
    const USER_COUNT: usize = 1000;

    let mut acm = Acm::new();
    acm.allow("identity:*", "users/*");
    acm.allow("identity:CreateUser*", "users/*");
    acm.deny("identity:DeleteUser", "users/admin");

    // generate the user_ids up-front to avoid a !format in the hot path of the
    // profiled benchmark
    let user_ids: Vec<String> = (0..USER_COUNT).map(|i| format!("users/{}", i)).collect();

    // simulate a few seconds of traffic
    for idx in 0..ITERATIONS {
        acm.enforce("identity:CreateUser", &user_ids[idx % USER_COUNT]);
    }
}
