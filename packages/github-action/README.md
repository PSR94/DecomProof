# GitHub Action

The source action runs the Rust CLI from this repository on the caller's checkout, writes a concise GitHub Step Summary and returns the CLI safety-gate exit code. The caller should upload `retirement.proof.json` with `actions/upload-artifact`.

```yaml
- uses: PSR94/DecomProof/packages/github-action@main
  id: decomproof
  with:
    target: service:legacy-export
    evidence: .decomproof/runtime.jsonl
- uses: actions/upload-artifact@v4
  if: always()
  with:
    name: retirement-proof
    path: ${{ steps.decomproof.outputs.proof-path }}
```

`fail-on: never` is provided for advisory rollout. The proof still records the authoritative deterministic verdict.
