#!/usr/bin/env python3
"""Small deterministic preflight; not a replacement for a dedicated secret scanner."""
from pathlib import Path
import re

patterns = [
    re.compile(r"AKIA[0-9A-Z]{16}"),
    re.compile(r"ghp_[A-Za-z0-9]{30,}"),
    re.compile(r"-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----"),
]
ignored = {"node_modules", ".git", "target", ".next"}
findings = []
for path in Path(".").rglob("*"):
    if not path.is_file() or any(part in ignored for part in path.parts):
        continue
    try: text = path.read_text(errors="ignore")
    except OSError: continue
    for pattern in patterns:
        if pattern.search(text): findings.append(f"{path}: {pattern.pattern}")
if findings:
    raise SystemExit("potential secrets found:\n" + "\n".join(findings))
print("no high-confidence secret patterns found")
