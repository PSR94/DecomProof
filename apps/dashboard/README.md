# DecomProof dashboard

A developer-tool UI for proofs persisted by the optional FastAPI service. The dashboard never fabricates proof data: when the API is empty or unavailable it renders explicit empty/error states.

```bash
npm install
DECOMPROOF_API_URL=http://localhost:8000/api/v1 npm run dev
```

The current preview includes overview/projects/candidates, candidate detail, evidence matrix, blockers, uncertainty, runtime, dependency graph, observation timeline, schedules, data, events, contracts, infrastructure, retirement plan, proof viewer, verification, history, configuration and AtlasCommerce routes, plus loading/error/empty states.
