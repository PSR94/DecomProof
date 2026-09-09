# Security policy

## Reporting

Please report vulnerabilities privately through GitHub Security Advisories for this repository rather than opening a public issue.

## Design principles

- analysis is read-only by default
- no automatic production resource deletion in v0.1
- database and message-broker connections must be explicitly configured
- use least-privilege, read-only credentials
- never persist plaintext authorization headers, cookies or API keys in normalized evidence
- hash consumer identifiers when a stable pseudonym is sufficient
- validate imported files and enforce practical size/time limits at integration boundaries
- avoid shell interpolation of user-controlled target strings
- network integrations stay disabled until configured
- generated proof artifacts may contain operational metadata and should be stored with appropriate access controls

The generic evidence ingester hashes supplied consumer IDs. It does not claim to sanitize every arbitrary value placed into `extra` fields; exporters should redact sensitive payloads before import.
