import os

import httpx


def run_once():
    base = os.getenv("ATLAS_BASE_URL", "http://atlascommerce:8080")
    response = httpx.post(f"{base}/v1/legacy-export", headers={"x-client-id": "nightly-finance-export"}, timeout=10)
    response.raise_for_status()


if __name__ == "__main__":
    run_once()
