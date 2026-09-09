# AtlasCommerce

AtlasCommerce is DecomProof's deterministic decommission scenario. Each `stages/stage-N` directory is a source/evidence snapshot, not a precomputed verdict. The CLI scans the files and ingests the runtime observations to produce the proof.

- stage-0: legacy export is active through hidden dependencies
- stage-1: deprecated, but consumers remain
- stage-2: external consumer migrated; internal dependencies remain
- stage-3: data becomes quiescent; contracts/infra remain
- stage-4: observation window is sufficient and blockers are cleared
- stage-5: target removed; verification finds cleanup leftovers
- stage-6: removal verified

Run `make demo-stage-0`, `make demo-ready`, and `make demo-verify`.
