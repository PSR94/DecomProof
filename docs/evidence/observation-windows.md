# Observation windows and freshness

Dynamic evidence is meaningful only over time. A window stores start, end and known gap seconds. Policy compares the available duration with a minimum. Evidence age is separately compared with a maximum freshness threshold.

A long but stale window can therefore fail freshness; a fresh but short window can remain `observing`. Gaps should be surfaced by adapters rather than hidden in aggregate counts.
