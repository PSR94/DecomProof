# Policy configuration

`.decomproof.yml` configures minimum runtime/external observation windows, maximum evidence age, schedule-cycle expectations, unknown-consumer handling, API sunset/deprecation expectations and data quiescence requirements.

The current Rust configuration uses explicit flat policy keys so unknown nested structures do not silently change semantics. Configuration validation is conservative: malformed YAML causes `doctor` to fail.
