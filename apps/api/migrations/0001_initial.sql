-- PostgreSQL reference migration for DecomProof API v0.1.
CREATE TABLE IF NOT EXISTS projects (
  id BIGSERIAL PRIMARY KEY,
  name VARCHAR(200) NOT NULL UNIQUE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE TABLE IF NOT EXISTS targets (
  id BIGSERIAL PRIMARY KEY,
  project_id BIGINT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
  stable_id VARCHAR(500) NOT NULL,
  kind VARCHAR(80) NOT NULL,
  target_key VARCHAR(420) NOT NULL,
  fingerprint VARCHAR(80) NOT NULL,
  lifecycle_state VARCHAR(40) NOT NULL DEFAULT 'ACTIVE',
  UNIQUE(project_id, stable_id)
);
CREATE TABLE IF NOT EXISTS analysis_runs (
  id BIGSERIAL PRIMARY KEY,
  target_id BIGINT NOT NULL REFERENCES targets(id) ON DELETE CASCADE,
  generated_at TIMESTAMPTZ NOT NULL,
  revision_commit VARCHAR(100) NOT NULL,
  revision_branch VARCHAR(200) NOT NULL,
  verdict VARCHAR(60) NOT NULL,
  score INTEGER NOT NULL CHECK(score BETWEEN 0 AND 100),
  policy_version VARCHAR(100) NOT NULL,
  proof_json JSONB NOT NULL
);
CREATE TABLE IF NOT EXISTS evidence_items (
  id BIGSERIAL PRIMARY KEY,
  analysis_id BIGINT NOT NULL REFERENCES analysis_runs(id) ON DELETE CASCADE,
  evidence_id VARCHAR(80) NOT NULL,
  signal VARCHAR(180) NOT NULL,
  source VARCHAR(180) NOT NULL,
  observed_at TIMESTAMPTZ NOT NULL,
  confidence VARCHAR(30) NOT NULL,
  payload JSONB NOT NULL
);
CREATE INDEX IF NOT EXISTS ix_evidence_analysis ON evidence_items(analysis_id);
CREATE INDEX IF NOT EXISTS ix_evidence_signal ON evidence_items(signal);
CREATE TABLE IF NOT EXISTS lifecycle_events (
  id BIGSERIAL PRIMARY KEY,
  target_id BIGINT NOT NULL REFERENCES targets(id) ON DELETE CASCADE,
  state VARCHAR(40) NOT NULL,
  occurred_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  note TEXT
);
CREATE TABLE IF NOT EXISTS verification_runs (
  id BIGSERIAL PRIMARY KEY,
  target_id BIGINT NOT NULL REFERENCES targets(id) ON DELETE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  verdict VARCHAR(60) NOT NULL,
  payload JSONB NOT NULL
);
