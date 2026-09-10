# Performance and stress validation

DecomProof v0.1.0 includes executable Criterion benchmarks for core deterministic analysis paths. The numbers below are measurements from GitHub Actions Release validation run #5 (`34421384497`) on 2026-09-10. They are engineering baselines, not production SLOs or guarantees.

## Reproduce

```bash
cargo bench -p decomproof-core --bench core -- --noplot
```

The release-validation workflow runs the same command on a GitHub-hosted Ubuntu 24.04 runner. The recorded run used `rustc 1.98.1 (48a229cea 2026-09-01)`, x86_64 Linux, with optimized benchmark builds. Future runs clear `target/criterion` first so cached comparison state cannot contaminate baseline reporting.

## Measured intervals

Criterion reports a three-value confidence interval for each benchmark. The middle value is shown separately for quick comparison.

| Scenario | Fixture scale | Criterion interval | Middle estimate |
| --- | ---: | ---: | ---: |
| `policy_evaluate` | one representative evidence set | 726.59–733.40 ns | 729.55 ns |
| `proof_serialization` | one representative retirement proof | 2.1131–2.1570 µs | 2.1326 µs |
| `temporal_pattern_1k` | 1,000 timestamps | 7.5704–7.5898 µs | 7.5787 µs |
| `graph_build_1k` | 1,000 evidence-backed dependency entries | 362.76–372.48 µs | 367.15 µs |
| `graph_build_10k_stress` | 10,000 evidence-backed dependency entries | 6.2889–6.4237 ms | 6.3530 ms |
| `source_scan_500_files` | 500 TypeScript fixture files | 1.7999–1.8044 ms | 1.8019 ms |
| `source_scan_5k_files_stress` | 5,000 TypeScript fixture files | 20.966–21.394 ms | 21.178 ms |
| `jsonl_ingest_100k_stress` | 100,000 telemetry JSONL records | 259.23–261.88 ms | 260.47 ms |

## What the stress scenarios cover

`graph_build_10k_stress` exercises construction of an evidence-linked dependency graph with ten thousand dependency claims. `source_scan_5k_files_stress` creates a synthetic 5,000-file TypeScript workspace containing references to the target and measures source scanning. `jsonl_ingest_100k_stress` generates 100,000 normalized JSONL telemetry records and measures offline ingestion into the universal evidence model.

These fixtures deliberately isolate DecomProof core work. They do not include network latency, external observability APIs, PostgreSQL server latency, browser rendering, container startup, or CI dependency download time. No latency or throughput claim is made for those paths here.

## Interpretation

The measurements demonstrate that the v0.1.0 core can execute the included large-repository and large-telemetry fixtures on a standard hosted CI runner without failures or timeouts. They should be treated as a reproducible baseline for regression detection rather than as a promise about arbitrary production repositories or telemetry volumes.

The raw Criterion log is retained as the `criterion-benchmark-log` artifact of Release validation run #5. That run emitted absolute timing intervals successfully; cached Criterion comparison metadata also produced non-fatal missing-baseline diagnostics. The workflow now removes cached `target/criterion` data before benchmarking so subsequent runs start with clean comparison state.
