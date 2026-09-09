use crate::{
    evidence::{Evidence, Uncertainty},
    graph::{build, DependencyGraph},
    policy::{evaluate, Blocker, PolicyDecision, Verdict},
    score::{readiness_score, verdict},
    Config, Target,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Revision {
    pub commit: String,
    pub branch: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationPolicy {
    pub runtime_window_days: u32,
    pub external_window_days: u32,
    pub maximum_evidence_age_hours: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub score: u8,
    pub verdict: Verdict,
    pub blockers: Vec<Blocker>,
    pub uncertainties: Vec<Uncertainty>,
    pub policy_decisions: Vec<PolicyDecision>,
    pub graph: DependencyGraph,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetirementProof {
    pub schema: String,
    pub target: Target,
    pub target_fingerprint: String,
    pub revision: Revision,
    pub generated_at: DateTime<Utc>,
    pub policy_version: String,
    pub observation_policy: ObservationPolicy,
    pub signals: BTreeMap<String, u64>,
    pub blockers: Vec<Blocker>,
    pub uncertainties: Vec<Uncertainty>,
    pub score: u8,
    pub verdict: Verdict,
    pub verdict_rationale: Vec<String>,
    pub policy_decisions: Vec<PolicyDecision>,
    pub evidence: Vec<Evidence>,
    pub evidence_digest: String,
}
impl RetirementProof {
    pub fn analyze(
        target: Target,
        mut evidence: Vec<Evidence>,
        config: &Config,
        revision: Revision,
        now: DateTime<Utc>,
    ) -> Self {
        for e in &mut evidence {
            e.freshness_seconds = Some(e.age_seconds(now));
        }
        evidence.sort_by(|a, b| a.id.cmp(&b.id));
        let pe = evaluate(&config.policy, &evidence, now);
        let score = readiness_score(&pe.blockers, &pe.uncertainties, &pe.decisions);
        let verdict = verdict(score, &pe.blockers, &pe.uncertainties, &pe.decisions, !evidence.is_empty());
        let mut signals = BTreeMap::new();
        for e in &evidence {
            *signals.entry(e.signal.clone()).or_insert(0) += 1
        }
        let digest_input = serde_json::to_vec(&evidence).unwrap_or_default();
        let evidence_digest = format!("sha256:{}", hex::encode(Sha256::digest(digest_input)));
        let verdict_rationale = rationale(verdict, &pe.blockers, &pe.decisions, &pe.uncertainties);
        Self {
            schema: "decomproof/v1".into(),
            target_fingerprint: target.fingerprint(),
            target,
            revision,
            generated_at: now,
            policy_version: "decomproof-policy/v1".into(),
            observation_policy: ObservationPolicy {
                runtime_window_days: config.policy.runtime_minimum_window_days,
                external_window_days: config.policy.external_minimum_window_days,
                maximum_evidence_age_hours: config.policy.evidence_maximum_age_hours,
            },
            signals,
            blockers: pe.blockers,
            uncertainties: pe.uncertainties,
            score,
            verdict,
            verdict_rationale,
            policy_decisions: pe.decisions,
            evidence,
            evidence_digest,
        }
    }
    pub fn result(&self) -> AnalysisResult {
        AnalysisResult {
            score: self.score,
            verdict: self.verdict,
            blockers: self.blockers.clone(),
            uncertainties: self.uncertainties.clone(),
            policy_decisions: self.policy_decisions.clone(),
            graph: build(&self.target.stable_id(), &self.evidence),
        }
    }
    pub fn canonical_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("serializable proof") + "\n"
    }
}
fn rationale(v: Verdict, b: &[Blocker], d: &[PolicyDecision], u: &[Uncertainty]) -> Vec<String> {
    let mut r = vec![format!("deterministic verdict: {:?}", v)];
    if b.iter().any(|x| x.hard) {
        r.push(format!("{} hard blocker(s) prevent readiness", b.iter().filter(|x| x.hard).count()));
    }
    for x in d.iter().filter(|x| !x.passed) {
        r.push(format!("policy {} failed: {}", x.policy, x.available));
    }
    if !u.is_empty() {
        r.push(format!("{} explicit uncertainty item(s) remain", u.len()));
    }
    r
}
