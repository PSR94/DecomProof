# DecomProof API

The optional FastAPI service persists proofs and lifecycle/verification history. It never recomputes readiness; the Rust-generated proof is authoritative.

Development defaults to SQLite. Docker/production examples use PostgreSQL. Apply `migrations/0001_initial.sql` for a production database rather than relying on startup schema creation.

```bash
pip install -e '.[dev]'
uvicorn decomproof_api.main:app --reload
pytest
```
