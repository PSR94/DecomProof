# Project status — v0.1.0 release candidate

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

The release candidate has been executed on GitHub-hosted runners rather than being accepted from repository inspection alone.

- CI run #59 (`34421384468`) passed Rust formatting, Clippy with warnings denied, Rust workspace tests, the deterministic AtlasCommerce demo, proof-schema validation, API Ruff/pytest, dashboard ESLint/unit/build/Playwright, the production-only npm high-severity audit gate, and secret preflight.
- Docker builds run #21 (`34420551630`) passed image builds for API, dashboard and AtlasCommerce; `docker compose config --quiet` also passed in the matrix.
- Release validation run #5 (`34421384497`) passed the README quick-start commands from a clean checkout, the complete Criterion micro/stress benchmark suite, and production-dashboard screenshot capture backed by a generated READY proof persisted through the live FastAPI service.
- Four release screenshots (overview, evidence matrix, dependency graph and proof viewer) were visually inspected after the run and contain loaded proof data rather than loading, empty or API-error states.
- Measured performance/stress results and exact reproduction commands are recorded in `docs/performance.md`. The raw benchmark log and screenshot bundle remain attached to the Release validation workflow run and are intended to be copied into the GitHub Release assets.

A `v0.1.0` tag and GitHub Release are still intentionally absent at this point. The atomic finalization workflow builds Linux and macOS CLI artifacts first and is triggered only after the release checklist is complete.

## Roadmap-only

Native Datadog/CloudWatch/Grafana/Snowflake/BigQuery integrations, GitLab CI, Argo/Temporal schedule discovery, service-catalog integrations, feature-flag provider integrations, organization-level campaigns/scorecards, IDE integrations and destructive cleanup automation.
