#!/usr/bin/env bash
set -euo pipefail
stage="${1:-0}"
root="examples/atlascommerce/stages/stage-${stage}"
case "$stage" in
  5) cargo run -q -p decomproof -- verify service:legacy-export --evidence "$root/cleanup-leftovers.jsonl" ;;
  6) cargo run -q -p decomproof -- verify service:legacy-export --evidence "$root/runtime.jsonl" ;;
  *) cargo run -q -p decomproof -- analyze service:legacy-export --root "$root" --evidence "$root/runtime.jsonl" --output "retirement.proof.json" ;;
esac
