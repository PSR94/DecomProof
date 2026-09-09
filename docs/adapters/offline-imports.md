# Offline evidence imports

Offline import is the stable v0.1 integration boundary. Export telemetry or metadata from the system of record, redact sensitive content, then provide JSONL rows with `signal`, `source`, `observed_at`, optional window fields, `count`, `active` and optional consumer identity fields.

This avoids requiring a SaaS account and makes evidence fixtures reproducible in CI. Live adapters should normalize into the same structure.
