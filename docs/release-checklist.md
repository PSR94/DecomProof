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
- [ ] create `v0.1.0` tag and GitHub Release only after finalization builds succeed

## Evidence

- CI #59 (`34421384468`): Rust fmt/Clippy/tests, proof/demo/schema validation, API Ruff/pytest, dashboard ESLint/unit/build/Playwright/production audit, secret preflight.
- Docker builds #21 (`34420551630`): API, dashboard and AtlasCommerce Docker images plus Compose config validation.
- Release validation #5 (`34421384497`): clean-checkout README commands, Criterion micro/stress benchmarks and seeded live-API Playwright screenshot capture.
- `docs/performance.md`: measured benchmark intervals and stress fixture definitions.

The final item remains unchecked until the atomic finalization workflow has built both Linux and macOS CLI artifacts and successfully created the GitHub Release. This checklist must not claim the tag exists before it does.
