# PostgreSQL adapter (experimental)

`inspect.py` opens an explicit `READ ONLY` transaction, applies a statement timeout, parameterizes metadata lookups and never performs destructive operations. It reports existence, row count, view references and foreign-key metadata. It does **not** invent read/write recency: recent writes require database audit/query telemetry or an application-owned timestamp that can be interpreted safely.

Install `psycopg[binary]` and pipe the JSONL output into DecomProof. Prefer a dedicated least-privilege role.
