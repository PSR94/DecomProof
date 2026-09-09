# Project status — v0.1.0 development

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
- executable Criterion benchmark scenarios without committed benchmark claims
- architecture/ADR/concept/security/research/release documentation and SVG branding

## Experimental / partial

- JavaScript/TypeScript and Python scanning is syntax/context-aware heuristic scanning, not full compiler-grade semantic analysis.
- OpenAPI, Terraform, Kubernetes, SQL and GitHub Actions analysis is structural/text-assisted; evidence confidence records this limitation.
- PostgreSQL live inspection is deliberately narrow and read-only; it does not invent read/write recency when query/audit telemetry is unavailable.
- Kafka v0.1 support normalizes exported metadata rather than connecting directly to brokers.
- Post-removal verification evaluates imported failures/leftovers; correlated error-rate and fallback analysis require suitable evidence exports.
- API/dashboard are developer-preview surfaces; authentication/multi-tenancy are not claimed.

## Validation status

The repository contains Rust unit/integration tests, API tests, dashboard tests, schema validation, demo checks, Docker build workflows and CI definitions. In this build session, GitHub did not create workflow runs for connector-originated pushes or the draft validation PR, and the execution sandbox could not clone GitHub because outbound DNS was unavailable. Therefore **no claim is made that CI, Docker builds, frontend/API runtime checks or benchmarks passed in this session**.

Real UI screenshots are intentionally not committed until a successful runnable dashboard session can generate them with Playwright. No GitHub Release or `v0.1.0` tag is created until the release checklist is actually satisfied.

## Roadmap-only

Native Datadog/CloudWatch/Grafana/Snowflake/BigQuery integrations, GitLab CI, Argo/Temporal schedule discovery, service-catalog integrations, feature-flag provider integrations, organization-level campaigns/scorecards, IDE integrations and destructive cleanup automation.
