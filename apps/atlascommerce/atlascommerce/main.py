import json
import os
from datetime import datetime, timezone
from pathlib import Path

import psycopg
from fastapi import FastAPI, Header

app = FastAPI(title="AtlasCommerce", version="2026.9")
LOG = Path(os.getenv("ATLAS_ACCESS_LOG", "/data/access.jsonl"))
DATABASE_URL = os.getenv("DATABASE_URL", "postgresql://atlas:atlas@postgres:5432/atlas")


def record(path: str, client: str, status: int = 200):
    LOG.parent.mkdir(parents=True, exist_ok=True)
    with LOG.open("a") as fh:
        fh.write(json.dumps({"timestamp": datetime.now(timezone.utc).isoformat(), "method": "POST", "path": path, "status": status, "client_id": client}) + "\n")


@app.get("/healthz")
def healthz():
    return {"status": "ok", "service": "atlascommerce"}


@app.post("/v2/export")
def current_export(x_client_id: str = Header(default="internal")):
    record("/v2/export", x_client_id)
    return {"export": "current"}


@app.post("/v1/legacy-export", deprecated=True)
def legacy_export(x_client_id: str = Header(default="unknown")):
    record("/v1/legacy-export", x_client_id)
    with psycopg.connect(DATABASE_URL) as conn:
        with conn.cursor() as cur:
            cur.execute("INSERT INTO legacy_exports(source) VALUES (%s) RETURNING id", (x_client_id,))
            export_id = cur.fetchone()[0]
        conn.commit()
    return {"export": "legacy", "id": export_id}
