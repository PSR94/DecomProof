# Contributing to DecomProof

DecomProof treats safety claims as code. Contributions that affect evidence interpretation, blockers, observation windows, score or verdicts must include tests showing both positive and conservative-failure behavior.

## Development

```bash
make setup
make test
make lint
make demo
```

Keep network integrations read-only unless a future ADR explicitly changes that boundary. Never add a missing-evidence fallback that means “clear”. Evidence adapters must document provenance, coverage limitations, timestamps and redaction behavior.

## Changes to the proof format

Additive v1 changes must preserve existing semantics. Breaking changes require a new schema identifier. Include golden fixtures when a serialization change is intentional.

## Pull requests

Explain the evidence claim being introduced, how false negatives are avoided, what uncertainty remains, and which tests exercise failure behavior. Small focused commits are preferred.
