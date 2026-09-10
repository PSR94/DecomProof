# v0.1.0 release checklist

- [x] `cargo fmt --all --check`
- [x] `cargo clippy --workspace --all-targets -- -D warnings`
- [x] `cargo test --workspace`
- [x] run AtlasCommerce blocked stage and inspect proof
- [x] run ready stage and inspect proof
- [x] validate proof against v1 JSON Schema
- [x] run post-removal verification fixtures
- [x] validate Docker/dashboard/API builds
- [x] validate README commands from a fresh clone
- [x] generate real UI screenshots with Playwright and visually inspect loaded output
- [x] run large-repository and large-telemetry Criterion stress scenarios
- [x] production dashboard dependency audit at high severity
- [x] secret scan
- [x] create `v0.1.0` tag and GitHub Release only after finalization builds succeed

## Evidence

- CI #66 (`34421870612`): exact-release-candidate Rust fmt/Clippy/tests, proof/demo/schema validation, API Ruff/pytest, dashboard ESLint/unit/build/Playwright/production audit, secret preflight.
- Docker builds #21 (`34420551630`): API, dashboard and AtlasCommerce Docker images plus Compose config validation.
- Release validation #6 (`34421870540`): exact-release-candidate clean-checkout README commands, clean Criterion micro/stress benchmarks and seeded live-API Playwright screenshot capture.
- Finalize v0.1.0 #1 (`34422108936`): Linux and macOS release archives built successfully; checklist/evidence/collision guards passed; GitHub Release creation passed.
- `docs/performance.md`: measured benchmark intervals and stress fixture definitions.

The `v0.1.0` tag points to validated release commit `66395c7dcba0be3a7509715c81ebb3a9dc515b7a`. GitHub Release `v0.1.0` is published with platform archives, SHA-256 checksum files and a bundled copy of the validated proof/benchmark/screenshot evidence.
