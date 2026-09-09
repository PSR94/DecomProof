# Testing and validation

`make test` runs Rust tests; `make lint` runs rustfmt/clippy. The CI workflow additionally runs the full AtlasCommerce staged demo, validates both golden and generated proofs against JSON Schema, tests/lints the FastAPI service, builds/tests the dashboard and runs a high-confidence secret preflight.

`cargo bench -p decomproof-core` contains executable benchmark scenarios for scanning, graph building, policy evaluation, temporal processing and proof serialization. No benchmark numbers are committed because results are environment-dependent and must be measured rather than invented.
