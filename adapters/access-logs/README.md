# Access log evidence

The CLI supports common/Nginx-style access lines and structured JSON lines. Requests are matched against the target identifier. Remote/client identifiers are hashed before appearing in normalized evidence, and common-log IP identity is classified as partial rather than authoritative.

```bash
decomproof ingest service:legacy-export --source access-log access.log
```
