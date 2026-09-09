import os

import httpx


def call_legacy_export():
    base = os.getenv("ATLAS_BASE_URL", "http://atlascommerce:8080")
    response = httpx.post(f"{base}/v1/legacy-export", headers={"x-client-id": "customer-api-key-1842"}, timeout=10)
    response.raise_for_status()


if __name__ == "__main__":
    call_legacy_export()
