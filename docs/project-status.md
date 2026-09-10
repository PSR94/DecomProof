# Project status — v0.1.0 released

## Implemented in the repository

- typed target identifiers and stable fingerprints
- universal evidence records with stable content IDs, raw hashes, timestamps, freshness and observation windows
- deterministic policy, hard-blocker, uncertainty, scoring and verdict pipeline
- fail-conservative behavior for missing required runtime evidence and empty verification evidence
- runtime/external/data observation-window checks and scheduled-cycle checks
- API deprecation/sunset policy metadata for scanned OpenAPI contracts
- conservative temporal cadence inference
- evidence-linked dependency graph model
- workspace scanning for JavaScript/TypeScript, Python, SQL, YAML/JSON-like configuration, OpenAPI, Terraform, Kubernetes, Docker Compose and GitHub Actions references
- offline generic JSONL, access-log, OTLP JSON and Prometheus-text ingestion
- hashed runtime consumer identifiers
- versioned `decomproof/v1` proof schema, JSON/terminal/Markdown/HTML reports, evidence filtering and proof diffing
- persisted lifecycle state with guarded `READY` and verification-controlled `VERIFIED` transitions
- AtlasCommerce staged source/evidence scenario with integration assertions for blocked stage 0 and ready stage 4
- optional live AtlasCommerce service that creates real access-log and PostgreSQL activity
- experimental read-only PostgreSQL exporter and Kafka metadata normalizer
- FastAPI proof/history persistence service and relational reference migration
- Next.js evidence dashboard with requested developer-tool views plus explicit empty/loading/error states
- GitHub composite removal gate, CI/Docker/docs/release workflow definitions, issue templates and PR template
- executable Criterion microbenchmarks plus large-repository and large-telemetry stress scenarios with measured baselines in `docs/performance.md`
- architecture/ADR/concept/security/research/release documentation and SVG branding

## Experimental / partial

- JavaScript/TypeScript and Python scanning is syntax/context-aware heuristic scanning, not full compiler-grade semantic analysis.
- OpenAPI, Terraform, Kubernetes, SQL and GitHub Actions analysis is structural/text-assisted; evidence confidence records this limitation.
- PostgreSQL live inspection is deliberately narrow and read-only; it does not invent read/write recency when query/audit telemetry is unavailable.
- Kafka v0.1 support normalizes exported metadata rather than connecting directly to brokers.
- Post-removal verification evaluates imported failures/leftovers; correlated error-rate and fallback analysis require suitable evidence exports.
- API/dashboard are developer-preview surfaces; authentication/multi-tenancy are not claimed.

## Validation status

The v0.1.0 release was executed on GitHub-hosted runners rather than being accepted from repository inspection alone.

- CI run #66 (`34421870612`) passed Rust formatting, Clippy with warnings denied, Rust workspace tests, the deterministic AtlasCommerce demo, proof-schema validation, API Ruff/pytest, dashboard ESLint/unit/build/Playwright, the production-only npm high-severity audit gate, and secret preflight against the exact tagged release candidate.
- Docker builds run #21 (`34420551630`) passed image builds for API, dashboard and AtlasCommerce; `docker compose config --quiet` also passed in the matrix.
- Release validation run #6 (`34421870540`) passed the README quick-start commands from a clean checkout, the complete clean Criterion micro/stress benchmark suite, and production-dashboard screenshot capture backed by a generated READY proof persisted through the live FastAPI service.
- Four release screenshots (overview, evidence matrix, dependency graph and proof viewer) were visually inspected and contain loaded proof data rather than loading, empty or API-error states.
- Finalize v0.1.0 run #1 (`34422108936`) successfully built x86_64 Linux and macOS CLI archives, collected validated proof/benchmark/screenshot evidence, enforced the release checklist and collision guards, and published the GitHub Release.
- Measured performance/stress results and exact reproduction commands are recorded in `docs/performance.md`.

The public `v0.1.0` tag points to commit `66395c7dcba0be3a7509715c81ebb3a9dc515b7a`. GitHub Release `v0.1.0` is published with Linux and macOS CLI archives, SHA-256 checksum files and a bundled release-evidence archive containing the validated proof, Criterion log and real UI screenshots.

## Roadmap-only

Native Datadog/CloudWatch/Grafana/Snowflake/BigQuery integrations, GitLab CI, Argo/Temporal schedule discovery, service-catalog integrations, feature-flag provider integrations, organization-level campaigns/scorecards, IDE integrations and destructive cleanup automation.
