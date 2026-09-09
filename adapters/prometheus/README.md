# Prometheus text evidence

The Rust CLI accepts Prometheus exposition text directly:

```bash
decomproof ingest service:legacy-export --source prometheus metrics.txt
```

Matching non-comment series containing the target identifier are summed into a `metric.usage` evidence item. Because an exposition snapshot does not itself prove a historical window, this evidence does not manufacture one.
