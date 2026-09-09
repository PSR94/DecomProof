# Kafka evidence

v0.1 uses an offline metadata normalizer rather than taking broker credentials by default. Export topic/consumer-group state from your existing read-only tooling, sanitize it, then normalize with `normalize.py`. Consumer group names are passed to the core ingester, which hashes consumer IDs in the proof.

A native AdminClient collector is roadmap work and must remain metadata/read-only.
