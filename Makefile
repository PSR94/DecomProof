SHELL := /bin/bash

.PHONY: setup dev test lint demo demo-stage-0 demo-ready demo-verify clean
setup:
	cargo fetch
	@echo "Rust dependencies fetched. Optional dashboard/API setup is documented in docs/getting-started/quickstart.md"

dev:
	cargo run -p decomproof -- doctor

test:
	cargo test --workspace

lint:
	cargo fmt --all --check
	cargo clippy --workspace --all-targets -- -D warnings

demo: demo-stage-0

demo-stage-0:
	./scripts/demo.sh 0

demo-ready:
	./scripts/demo.sh 4

demo-verify:
	./scripts/demo.sh 5

clean:
	cargo clean
	rm -f retirement.proof.json
