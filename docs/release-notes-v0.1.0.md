# DecomProof v0.1.0

DecomProof v0.1.0 is the first public developer release of the repository's decommission-evidence workflow: collect evidence, evaluate deterministic policy, explain blockers and uncertainty, produce a versioned retirement proof, and verify the post-removal state.

## Highlights

- Rust core and CLI with typed targets, stable fingerprints, universal evidence records, deterministic scoring/verdicts, evidence-linked graphs, temporal inference, proof diffing and lifecycle state.
- Conservative retirement behavior: missing required runtime evidence and incomplete verification remain blocking or uncertain instead of being guessed away.
- Source/configuration scanning across the repository's supported JavaScript/TypeScript, Python, SQL, YAML/JSON-like, OpenAPI, Terraform, Kubernetes, Docker Compose and GitHub Actions patterns.
- Offline runtime/evidence ingestion for generic JSONL, access logs, OTLP JSON and Prometheus text, with hashed consumer identifiers where configured.
- Versioned `decomproof/v1` proof artifacts plus JSON, terminal, Markdown and HTML reporting.
- AtlasCommerce staged sample that demonstrates a blocked candidate, a READY candidate after sufficient evidence, and post-removal verification.
- FastAPI proof/history persistence service and Next.js evidence dashboard.
- GitHub CI, Docker validation, a composite removal gate and release-evidence workflows.

## Release validation

The release candidate was exercised on GitHub-hosted runners. CI #59 (`34421384468`) passed Rust formatting, Clippy with warnings denied, workspace tests, demo/schema validation, API Ruff/pytest, dashboard ESLint/unit/build/Playwright, production npm audit and secret preflight. Docker builds #21 (`34420551630`) passed API, dashboard and AtlasCommerce images plus Compose configuration. Release validation #5 (`34421384497`) passed the README commands from a clean checkout, Criterion benchmark/stress scenarios and real dashboard screenshot capture from a generated READY proof served by the live API.

Measured performance baselines are documented in `docs/performance.md`. The release assets include Linux and macOS CLI binaries with SHA-256 checksum files. The release finalization workflow also carries forward the raw benchmark log and validated UI screenshot evidence when available.

## Scope and limitations

This release is intentionally conservative about unsupported evidence. JavaScript/TypeScript and Python scanning is heuristic rather than compiler-grade semantic analysis; several infrastructure/configuration analyzers are structural/text-assisted. PostgreSQL inspection is narrow and read-only. Kafka support normalizes exported metadata rather than connecting directly to brokers. The API and dashboard are developer-preview surfaces and do not claim production authentication or multi-tenancy.

Native vendor integrations, broader service-catalog/flag providers, organization-level campaigns and destructive cleanup automation remain roadmap work rather than v0.1.0 claims.
