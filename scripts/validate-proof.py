#!/usr/bin/env python3
import argparse
import json
from pathlib import Path

import jsonschema

p = argparse.ArgumentParser()
p.add_argument("proof", type=Path)
p.add_argument("--schema", type=Path, default=Path("schemas/retirement-proof-v1.schema.json"))
args = p.parse_args()
schema = json.loads(args.schema.read_text())
proof = json.loads(args.proof.read_text())
jsonschema.Draft202012Validator(schema, format_checker=jsonschema.FormatChecker()).validate(proof)
print(f"valid: {args.proof} against {args.schema}")
