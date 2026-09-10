<p align="center">
  <img src="assets/hero.svg" alt="DecomProof: evidence-backed software decommissioning" width="920" />
</p>

# DecomProof

**Prove it’s safe to delete.**

Finding a reference is easy. Proving that no important dependency remains is much harder. DecomProof combines static repository analysis, imported runtime evidence, contracts, data activity, schedules, events, infrastructure, consumer identity, observation windows, uncertainty, and deterministic policy into a reviewable **retirement proof**.

[![CI](https://github.com/PSR94/DecomProof/actions/workflows/ci.yml/badge.svg)](https://github.com/PSR94/DecomProof/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/PSR94/DecomProof?label=release)](https://github.com/PSR94/DecomProof/releases/tag/v0.1.0)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-v0.1.0%20released-success.svg)](docs/project-status.md)

> **Current state:** `v0.1.0` is released and validated. The release tag points to commit `66395c7dcba0be3a7509715c81ebb3a9dc515b7a`. The repository `main` branch also contains post-release documentation updates. See [Project handoff / restart point](#project-handoff--restart-point) before starting new work.

---

## Table of contents

- [What DecomProof does](#what-decomproof-does)
- [What DecomProof does not claim](#what-decomproof-does-not-claim)
- [Why this exists](#why-this-exists)
- [How the proof pipeline works](#how-the-proof-pipeline-works)
- [Safety model](#safety-model)
- [v0.1.0 release](#v010-release)
- [Quick start](#quick-start)
- [CLI reference](#cli-reference)
- [Exit behavior](#exit-behavior)
- [Configuration](#configuration)
- [Target identifiers](#target-identifiers)
- [Evidence sources](#evidence-sources)
- [Observation windows and uncertainty](#observation-windows-and-uncertainty)
- [Dependency graph](#dependency-graph)
- [Retirement proof](#retirement-proof)
- [Lifecycle and post-removal verification](#lifecycle-and-post-removal-verification)
- [AtlasCommerce demo](#atlascommerce-demo)
- [Optional API](#optional-api)
- [Optional dashboard](#optional-dashboard)
- [Docker Compose local stack](#docker-compose-local-stack)
- [GitHub removal gate](#github-removal-gate)
- [Performance baseline](#performance-baseline)
- [Validation evidence](#validation-evidence)
- [Repository layout](#repository-layout)
- [Documentation map](#documentation-map)
- [Security and privacy](#security-and-privacy)
- [Known limitations](#known-limitations)
- [Project handoff / restart point](#project-handoff--restart-point)
- [TODO — remaining work](#todo--remaining-work)
- [Next-session start order](#next-session-start-order)
- [Contributing](#contributing)
- [License](#license)

---

## What DecomProof does

DecomProof answers a narrow but important engineering question:

> **Does the evidence we have satisfy the configured policy for safely retiring this software target?**

It does this by producing a deterministic, machine-readable proof rather than a free-form recommendation.

The v0.1.0 repository includes:

- a Rust evidence/proof core;
- a Rust CLI;
- typed software target identifiers and stable fingerprints;
- a universal evidence model with stable content IDs, raw hashes, timestamps, freshness, observation windows, gaps, provenance, and confidence;
- workspace scanning across supported code/configuration formats;
- offline ingestion for generic JSONL, access logs, OTLP JSON, and Prometheus text;
- hashed runtime consumer identifiers where configured;
- conservative temporal/cadence inference;
- evidence-linked dependency graphs;
- deterministic policy evaluation, hard blockers, explicit uncertainty, scoring, and verdicts;
- a versioned `decomproof/v1` retirement-proof schema;
- JSON, terminal, Markdown, and HTML proof reporting;
- evidence filtering, blocker inspection, graph inspection, history search, and proof diffing;
- lifecycle state with guarded `READY` and verification-controlled `VERIFIED` transitions;
- post-removal verification;
- the staged AtlasCommerce demonstration project;
- an optional read-only PostgreSQL evidence exporter;
- an experimental Kafka metadata normalizer;
- an optional FastAPI proof/history persistence service;
- an optional Next.js developer dashboard;
- Docker/Compose deployment examples;
- a GitHub composite removal gate;
- CI, Docker, release-validation, and release-finalization workflows;
- executable Criterion microbenchmarks and large-repository / large-telemetry stress scenarios;
- architecture decisions, concepts, security, evidence, research, release, and development documentation.

No AI model participates in the readiness verdict.

## What DecomProof does not claim

DecomProof does **not** mathematically prove that a dependency cannot exist. It evaluates whether the evidence collected for a target satisfies a configured retirement policy.

It also does not claim, in v0.1.0, to provide:

- complete compiler-grade semantic understanding for every supported language;
- complete visibility into dynamically constructed references or reflection;
- complete coverage when telemetry is missing, sampled, stale, or too short;
- production-ready authentication or multi-tenancy for the API/dashboard;
- direct SaaS integrations for every observability/data platform;
- automatic production deletion of infrastructure or data objects.

Coverage gaps are represented as uncertainty or blockers rather than silently treated as safety.

See [Proof of absence](docs/concepts/proof-of-absence.md) and [Limitations](docs/concepts/limitations.md).

## Why this exists

A repository search can find a reference, but deletion safety depends on more than static references.

Examples:

- a customer may call an endpoint once per quarter;
- a monthly batch can look inactive during a two-hour observation window;
- a table may still receive writes despite a clean application dependency graph;
- an OpenAPI endpoint may remain public while an internal caller has disappeared;
- a deprecated API may still be exported by an SDK;
- a CronJob, workflow, Terraform resource, Kubernetes object, event consumer, or direct database client may still depend on the target;
- post-removal errors or leftovers may appear only after deletion.

DecomProof treats these as separate evidence claims, each with provenance, timestamps, confidence, and coverage.

<p align="center"><img src="assets/evidence-matrix.svg" alt="Evidence matrix" width="900" /></p>

## How the proof pipeline works

At a high level:

1. **Identify the target** using a typed target ID.
2. **Scan the repository** for supported references.
3. **Import runtime/external evidence** from sanitized offline exports.
4. **Normalize evidence** into a common model.
5. **Evaluate freshness and observation windows**.
6. **Infer temporal patterns conservatively** when suitable timestamps exist.
7. **Construct an evidence-linked dependency graph**.
8. **Apply deterministic policy** and hard blockers.
9. **Record uncertainty** where evidence is incomplete.
10. **Compute the readiness score and verdict**.
11. **Write `retirement.proof.json`**.
12. After removal, **run verification separately** and only then allow the lifecycle to become `VERIFIED`.

<p align="center"><img src="assets/proof-flow.svg" alt="Evidence-linked proof flow" width="900" /></p>

The Rust core owns target identity, evidence normalization, temporal logic, graph construction, policy evaluation, scoring, and proof generation. Network-facing systems are optional shells around that deterministic core.

## Safety model

The design is deliberately fail-conservative.

Important rules include:

- missing required runtime evidence is not interpreted as zero usage;
- a zero count without a sufficient observation window is weak evidence;
- stale evidence creates uncertainty rather than clearance;
- unidentified consumers can block removal;
- active public API exposure can block removal;
- scheduled workloads can require multiple observed cycles;
- insufficient post-removal verification cannot produce `VERIFIED`;
- `READY` requires a generated proof whose verdict permits the transition;
- `VERIFIED` can only be recorded by successful `decomproof verify --record-lifecycle`;
- analysis is read-only by default;
- production deletion is not automated by v0.1.0.

The architectural decisions behind these boundaries are recorded in [`docs/architecture/decisions`](docs/architecture/decisions/).

## v0.1.0 release

The first public developer release is available here:

**[DecomProof v0.1.0](https://github.com/PSR94/DecomProof/releases/tag/v0.1.0)**

Release tag:

```text
v0.1.0 -> 66395c7dcba0be3a7509715c81ebb3a9dc515b7a
```

Published assets include:

- `decomproof-x86_64-unknown-linux-gnu.tar.gz`
- `decomproof-x86_64-unknown-linux-gnu.tar.gz.sha256`
- `decomproof-x86_64-apple-darwin.tar.gz`
- `decomproof-x86_64-apple-darwin.tar.gz.sha256`
- `decomproof-v0.1.0-release-evidence.tar.gz`
- `decomproof-v0.1.0-release-evidence.tar.gz.sha256`

The evidence bundle carries forward validated release evidence such as the generated proof, Criterion benchmark log, and real dashboard screenshots.

Release notes: [`docs/release-notes-v0.1.0.md`](docs/release-notes-v0.1.0.md)

## Quick start

### Requirements

For the stable core/CLI path:

- Git;
- a current stable Rust toolchain;
- `make` for the convenience targets below.

The API/dashboard are optional and have separate Python/Node requirements.

### Build and validate from source

```bash
git clone https://github.com/PSR94/DecomProof.git
cd DecomProof
make setup
make test
make demo
```

The same clean-checkout path was exercised in release validation.

### First analysis

```bash
cargo run -p decomproof -- init
cargo run -p decomproof -- analyze \
  service:legacy-export \
  --root examples/atlascommerce/stages/stage-0 \
  --evidence examples/atlascommerce/stages/stage-0/runtime.jsonl
```

A blocked result is expected for the stage-0 demo. `retirement.proof.json` is still written so the proof can be inspected or uploaded in CI.

Typical terminal output looks like:

```text
DecomProof

Target: service:legacy-export
Removal Readiness: 41 / 100
Verdict: Blocked

Blockers: 4
Uncertainties: 1
```

Use global `--json` for machine-readable CLI output where supported.

### Useful Make targets

```bash
make setup          # fetch Rust dependencies
make dev            # run decomproof doctor
make test           # cargo test --workspace
make lint           # rustfmt check + Clippy with warnings denied
make demo           # full AtlasCommerce demonstration
make demo-stage-0   # blocked stage
make demo-ready     # READY stage
make demo-verify    # intentionally finds a post-removal leftover
make clean          # cargo clean + remove generated retirement.proof.json
```

See [`docs/getting-started/quickstart.md`](docs/getting-started/quickstart.md).

## CLI reference

The v0.1.0 CLI exposes the following commands:

| Command | Purpose |
| --- | --- |
| `decomproof init` | Create a starter `.decomproof.yml`. Refuses to overwrite an existing file. |
| `decomproof doctor` | Load configuration and report the read-only/network-integration state. |
| `decomproof analyze <target>` | Scan source, import evidence, evaluate policy, write a retirement proof, and return the gate status. |
| `decomproof scan <target>` | Run repository scanning only and print normalized scan evidence. |
| `decomproof ingest <target> <file>` | Normalize one supported evidence input source. |
| `decomproof proof <file>` | Render a proof as terminal, JSON, Markdown, or HTML. |
| `decomproof evidence <file>` | Print all proof evidence or filter by signal. |
| `decomproof graph <file>` | Print the evidence-linked dependency graph. |
| `decomproof blockers <file>` | Print the proof blockers. |
| `decomproof verify <target>` | Evaluate post-removal verification evidence. |
| `decomproof status <target>` | Read the persisted lifecycle state. |
| `decomproof lifecycle <target>` | Read/change lifecycle state subject to readiness/verification guards. |
| `decomproof history <target>` | Search Git history for the target identifier. |
| `decomproof diff <old> <new>` | Compare score, verdict, blocker count, and uncertainty count across two proofs. |
| `decomproof config` | Print the loaded effective configuration. |
| `decomproof demo --stage <0..6>` | Execute one AtlasCommerce demo stage. |

Global options include:

```text
--config <path>   configuration file; default .decomproof.yml
--json            machine-readable output where supported
```

### Ingest sources

```text
generic-json
access-log
otel-json
prometheus
```

Examples:

```bash
decomproof scan service:legacy-export --root .
decomproof ingest service:legacy-export --source access-log ./access.jsonl
decomproof ingest service:legacy-export --source otel-json ./traces.json
decomproof ingest service:legacy-export --source prometheus ./metrics.txt
```

### Proof/report commands

```bash
decomproof proof retirement.proof.json --format terminal
decomproof proof retirement.proof.json --format json
decomproof proof retirement.proof.json --format markdown --output retirement.md
decomproof proof retirement.proof.json --format html --output retirement.html

decomproof evidence retirement.proof.json
decomproof evidence retirement.proof.json --signal runtime.http.requests
decomproof graph retirement.proof.json
decomproof blockers retirement.proof.json
decomproof diff previous.proof.json retirement.proof.json
```

## Exit behavior

The CLI intentionally uses exit status as a safety gate.

For `analyze`:

- a `Blocked` verdict exits with code `2`;
- an `InsufficientEvidence` verdict exits with code `2`;
- the proof is still written before the process exits.

For `verify`:

- `regression-detected` exits with code `2`;
- `insufficient-evidence` exits with code `2`;
- only successful verification can record lifecycle `VERIFIED`.

This allows CI to stop deletion work while still preserving the generated artifact for review.

## Configuration

`decomproof init` creates a conservative starter configuration. The repository example currently uses:

```yaml
project:
  name: decomproof

policy:
  runtime_minimum_window_days: 30
  external_minimum_window_days: 45
  evidence_maximum_age_hours: 24
  scheduled_minimum_cycles: 3
  unidentified_consumers_block: true
  public_api_requires_deprecation: true
  api_minimum_sunset_days: 30
  data_zero_writes_days: 30

privacy:
  hash_consumer_ids: true
  redact_regex:
    - '(?i)authorization:.*'
    - '(?i)cookie:.*'
```

These values are policy inputs, not universal safety guarantees. Teams should configure them for the behavior and cadence of the software being retired.

## Target identifiers

Targets are typed strings. Examples include:

```text
service:legacy-export
api:POST:/v1/export
feature:legacy-checkout
flag:old-dashboard
env:ENABLE_OLD_FLOW
job:nightly-export
db-table:legacy_sessions
db-column:users.legacy_id
event:order.created.v1
topic:order-events-v1
package-export:LegacyClient
terraform:aws_lambda_function.legacy_export
k8s:deployment:legacy-export
```

Typed targets produce stable identities/fingerprints and help prevent unrelated references from being merged into one retirement claim.

## Evidence sources

| Family | v0.1 path | What it contributes / boundary |
| --- | --- | --- |
| Static code/config | workspace scanner | JS/TS/Python heuristic scanning; SQL/YAML/OpenAPI/Terraform/K8s/Docker Compose/GitHub Actions context classification |
| Runtime HTTP | access-log / generic JSONL | offline-first usage evidence; consumer IDs can be hashed |
| OpenTelemetry | OTLP JSON spans | runtime evidence; maps `service.name` when present |
| Prometheus | exposition text | snapshot usage evidence; does not invent historical coverage |
| Schedules | static CronJob/workflow + imported cycle evidence | active refs block; absence may require observed cycles |
| Databases | SQL scan + PostgreSQL evidence exporter | exporter is explicit read-only and experimental |
| Events | imported Kafka/event metadata | active consumer evidence can hard-block |
| Contracts | OpenAPI scan | public exposure is blocking in v0.1 policy behavior |
| SDK/package | Python/JS export/reference scan | public exports can be high-severity blockers |
| Infrastructure | Terraform/Kubernetes/Docker Compose scan | analysis only; never deletes resources |
| Verification | post-removal evidence imports | failures/leftovers prevent verified status |

The stable v0.1 path is intentionally offline-first: scan local source and import sanitized evidence exports. Network integrations remain disabled unless explicitly configured.

## Observation windows and uncertainty

Dynamic evidence can carry:

- observation start;
- observation end;
- known gap duration;
- freshness/age;
- consumer identity quality;
- confidence;
- normalized counts/activity.

A zero count without time is not equivalent to “unused.” Policy compares available windows against configured minimums. Stale or incomplete evidence becomes uncertainty rather than a silent pass.

Temporal inference is deliberately conservative. Scheduled-cycle requirements exist because a quiet interval can easily miss weekly, monthly, quarterly, or irregular consumers.

## Dependency graph

Every dependency edge in the proof stores the evidence ID that caused it.

The graph is therefore an **explanation layer over evidence**, not an invented architecture model. Reviewers can trace a blocker or graph edge back to its source evidence record.

## Retirement proof

The machine-readable artifact defaults to:

```text
retirement.proof.json
```

It validates against:

[`schemas/retirement-proof-v1.schema.json`](schemas/retirement-proof-v1.schema.json)

A proof records, among other fields:

- schema/proof version;
- target identity and fingerprint;
- source revision;
- policy context;
- normalized evidence;
- stable evidence IDs and raw hashes;
- timestamps, freshness, observation windows, and gaps;
- confidence and consumer identity quality;
- evidence-linked graph edges;
- blockers;
- uncertainties;
- score;
- verdict and rationale.

Proof format changes are governed by the repository’s compatibility rules: additive v1 changes must preserve semantics; breaking changes require a new schema identifier.

See [`docs/reference/retirement-proof.md`](docs/reference/retirement-proof.md).

## Lifecycle and post-removal verification

Pre-removal readiness and post-removal verification are intentionally separate.

Typical lifecycle work:

```bash
decomproof status service:legacy-export

decomproof lifecycle service:legacy-export \
  --set ready \
  --proof retirement.proof.json \
  --note "approved after evidence review"

decomproof verify service:legacy-export \
  --evidence verification.jsonl \
  --record-lifecycle
```

Important guards:

- entering `READY` requires a generated proof for the same target;
- the proof verdict must be `Ready` or `ReadyWithUncertainty`;
- users cannot manually set `VERIFIED` through the lifecycle command;
- `VERIFIED` is written only after successful verification;
- no verification evidence means DecomProof fails conservatively instead of declaring success.

Default lifecycle state file:

```text
.decomproof/lifecycle.json
```

See [`docs/concepts/lifecycle.md`](docs/concepts/lifecycle.md).

## AtlasCommerce demo

AtlasCommerce is the end-to-end demonstration project for a mature SaaS retirement.

The initial `legacy-export` target remains reachable through multiple evidence families, including examples of:

- a finance CronJob;
- a public OpenAPI path;
- an old SDK/package export;
- database activity;
- an event consumer;
- deployment/infrastructure references;
- runtime usage from an identified external consumer.

The staged fixtures remove dependencies through actual source/evidence changes; the score is not hardcoded.

Run:

```bash
make demo-stage-0   # blocked
make demo-ready     # sufficient evidence / READY demonstration
make demo-verify    # intentionally demonstrates a post-removal leftover
```

The CLI also supports stages `0..6`:

```bash
cargo run -p decomproof -- demo --stage 0
cargo run -p decomproof -- demo --stage 4
cargo run -p decomproof -- demo --stage 5
cargo run -p decomproof -- demo --stage 6
```

See:

- [`examples/atlascommerce`](examples/atlascommerce)
- [`docs/demo-script.md`](docs/demo-script.md)
- [`apps/atlascommerce`](apps/atlascommerce)

`apps/atlascommerce` is an optional live service whose deprecated endpoint can produce access-log and PostgreSQL activity for evidence experiments.

## Optional API

The FastAPI service persists proofs and lifecycle/verification history. It **does not recompute readiness**; the Rust-generated proof remains authoritative.

Development defaults to SQLite. Production/Docker examples use PostgreSQL.

```bash
cd apps/api
pip install -e '.[dev]'
uvicorn decomproof_api.main:app --reload
```

Run API tests:

```bash
pytest
```

For a production database, apply:

```text
apps/api/migrations/0001_initial.sql
```

rather than relying on development startup schema creation.

API documentation: [`apps/api/README.md`](apps/api/README.md)

## Optional dashboard

The Next.js dashboard is a developer-tool UI for proofs persisted by the API. It never fabricates proof data: when the API is empty or unavailable it renders explicit empty/loading/error states.

```bash
cd apps/dashboard
npm install
DECOMPROOF_API_URL=http://localhost:8000/api/v1 npm run dev
```

Available dashboard scripts:

```bash
npm run dev
npm run build
npm run start
npm run lint
npm run test
npm run e2e
```

The v0.1 preview includes routes/views for:

- overview;
- projects;
- candidates;
- candidate detail;
- evidence matrix;
- blockers;
- uncertainty;
- runtime;
- dependency graph;
- observation timeline;
- schedules;
- data;
- events;
- contracts;
- infrastructure;
- retirement plan;
- proof viewer;
- verification;
- history;
- configuration;
- AtlasCommerce;
- explicit loading/error/empty states.

Four release screenshots — overview, evidence matrix, dependency graph, and proof viewer — were generated against a real READY proof served through the live API and visually checked before release.

Dashboard documentation: [`apps/dashboard/README.md`](apps/dashboard/README.md)

> **Known reproducibility TODO:** the dashboard currently has no committed `package-lock.json`; the validated workflows therefore still use `npm install`. Committing the runner-generated lockfile and switching deterministic paths to `npm ci` is the first pending maintenance task. See [TODO](#todo--remaining-work).

## Docker Compose local stack

`docker-compose.yml` defines the complete local topology:

- PostgreSQL 17;
- Redis 7;
- MinIO;
- AtlasCommerce;
- DecomProof API;
- DecomProof dashboard.

Start the stack with:

```bash
docker compose up --build
```

Default exposed local ports:

```text
AtlasCommerce  http://localhost:8080
API            http://localhost:8000
Dashboard      http://localhost:3000
```

The Compose configuration uses local-development credentials only. Do not copy those credentials into production environments.

Deployment examples also include Kubernetes and Terraform material under [`infrastructure`](infrastructure/). The Terraform example intentionally avoids inventing unrelated cloud resources.

## GitHub removal gate

A composite GitHub Action is included for using DecomProof as a deletion/removal gate.

Example:

```yaml
- uses: PSR94/DecomProof/packages/github-action@main
  with:
    target: service:legacy-export
    root: .
    evidence: .decomproof/runtime.jsonl
```

The action writes a GitHub Step Summary and returns the DecomProof safety-gate exit code. CI should also upload `retirement.proof.json` with `actions/upload-artifact` so reviewers retain the evidence behind a refusal or approval.

A repository example workflow is available at:

[`/.github/workflows/removal-gate-example.yml`](.github/workflows/removal-gate-example.yml)

## Performance baseline

DecomProof includes Criterion benchmarks for deterministic core paths.

Reproduce with:

```bash
cargo bench -p decomproof-core --bench core -- --noplot
```

Representative measured middle estimates from the recorded GitHub-hosted baseline are:

| Scenario | Scale | Middle estimate |
| --- | ---: | ---: |
| `policy_evaluate` | representative evidence set | `729.55 ns` |
| `proof_serialization` | representative proof | `2.1326 µs` |
| `temporal_pattern_1k` | 1,000 timestamps | `7.5787 µs` |
| `graph_build_1k` | 1,000 dependency entries | `367.15 µs` |
| `graph_build_10k_stress` | 10,000 dependency entries | `6.3530 ms` |
| `source_scan_500_files` | 500 TypeScript files | `1.8019 ms` |
| `source_scan_5k_files_stress` | 5,000 TypeScript files | `21.178 ms` |
| `jsonl_ingest_100k_stress` | 100,000 telemetry records | `260.47 ms` |

These are engineering baselines for regression detection, **not production SLOs or guarantees**. They isolate core DecomProof work and exclude network latency, external APIs, database server latency, browser rendering, container startup, and dependency-download time.

Full intervals, runner details, fixture definitions, and interpretation: [`docs/performance.md`](docs/performance.md)

## Validation evidence

The v0.1.0 release was not accepted from code inspection alone. It was executed on GitHub-hosted runners.

### Exact release-candidate validation

- **CI #66** — run `34421870612`
  - `cargo fmt --all --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - deterministic AtlasCommerce demo
  - proof JSON-Schema validation
  - API Ruff + pytest
  - dashboard ESLint + unit tests + production build + Playwright E2E
  - production-only npm audit at high severity
  - secret preflight

- **Docker builds #21** — run `34420551630`
  - API image build
  - dashboard image build
  - AtlasCommerce image build
  - `docker compose config --quiet`

- **Release validation #6** — run `34421870540`
  - clean-checkout README/Make commands
  - clean Criterion micro/stress suite
  - generated real READY proof
  - live FastAPI persistence
  - production dashboard startup
  - loaded-state Playwright screenshot capture

- **Finalize v0.1.0 #1** — run `34422108936`
  - Linux x86_64 release archive
  - macOS x86_64 release archive
  - checksums
  - release checklist enforcement
  - evidence collection
  - existing tag/release collision guard
  - successful GitHub Release publication

Release checklist: [`docs/release-checklist.md`](docs/release-checklist.md)

Project status: [`docs/project-status.md`](docs/project-status.md)

## Repository layout

```text
DecomProof/
├── crates/
│   ├── core/                 # deterministic evidence/proof engine
│   └── cli/                  # decomproof command-line application
├── apps/
│   ├── api/                  # optional FastAPI persistence service
│   ├── dashboard/            # optional Next.js developer dashboard
│   └── atlascommerce/        # optional live demo application
├── adapters/
│   ├── access-logs/
│   ├── kafka/
│   ├── otel/
│   ├── postgres/
│   └── prometheus/
├── examples/
│   └── atlascommerce/        # staged decommission scenario
├── schemas/                  # retirement-proof JSON Schema
├── packages/
│   └── github-action/        # composite removal gate
├── infrastructure/           # Compose/Kubernetes/Terraform examples
├── scripts/                  # demo, proof validation, secret checks, helpers
├── docs/                     # architecture, concepts, evidence, research, releases
├── assets/                   # README/project artwork
├── .github/workflows/        # CI, Docker, docs, validation, release workflows
├── .decomproof.yml           # example policy/privacy configuration
├── docker-compose.yml        # complete local developer topology
├── Makefile
├── ROADMAP.md
├── SECURITY.md
├── CONTRIBUTING.md
├── CHANGELOG.md
└── README.md                 # primary overview + maintainer handoff
```

## Documentation map

You should not need to read every document before resuming work. Use this map only when a task needs deeper detail.

### Start/status/release

- [`README.md`](README.md) — **primary restart point and authoritative TODO for the next working session**
- [`docs/project-status.md`](docs/project-status.md) — released feature/status snapshot
- [`docs/release-checklist.md`](docs/release-checklist.md) — v0.1.0 release gates and evidence IDs
- [`docs/release-notes-v0.1.0.md`](docs/release-notes-v0.1.0.md) — v0.1.0 release notes
- [`docs/performance.md`](docs/performance.md) — benchmark/stress methodology and results
- [`CHANGELOG.md`](CHANGELOG.md) — changelog; **currently needs post-release cleanup, see TODO**
- [`ROADMAP.md`](ROADMAP.md) — historical/current roadmap; **needs reconciliation after v0.1.0, see TODO**

### Getting started and demo

- [`docs/getting-started/quickstart.md`](docs/getting-started/quickstart.md)
- [`docs/demo-script.md`](docs/demo-script.md)
- [`apps/api/README.md`](apps/api/README.md)
- [`apps/dashboard/README.md`](apps/dashboard/README.md)
- [`infrastructure/README.md`](infrastructure/README.md)

### Core concepts

- [`docs/concepts/proof-of-absence.md`](docs/concepts/proof-of-absence.md)
- [`docs/concepts/scoring.md`](docs/concepts/scoring.md)
- [`docs/concepts/lifecycle.md`](docs/concepts/lifecycle.md)
- [`docs/concepts/limitations.md`](docs/concepts/limitations.md)

### Architecture

- [`docs/architecture/diagrams.md`](docs/architecture/diagrams.md)
- [`docs/architecture/decisions/0001-rust-core.md`](docs/architecture/decisions/0001-rust-core.md)
- [`docs/architecture/decisions/0002-evidence-first-verdicts.md`](docs/architecture/decisions/0002-evidence-first-verdicts.md)
- [`docs/architecture/decisions/0003-proof-format.md`](docs/architecture/decisions/0003-proof-format.md)
- [`docs/architecture/decisions/0004-fail-conservatively.md`](docs/architecture/decisions/0004-fail-conservatively.md)
- [`docs/architecture/decisions/0005-observation-windows.md`](docs/architecture/decisions/0005-observation-windows.md)
- [`docs/architecture/decisions/0006-static-plus-runtime-analysis.md`](docs/architecture/decisions/0006-static-plus-runtime-analysis.md)
- [`docs/architecture/decisions/0007-read-only-default.md`](docs/architecture/decisions/0007-read-only-default.md)
- [`docs/architecture/decisions/0008-optional-ai.md`](docs/architecture/decisions/0008-optional-ai.md)

### Proof reference, evidence, adapters, policy, research, development

- [`docs/reference/retirement-proof.md`](docs/reference/retirement-proof.md)
- [`docs/evidence`](docs/evidence/)
- [`docs/adapters`](docs/adapters/)
- [`docs/policies`](docs/policies/)
- [`docs/research`](docs/research/)
- [`docs/development`](docs/development/)
- [`adapters`](adapters/)

### Project policies

- [`SECURITY.md`](SECURITY.md)
- [`CONTRIBUTING.md`](CONTRIBUTING.md)
- [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md)
- [`GOVERNANCE.md`](GOVERNANCE.md)
- [`SUPPORT.md`](SUPPORT.md)

## Security and privacy

Security principles for v0.1 include:

- read-only analysis by default;
- no automatic production resource deletion;
- explicit configuration for database/message-broker connections;
- least-privilege, read-only credentials;
- no plaintext authorization headers, cookies, or API keys in normalized evidence;
- consumer-ID hashing when a pseudonym is sufficient;
- imported-file validation and practical integration limits;
- avoidance of unsafe shell interpolation for user-controlled target strings;
- network integrations disabled until explicitly configured;
- appropriate access control for generated proofs because they may contain operational metadata.

The generic evidence ingester hashes supplied consumer IDs, but arbitrary exporter `extra` values are not guaranteed to be fully sanitized. Redact sensitive payloads before import.

Report vulnerabilities privately through GitHub Security Advisories rather than public issues.

See [`SECURITY.md`](SECURITY.md).

## Known limitations

These are current product boundaries, not hidden defects:

- JavaScript/TypeScript and Python scanning is syntax/context-aware heuristic analysis rather than compiler-grade semantic analysis.
- OpenAPI, Terraform, Kubernetes, SQL, and GitHub Actions analysis is structural/text-assisted.
- Dynamic reflection, unsupported languages, generated identifiers, and manual dependencies can remain invisible.
- Missing/sampled telemetry can leave rare consumers undiscovered.
- PostgreSQL live inspection is deliberately narrow and read-only; it does not invent read/write recency where query/audit telemetry is unavailable.
- Kafka v0.1 normalizes exported metadata rather than connecting directly to brokers.
- Post-removal verification evaluates imported failures/leftovers; correlated error-rate/fallback analysis requires suitable evidence exports.
- Prometheus snapshot ingestion does not invent a historical observation window.
- The optional API and dashboard are developer-preview surfaces; authentication and multi-tenancy are not claimed.
- Production deletion remains outside the v0.1 safety boundary.

---

# Project handoff / restart point

> **Maintainer handoff date:** **2026-09-09 (America/New_York)**  
> **Release state:** **v0.1.0 published**  
> **Release commit:** `66395c7dcba0be3a7509715c81ebb3a9dc515b7a`  
> **Primary rule for the next session:** start with the TODO below; do **not** re-audit the whole repository unless a TODO item specifically requires it.

This section records where the project stopped so a future working session can resume directly from this README.

## What has been completed so far

### Product/core

- Implemented the Rust deterministic evidence/proof core.
- Implemented typed targets and stable target fingerprints.
- Implemented universal normalized evidence with stable IDs/raw hashes/provenance/timestamps/confidence.
- Implemented observation-window, freshness, gap, scheduled-cycle, public-API, data-write, and unidentified-consumer policy checks.
- Implemented hard blockers, explicit uncertainty, readiness scoring, and deterministic verdicts.
- Implemented fail-conservative behavior for missing runtime evidence and missing verification evidence.
- Implemented conservative temporal cadence inference.
- Implemented evidence-linked dependency graph construction.
- Implemented proof generation and the `decomproof/v1` schema.
- Implemented terminal/JSON/Markdown/HTML reports, evidence filtering, graph/blocker inspection, Git history lookup, and proof diffing.
- Implemented guarded lifecycle state and separate post-removal verification.

### Scanning/evidence

- Added JS/TS and Python context-aware heuristic scanning.
- Added SQL, YAML/JSON-like config, OpenAPI, Terraform, Kubernetes, Docker Compose, and GitHub Actions reference classification.
- Added generic JSONL ingestion.
- Added access-log ingestion.
- Added OTLP JSON ingestion.
- Added Prometheus text ingestion.
- Added consumer hashing/privacy behavior.
- Added experimental read-only PostgreSQL inspection/export.
- Added Kafka exported-metadata normalization.

### Demonstration/integration

- Built the staged AtlasCommerce retirement scenario.
- Added blocked and READY fixture stages based on actual changing evidence/source, not hardcoded scores.
- Added post-removal leftover/regression demonstration.
- Added optional live AtlasCommerce service generating access-log/database evidence.

### API/dashboard

- Built the optional FastAPI persistence/history service.
- Added relational migration material.
- Built the Next.js developer dashboard and the requested evidence/readiness views.
- Added explicit loading, empty, and API-error states.
- Added dashboard unit and Playwright E2E coverage.
- Added release screenshot capture against generated proof data persisted through a live API.

### CI/release engineering

- Added secret-preflight checks.
- Added Rust formatting, Clippy-with-warnings-denied, workspace-test gates.
- Added API Ruff/pytest gates.
- Added dashboard lint/unit/build/E2E/production-dependency-audit gates.
- Added Docker image matrix and Compose validation.
- Added fresh-checkout README-command release validation.
- Added Criterion benchmark release validation.
- Added large stress fixtures for 10k dependency edges, 5k repository files, and 100k JSONL telemetry records.
- Added live release screenshot validation.
- Added atomic release finalization that builds both Linux and macOS artifacts before publication.
- Added collision guards so finalization refuses to overwrite an existing `v0.1.0` tag/release.
- Published `v0.1.0` with archives, checksums, and a release-evidence bundle.

### Important validation lesson preserved from the release process

The first screenshot evidence attempt technically passed Playwright but visually contained a loading state. That evidence was **rejected**, the test was strengthened to wait for route-specific loaded content, and release validation was rerun. The final screenshot gate then passed and the four images were visually inspected. This is why release evidence must be checked for semantic validity, not merely a green process exit.

A later stress-benchmark change also failed the Rust formatting gate. The runner-formatted result was applied and the candidate was rerun through formatting, Clippy, tests, benchmarks, and release validation rather than relaxing the gate.

### Final validated runs

Use these IDs instead of reconstructing the old sequence of intermediate CI attempts:

```text
CI #66                 34421870612   PASS
Docker builds #21      34420551630   PASS
Release validation #6  34421870540   PASS
Finalize v0.1.0 #1     34422108936   PASS
```

Earlier intermediate runs were useful during debugging but are **not** the authoritative release evidence.

---

# TODO — remaining work

This is the **authoritative restart backlog as of 2026-09-09 (America/New_York)**. Work can resume directly from here later.

## P0 — repository hygiene and reproducibility

- [ ] **Commit the dashboard dependency lockfile.** A runner-generated `package-lock.json` existed during validation, but it could not be written back through the connector used during the release session. Generate/verify the exact lockfile from `apps/dashboard/package.json`, commit it, and review the diff carefully.
- [ ] **Switch deterministic dashboard installs from `npm install` to `npm ci`** after the lockfile is committed. Review at least CI, release validation, dashboard Docker build, and any other scripted build path before changing them.
- [ ] **Rerun the full dashboard gate after switching to `npm ci`:** lint, unit tests, production build, Playwright E2E, and production `npm audit --omit=dev --audit-level=high`.
- [ ] **Review development-only npm audit findings.** The release production-only high-severity audit passed, but ordinary install output showed vulnerabilities in the full dependency graph during the release session. Identify whether they are only dev/tooling dependencies and upgrade/remediate without weakening the production gate.
- [ ] **Fix `CHANGELOG.md`.** It still labels `0.1.0` as `Unreleased`; change it to the actual released version/date and add a new `Unreleased` section for future development.
- [ ] **Reconcile `ROADMAP.md`.** Some items listed under the old v0.1 roadmap are now completed. Move completed work out of the pending roadmap and make the next milestones explicit.
- [ ] **Reconcile release documentation run references.** `docs/release-notes-v0.1.0.md` and `docs/performance.md` preserve measurements/references from earlier successful release-validation passes (#59/#5 in parts of the documentation), while the final authoritative exact-candidate evidence is CI #66 / Release validation #6 / Finalize #1. Keep historical benchmark provenance where required, but make the distinction explicit and consistent.
- [ ] **Check documentation links/status after this README rewrite** with the docs workflow and fix any broken anchors or stale wording discovered by that validation.

## P1 — deepen analysis correctness and coverage

- [ ] Deepen JavaScript/TypeScript analysis toward compiler/AST/semantic resolution where practical.
- [ ] Deepen Python analysis beyond current syntax/context-aware heuristics.
- [ ] Add stronger structured analyzers/tests for OpenAPI, Terraform, Kubernetes, SQL, configuration, and GitHub Actions where text-assisted classification can produce ambiguity.
- [ ] Expand golden fixtures to cover difficult false-negative/false-positive cases for every analyzer.
- [ ] Continue enforcing the contribution rule that evidence interpretation changes include both positive and conservative-failure tests.
- [ ] Improve detection/representation of generated identifiers, reflection, dynamic imports, runtime registration, and other references that static heuristics can miss.

## P1 — runtime/data/event adapters

- [ ] Finish broader read-only PostgreSQL inspection while preserving the rule that unavailable query/audit history must not be invented.
- [ ] Improve PostgreSQL evidence around read/write recency when trustworthy telemetry is actually available.
- [ ] Move Kafka beyond exported-metadata normalization toward an explicitly configured, least-privilege read-only metadata collector if/when the security model is ready.
- [ ] Expand runtime correlation for rare/scheduled consumers and observation-window gaps.
- [ ] Improve post-removal verification correlation for error-rate changes, fallbacks, and related runtime regressions when sufficient exported evidence exists.

## P2 — API/dashboard hardening

- [ ] Add an explicit authentication design before claiming production API/dashboard readiness.
- [ ] Add authorization/access-control boundaries for persisted proofs and operational metadata.
- [ ] Add multi-tenancy only after the data/security model is documented and tested.
- [ ] Add production deployment hardening guidance rather than relying on local-development defaults.
- [ ] Review retention, redaction, and access controls for proof/history storage.
- [ ] Continue improving dashboard UX only when it does not invent or obscure evidence state.

## P2 — integrations and ecosystem roadmap

- [ ] Native Datadog adapter.
- [ ] Native CloudWatch adapter.
- [ ] Grafana integration.
- [ ] Snowflake evidence integration.
- [ ] BigQuery evidence integration.
- [ ] GitLab CI integration/removal gate.
- [ ] Argo schedule/workflow discovery.
- [ ] Temporal schedule/workflow discovery.
- [ ] Service-catalog / Backstage integration.
- [ ] Feature-flag provider integrations.
- [ ] Cloud-cost evidence where it can support prioritization without affecting safety semantics incorrectly.
- [ ] Organization-level decommission campaigns.
- [ ] Organization-level scorecards/reporting.
- [ ] IDE/editor integrations.

## P3 — future product boundaries requiring explicit design decisions

- [ ] Decide whether additional platform release artifacts are needed beyond the current x86_64 Linux and macOS release packages.
- [ ] Consider richer service/dependency inventory workflows without turning inferred topology into unsupported evidence.
- [ ] If destructive cleanup automation is ever introduced, require a new ADR/security design and keep it separate from the current read-only evidence engine. **Do not treat automatic deletion as implied roadmap completion.**
- [ ] Preserve the rule that optional AI assistance must never become the authoritative readiness verdict unless a future explicit architecture decision changes that boundary.

## Things that are already done — do not redo from scratch

- [x] v0.1.0 core/CLI implementation
- [x] proof schema and proof validator
- [x] staged AtlasCommerce scenario
- [x] API developer preview
- [x] dashboard developer preview
- [x] CI gates
- [x] Docker matrix and Compose validation
- [x] production dashboard dependency audit gate
- [x] secret scan
- [x] fresh-clone README validation
- [x] Criterion microbenchmarks
- [x] 10k graph stress benchmark
- [x] 5k-file repository scan stress benchmark
- [x] 100k-record JSONL telemetry stress benchmark
- [x] real loaded-state dashboard release screenshots
- [x] visual screenshot inspection
- [x] Linux release archive
- [x] macOS release archive
- [x] SHA-256 release checksums
- [x] bundled release evidence
- [x] atomic finalization/collision guard
- [x] public `v0.1.0` GitHub Release
- [x] root README converted into this complete project/restart document

---

# Next-session start order

When returning to this project later, **do not begin by rereading the whole repository**. Start here:

1. Read only the [TODO](#todo--remaining-work) section above.
2. Start with **P0 dashboard reproducibility**: create/verify `apps/dashboard/package-lock.json` and switch controlled build paths to `npm ci`.
3. Run the complete affected dashboard/CI validation; do not weaken the production audit or E2E gates to get green.
4. Clean the stale release-era documentation: `CHANGELOG.md`, `ROADMAP.md`, and validation-run references.
5. Commit those maintenance changes separately from feature work.
6. Then choose **one** P1 stream:
   - semantic scanner depth, or
   - runtime/database/event adapter depth.
7. For any evidence/policy change, add positive **and conservative-failure** tests before considering it complete.
8. Keep all new unfinished work reflected in this TODO so this README remains the restart point.

Useful health commands when resuming:

```bash
make setup
make lint
make test
make demo
cargo bench -p decomproof-core --bench core -- --noplot
```

For dashboard work:

```bash
cd apps/dashboard
npm install          # replace with npm ci after the lockfile TODO is completed
npm run lint
npm run test
npm run build
npm run e2e
npm audit --omit=dev --audit-level=high
```

For API work:

```bash
cd apps/api
pip install -e '.[dev]'
ruff check decomproof_api tests
pytest -q
```

---

## Contributing

DecomProof treats safety claims as code.

Changes that affect evidence interpretation, blockers, observation windows, score, lifecycle safety, or verdicts should include tests for both:

- the intended positive behavior; and
- conservative failure when required evidence is absent, stale, ambiguous, or insufficient.

Development baseline:

```bash
make setup
make test
make lint
make demo
```

Additional contribution rules:

- keep network integrations read-only unless a future ADR explicitly changes the boundary;
- never add a missing-evidence fallback that means “clear”;
- evidence adapters must document provenance, coverage limitations, timestamps, and redaction behavior;
- proof-format breaking changes require a new schema identifier;
- intentional serialization changes should include golden fixtures;
- PRs should explain how false negatives are avoided and what uncertainty remains.

See [`CONTRIBUTING.md`](CONTRIBUTING.md), [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md), and [`GOVERNANCE.md`](GOVERNANCE.md).

## License

Apache License 2.0. See [`LICENSE`](LICENSE).

---

**DecomProof principle:** absence of evidence is not automatically evidence of absence. Removal readiness is a deterministic policy decision over explicit, reviewable evidence — and uncertainty stays visible.
