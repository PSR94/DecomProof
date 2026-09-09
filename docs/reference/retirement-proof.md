# `retirement.proof.json`

Schema: `decomproof/v1`, defined by `schemas/retirement-proof-v1.schema.json`.

The proof records target identity and fingerprint, source revision, generation time, policy version, observation requirements, signal counts, blockers, explicit uncertainties, score, verdict rationale, policy decisions, evidence items, and a digest of the deterministically ordered evidence list.

## Compatibility

Within schema v1, fields are additive only where consumers can safely ignore them. Breaking representation changes require a new schema identifier and a new schema file. The CLI may read older v1 proofs but must never silently reinterpret a v1 field with different semantics.
