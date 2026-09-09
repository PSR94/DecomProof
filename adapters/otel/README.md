# OpenTelemetry JSON evidence

`--source otel-json` parses OTLP JSON `resourceSpans`, searches matching spans and uses `service.name` as an identified consumer when available. No sampling-completeness claim is inferred from the file; users should record export coverage gaps separately when traces are sampled or incomplete.
