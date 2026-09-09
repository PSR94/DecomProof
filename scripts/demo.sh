#!/usr/bin/env bash
set -euo pipefail
stage="${1:-all}"
run_stage() {
  local s="$1"
  echo
  echo "=== AtlasCommerce stage ${s} ==="
  set +e
  cargo run -q -p decomproof -- demo --stage "$s"
  local code=$?
  set -e
  if [[ "$s" -le 3 && "$code" -eq 2 ]]; then
    echo "stage ${s}: expected safety gate refusal"
    return 0
  fi
  return "$code"
}
if [[ "$stage" == "all" ]]; then
  for s in 0 1 2 3 4 5 6; do run_stage "$s"; done
else
  run_stage "$stage"
fi
