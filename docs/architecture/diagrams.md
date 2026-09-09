# Architecture diagrams

## 1. Overall architecture
```mermaid
flowchart LR
  S[Source tree] --> C[Rust core]
  T[Telemetry exports] --> C
  D[Data/event metadata] --> C
  C --> P[retirement.proof.json]
  C --> CLI[CLI]
  P --> API[Optional API]
  P --> UI[Dashboard]
```

## 2. Evidence ingestion
```mermaid
flowchart LR
  A[Adapter/importer] --> N[Normalize]
  N --> R[Redact/hash]
  R --> E[Evidence item]
  E --> H[Raw hash + provenance]
```

## 3. Analysis pipeline
```mermaid
flowchart LR
  T[Target] --> S[Scan/import]
  S --> G[Evidence graph]
  G --> W[Window/freshness checks]
  W --> B[Blockers + uncertainty]
  B --> V[Deterministic verdict]
  V --> P[Proof]
```

## 4. Target lifecycle
```mermaid
stateDiagram-v2
  ACTIVE --> DEPRECATED
  DEPRECATED --> OBSERVING
  OBSERVING --> QUIESCENT
  QUIESCENT --> READY
  READY --> REMOVED
  REMOVED --> VERIFIED
```

## 5. Proof generation
```mermaid
flowchart TD
  E[Sorted evidence] --> D[Evidence digest]
  E --> PE[Policy evaluation]
  PE --> S[Score]
  PE --> R[Rationale]
  D --> P[Proof v1]
  S --> P
  R --> P
```

## 6. Static dependency graph
```mermaid
flowchart TD
  L[legacy-export]
  C[Finance CronJob] -->|schedule.reference / ev_*| L
  O[OpenAPI] -->|contract.public / ev_*| L
  K[Kubernetes] -->|infra.resource / ev_*| L
  SDK[LegacyExportClient] -->|sdk.export / ev_*| L
```

## 7. Runtime consumer mapping
```mermaid
flowchart LR
  Logs[Runtime records] --> ID{Identity quality}
  ID -->|identified| H[Hashed consumer ID]
  ID -->|partial| U[Explicit uncertainty]
  ID -->|unknown| B[Policy blocker]
```

## 8. Temporal evidence flow
```mermaid
flowchart LR
  O[Observations] --> G[Inter-observation gaps]
  G --> P[Conservative cadence]
  P --> W[Recommended window]
  W --> PE[Policy evidence]
```

## 9. Policy evaluation
```mermaid
flowchart TD
  P[Configured policy] --> C{Checks}
  E[Evidence] --> C
  C --> HB[Hard blockers]
  C --> U[Uncertainty]
  C --> PD[Policy decisions]
  HB --> V[Verdict]
  U --> V
  PD --> V
```

## 10. GitHub PR gate
```mermaid
sequenceDiagram
  participant PR
  participant Action
  participant DecomProof
  PR->>Action: target + evidence
  Action->>DecomProof: analyze
  DecomProof-->>Action: proof + exit code
  Action-->>PR: summary / gate
```

## 11. Post-removal verification
```mermaid
flowchart LR
  R[Removed target] --> V[Verification import]
  V --> F[Failures]
  V --> L[Leftovers]
  F --> D{Decision}
  L --> D
  D -->|none| OK[VERIFIED]
  D -->|leftovers| P[PARTIAL CLEANUP]
  D -->|failures| X[REGRESSION DETECTED]
```

## 12. Database ER model
```mermaid
erDiagram
  PROJECT ||--o{ TARGET : owns
  TARGET ||--o{ ANALYSIS_RUN : analyzed
  ANALYSIS_RUN ||--o{ EVIDENCE_ITEM : contains
  ANALYSIS_RUN ||--|| PROOF : produces
  TARGET ||--o{ LIFECYCLE_EVENT : transitions
  TARGET ||--o{ VERIFICATION_RUN : verifies
```

## 13. Deployment architecture
```mermaid
flowchart LR
  Browser --> Web[Next.js dashboard]
  Web --> API[FastAPI]
  API --> PG[(PostgreSQL)]
  API --> Store[Proof artifact store]
  Worker[Optional worker] --> PG
  CLI --> Store
```

## 14. AtlasCommerce demo architecture
```mermaid
flowchart TD
  Customer[External customer] --> Legacy[legacy-export]
  Finance[Finance CronJob] --> Legacy
  Legacy --> DB[(legacy_exports)]
  Recon[Reconciliation worker] --> Events[Event broker]
  SDK[Python SDK] --> Legacy
  OpenAPI[Public contract] --> Legacy
  Infra[Terraform / K8s] --> Legacy
```

## 15. Security trust boundaries
```mermaid
flowchart LR
  subgraph Untrusted inputs
    Files[Repository files]
    Logs[Telemetry exports]
  end
  Files --> Core[Read-only core]
  Logs --> Core
  Core --> Proof[Sanitized proof]
  subgraph Explicit network boundary
    DB[(Read-only DB)]
    Kafka[(Read-only broker)]
  end
  DB -. configured .-> Core
  Kafka -. configured .-> Core
```
