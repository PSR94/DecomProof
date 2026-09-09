# v0.1.0 release checklist

- [ ] `cargo fmt --all --check`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `cargo test --workspace`
- [ ] run AtlasCommerce blocked stage and inspect proof
- [ ] run ready stage and inspect proof
- [ ] validate proof against v1 JSON Schema
- [ ] run post-removal verification fixtures
- [ ] validate Docker/dashboard/API builds
- [ ] validate README commands from a fresh clone
- [ ] generate real UI screenshots with Playwright
- [ ] secret scan
- [ ] tag only after all required checks pass

Unchecked items are intentional: this file is the release gate and must not pretend validation occurred.
