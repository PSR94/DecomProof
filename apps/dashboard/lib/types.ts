export type Evidence = {
  id: string;
  signal: string;
  source: string;
  observed_at: string;
  confidence: string;
  artifact?: string;
  normalized: Record<string, unknown>;
  observed: Record<string, unknown>;
  consumer?: { identity: string; type: string; id: string };
  observation_window?: { start: string; end: string; gaps_seconds: number };
};

export type Blocker = { id: string; severity: string; kind: string; message: string; hard: boolean; evidence_ids: string[] };
export type Uncertainty = { id: string; kind: string; message: string; severity: string; evidence_ids: string[] };
export type Proof = {
  schema: "decomproof/v1";
  target: { kind: string; id: string; metadata?: Record<string, string> };
  target_fingerprint: string;
  generated_at: string;
  revision: { commit: string; branch: string };
  policy_version: string;
  signals: Record<string, number>;
  score: number;
  verdict: string;
  blockers: Blocker[];
  uncertainties: Uncertainty[];
  verdict_rationale: string[];
  policy_decisions: Array<{ policy: string; required: string; available: string; passed: boolean; effect: string; evidence_ids: string[] }>;
  evidence: Evidence[];
};

export type Target = { id: number; project_id: number; stable_id: string; kind: string; target_key: string; fingerprint: string; lifecycle_state: string };
export type AnalysisRun = { id: number; target_id: number; generated_at: string; revision_commit: string; revision_branch: string; verdict: string; score: number; policy_version: string };
