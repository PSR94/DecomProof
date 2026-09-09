# Quick start

```bash
git clone https://github.com/PSR94/DecomProof.git
cd DecomProof
make setup
cargo run -p decomproof -- init
cargo run -p decomproof -- analyze service:legacy-export --root examples/atlascommerce/stages/stage-0 --evidence examples/atlascommerce/stages/stage-0/runtime.jsonl
```

Exit code 2 means the safety gate refused removal because the verdict is blocked or evidence is insufficient. `retirement.proof.json` is still written so CI can upload it for inspection.

Use `--json` for machine output. Network integrations are disabled unless configured; the stable v0.1 path is scanning local source and importing sanitized evidence exports.
