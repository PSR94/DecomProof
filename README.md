<p align="center">
  <img src="assets/hero.svg" alt="DecomProof: evidence-backed software decommissioning" width="920" />
</p>

# DecomProof

**Prove it’s safe to delete.**

Finding a reference is easy. Proving there are no important dependencies left is hard. DecomProof gathers the evidence you need before deleting software.

[![CI](https://github.com/PSR94/DecomProof/actions/workflows/ci.yml/badge.svg)](https://github.com/PSR94/DecomProof/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![version](https://img.shields.io/badge/version-0.1.0--dev-5b667a.svg)](docs/project-status.md)

Static analysis tells you what references something. Telemetry tells you what used it recently. DecomProof combines code, runtime, contracts, data, schedules, events, infrastructure and consumer evidence to decide whether the available evidence supports removal.

## Immediate example

```bash
decomproof init
decomproof analyze service:legacy-export \
  --root . \
  --evidence ./runtime.jsonl
```

A blocked target is explicit:

```text
DecomProof

Target: service:legacy-export
Removal Readiness: 41 / 100
Verdict: Blocked

Blockers: 4
Uncertainties: 1
```

The machine-readable artifact is `retirement.proof.json` and validates against [`schemas/retirement-proof-v1.schema.json`](schemas/retirement-proof-v1.schema.json).

## Why grep is not enough

A static reference scan cannot tell you about a customer calling an endpoint once per quarter. Zero traffic over two hours cannot rule out a monthly batch. A clean application graph cannot tell you that a table is still receiving writes. A deprecated OpenAPI endpoint may still be present in the current SDK. DecomProof treats every one of these as a separate evidence claim with provenance, freshness, window and confidence.

<p align="center"><img src="assets/evidence-matrix.svg" alt="Evidence matrix" width="900" /></p>

## The proof, not a guess

A proof records:

- target identity and fingerprint
- source revision and policy version
- normalized evidence with stable IDs and raw hashes
- observation windows, gaps and freshness
- consumer identity quality
- dependency edges that cite evidence
- hard blockers and explicit uncertainty
- deterministic readiness score and verdict rationale

DecomProof does not mathematically prove nonexistence. It establishes whether collected evidence satisfies a configured retirement policy. See [Proof of absence](docs/concepts/proof-of-absence.md).

## Quick start

Requirements: Rust 1.80+ for the core/CLI. Optional API/dashboard dependencies are documented separately.

```bash
git clone https://github.com/PSR94/DecomProof.git
cd DecomProof
make setup
make test
make demo
```

To inspect one stage directly:

```bash
make demo-stage-0   # expected safety refusal: hidden dependencies exist
make demo-ready     # blockers removed and observation window sufficient
make demo-verify    # intentionally finds a post-removal leftover
```

## Target identifiers

The parser keeps identifiers simple and stable:

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

## Evidence sources

| Family | v0.1 path | Notes |
|---|---|---|
| Static code/config | workspace scanner | JS/TS/Python heuristic; SQL/YAML/OpenAPI/Terraform/K8s/GitHub Actions classified by context |
| Runtime HTTP | generic JSONL / access-log-shaped evidence | offline first; consumer IDs hashed |
| Schedules | static CronJob/workflow/config scan | active references are hard blockers |
| Databases | SQL scan + imported PostgreSQL evidence | live read-only adapter is experimental |
| Events | imported event/Kafka metadata evidence | consumer activity can hard-block |
| Contracts | OpenAPI scan | public exposure is blocking until removed under v0.1 policy |
| SDK/package | Python/JS export/reference scan | current public export is a high blocker |
| Infrastructure | Terraform + Kubernetes scan | read-only; never deletes resources |
| Verification | imported post-removal checks | failures or leftovers prevent `verified` |

See [project status](docs/project-status.md) for exact support boundaries.

## Observation windows

A zero count without time is weak evidence. Dynamic evidence can carry:

```json
{
  "window_start": "2026-07-20T00:00:00Z",
  "window_end": "2026-09-09T20:56:00Z",
  "count": 0,
  "active": false
}
```

Policy checks compare available windows with configured minimums and mark stale dynamic evidence as uncertainty rather than silently treating it as clear.

## Dependency graph

<p align="center"><img src="assets/proof-flow.svg" alt="Evidence linked proof flow" width="900" /></p>

Every graph edge stores the evidence ID that caused it. The graph is an explanation layer over evidence, not an invented architecture model.

## AtlasCommerce demo

AtlasCommerce models a mature SaaS retirement. The initial `legacy-export` target looks quiet from ordinary application code but remains reachable through a finance CronJob, a public OpenAPI path, an old SDK export, database writes, an event consumer and deployment resources. Runtime evidence also contains an identified external consumer.

The staged demo removes those dependencies through real source/evidence snapshots. Scores are not hardcoded; each stage executes the normal scanner, ingestion and policy engine.

See [`examples/atlascommerce`](examples/atlascommerce) and [`docs/demo-script.md`](docs/demo-script.md).

## GitHub removal gate

A Docker action is provided under [`packages/github-action`](packages/github-action):

```yaml
- uses: PSR94/DecomProof/packages/github-action@main
  with:
    target: service:legacy-export
    root: .
    evidence: .decomproof/runtime.jsonl
```

The action emits the generated proof as an artifact in the repository's example workflow. A blocked or insufficient-evidence verdict exits non-zero.

## Post-removal verification

Pre-removal readiness and post-removal verification are different questions:

```bash
decomproof verify service:legacy-export \
  --evidence verification.jsonl
```

Verification evidence can report requests to removed endpoints, scheduler failures, correlated failures and cleanup leftovers. With no verification evidence, DecomProof fails conservatively instead of declaring success.

## Architecture

The Rust core owns target identity, evidence normalization, temporal logic, graph construction, policy evaluation, scoring and proof generation. Network-facing systems are optional shells around that deterministic core.

- [Architecture overview and diagrams](docs/architecture/diagrams.md)
- [ADRs](docs/architecture/decisions/)
- [Retirement proof reference](docs/reference/retirement-proof.md)
- [Scoring](docs/concepts/scoring.md)
- [Lifecycle](docs/concepts/lifecycle.md)

## Security and privacy

Analysis is read-only by default. DecomProof does not automatically delete infrastructure or database objects. Imported consumer identifiers are hashed by the generic ingester; sensitive source systems should provide least-privilege credentials and pre-redacted exports. See [`SECURITY.md`](SECURITY.md).

## Limitations

Dynamic reflection, unsupported languages, sampled or missing telemetry, rare/annual jobs, direct external database clients and human/manual dependencies can remain invisible. DecomProof surfaces these coverage gaps as uncertainty where evidence allows, but cannot observe what no configured source exposes. See [limitations](docs/concepts/limitations.md).

## Roadmap and contributing

The goal for 0.1 is a trustworthy offline-first research/developer tool before broad SaaS integrations. See [`ROADMAP.md`](ROADMAP.md) and [`CONTRIBUTING.md`](CONTRIBUTING.md).

Apache-2.0 licensed. No AI model participates in the readiness verdict.
