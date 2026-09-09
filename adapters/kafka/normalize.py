#!/usr/bin/env python3
"""Normalize a sanitized Kafka metadata export into DecomProof generic evidence JSONL."""
import argparse
import json
from datetime import datetime, timezone
from pathlib import Path


def main():
    p = argparse.ArgumentParser()
    p.add_argument("file", type=Path, help="JSON array of {topic, consumer_group, active, lag?, last_observed?}")
    args = p.parse_args()
    rows = json.loads(args.file.read_text())
    if not isinstance(rows, list):
        raise SystemExit("Kafka export must be a JSON array")
    now = datetime.now(timezone.utc).isoformat().replace("+00:00", "Z")
    for row in rows:
        active = bool(row.get("active", False))
        out = {
            "signal": "event.consume",
            "source": "kafka-metadata-export",
            "observed_at": row.get("last_observed", now),
            "count": 1 if active else 0,
            "active": active,
            "consumer_type": "consumer-group",
            "consumer_id": row.get("consumer_group", "unknown"),
            "consumer_identification": "identified" if row.get("consumer_group") else "unknown",
            "topic": row.get("topic"),
        }
        if "lag" in row:
            out["lag"] = row["lag"]
        print(json.dumps(out, separators=(",", ":")))


if __name__ == "__main__":
    main()
