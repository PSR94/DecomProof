import httpx


class ExportClient:
    def __init__(self, base_url: str): self.base_url = base_url
    def export(self): return httpx.post(f"{self.base_url}/v2/export", timeout=10).json()


class LegacyExportClient:
    """Deprecated public compatibility client retained to demonstrate SDK exposure."""
    def __init__(self, base_url: str): self.base_url = base_url
    def export(self): return httpx.post(f"{self.base_url}/v1/legacy-export", timeout=10).json()


__all__ = ["ExportClient", "LegacyExportClient"]
