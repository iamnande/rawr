# quality

> we have the best authorization.. because of jail.

for full releases or performance-critical work, please add the `test/quality`
label.

## philosophy

> the things you think about determine the **_quality_** of your mind. your soul
> takes on the **_color_** of your thoughts.

this project treats performance as a correctness concern.

we prefer:

- predictable over peak performance.

- explicit costs over implicit convenience.

- mechanical enforcement over convention.

when performance trade-offs are required, they must be made consciously, documented, and reviewed.

## goals

> don't let perfection get in the way of greatness.

1. core API paths must perform zero heap allocations.

2. steady-state CPU cost must scale linearly with input size.

3. no per-call syscalls in the hot path.

## allocations

> in terms of heap... we have no heap.

allocation tests for zero-allocation APIs must pass on every PR.

allocation tests assert zero allocations per call unless an approved 
exception applies.

any new allocation in a hot-path is considered a regression unless explicitly
documented and approved.

### allowed allocations

1. initialization paths.

2. test helpers, benchmark and profiling setup.

3. feature-gated debug functionality (must be documented, approved, and remain 
off by default).

approved exceptions:

| component | number | description |
| --------- | ------ | ----------- |
| `acm`     | `001`  | single allocation for the evaluation stack, if and only if the number of segments exceeds 10 |

### techniques

- caller-provided buffers.

- borrowed data (e.g. `&[u8]`, `&str`) over owned types.

- `SmallVec` when unavoidable (must be approved and documented above).

- `SmallVec` capacity must cover >90% of observed cases.

## benchmarks

> on a scale of Franklin to Ricky Bobby, how fast are you?

full benchmarks are not run on every PR.

full benchmarks must be run for performance-sensitive changes and before release.

- `criterion` is used for benchmarks.

- benchmarks live in `${component}/benches/*.rs` files.

- results must be measured and reviewed against the most recent baseline before
merge.

- benchmarks must include: 
    - representative inputs
    
    - warm-up time
    
    - stable compiler flags (e.g. `--release`, fixed features)

### how-to benchmark

for ACMs:
```sh
$ git checkout main
$ BASELINE=true make benchmark-acm
$ git checkout ${feature_branch}
$ make benchmark-acm
```

for resource names:
```sh
$ git checkout main
$ BASELINE=true make benchmark-resource-name
$ git checkout ${feature_branch}
$ make benchmark-resource-name
```

## CPU profiling

CPU profiling is diagnostic, not a gate.

- `samply` is used for CPU profiling.

- profiles live in `${component}/examples/profile_XXX.rs` files.

- profiles must be taken with the `--release` profile flag.

CPU profiling is expected for:

- new algorithms

- refactors in hot paths

- investigating benchmark regressions


### how-to CPU profile

to CPU profile ACMs:
```sh
$ make cpu-profile-acm
```

to CPU profile resource names:
```sh
$ make cpu-profile-resource-name
```

## memory profiling

> your call is important to us, please stay on the line.
> we are consulting a council of wise llamas before proceeding to summon an
> army of highly trained squirrels to assist you.

## regression protection

- zero-allocation APIs must have allocation-counting tests.

- performance-sensitive changes must include benchmarks or profiling notes.

- regressions must be explained, not ignored.
