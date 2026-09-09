# Limitations

DecomProof fails conservatively around evidence it cannot observe. Important limitations include dynamic code and reflection, unknown external clients, missing or sampled telemetry, incomplete log retention, annual/rare workloads, encrypted traffic, unsupported languages, external SaaS integrations, direct database clients, legacy systems and manual processes.

The v0.1 workspace scanner is intentionally heuristic for JavaScript/TypeScript and Python. It classifies findings and evidence quality but is not yet a full semantic compiler frontend. Live PostgreSQL and Kafka collection are read-only integration surfaces in progress; offline evidence import is the stable path for v0.1.
