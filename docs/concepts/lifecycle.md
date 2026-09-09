# Lifecycle and verdicts

Operational lifecycle: `ACTIVE → DEPRECATED → OBSERVING → QUIESCENT → READY → REMOVED → VERIFIED`.

Verdict states are separate and evidence-derived:

- `blocked`: one or more hard blockers are active.
- `insufficient-evidence`: required evidence is missing/stale.
- `observing`: evidence exists but a required window is too short.
- `quiescent`: no hard blocker is observed, but policy/cleanup work remains.
- `ready-with-uncertainty`: readiness threshold is met with explicit residual uncertainty.
- `ready`: all required deterministic policy checks pass without material uncertainty.
- `removed`: lifecycle metadata records removal; pre-removal readiness no longer applies.
- `verified`: post-removal checks meet verification policy.
