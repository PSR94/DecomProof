# Proof of absence

DecomProof does **not** mathematically prove nonexistence. It gathers evidence sufficient to support a decommission decision under an explicit policy.

## Evidence classes

- **Known-known dependencies** are directly represented in source, schedules, contracts, databases, events or infrastructure.
- **Observable external dependencies** are inferred from telemetry and should carry an observation window and consumer identity quality.
- **Unknown/unobservable consumers** remain uncertainty. Missing telemetry is not equivalent to zero traffic.

## Windows matter

A zero count has meaning only with a time interval and coverage statement. A 24-hour clean window cannot rule out a monthly consumer. DecomProof therefore stores requested/available windows, gaps and freshness and can remain in `observing` or `insufficient-evidence` even when all observed counts are zero.

## Confidence is not certainty

Confidence describes the quality of a particular evidence item. The verdict is deterministic policy evaluation over evidence, blockers and uncertainty; it is not an AI confidence score.
