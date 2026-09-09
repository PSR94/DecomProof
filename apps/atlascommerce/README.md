# AtlasCommerce live demo service

This optional service makes the hidden runtime/data path reproducible: both a finance worker and a fake external customer can call the deprecated `/v1/legacy-export` endpoint, which appends structured access evidence and writes `legacy_exports` rows. The offline staged fixtures remain the deterministic CI path.

No production credentials or external systems are required.
