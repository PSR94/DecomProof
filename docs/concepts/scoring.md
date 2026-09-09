# Deterministic readiness scoring

The score starts at 100 and applies deterministic penalties: critical hard blockers -25, high -15, medium -8, low -3; high uncertainty -10, other uncertainty -5; failed policy decisions -8. Values clamp to 0–100.

The score **never overrides hard rules**. Any hard blocker yields `blocked` regardless of score. Missing required runtime evidence yields `insufficient-evidence`. An inadequate runtime observation window yields `observing`. `ready` requires score 100 and no recorded uncertainty; scores at least 90 without hard/policy failures may be `ready-with-uncertainty`.

The scoring formula is intentionally simple in v0.1 so every point is explainable. Future versions may add policy-configurable weights without changing the hard-blocker invariant.
