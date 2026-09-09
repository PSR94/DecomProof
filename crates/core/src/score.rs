use crate::{
    evidence::Uncertainty,
    policy::{Blocker, PolicyDecision, Verdict},
};

pub fn readiness_score(
    blockers: &[Blocker],
    uncertainties: &[Uncertainty],
    decisions: &[PolicyDecision],
) -> u8 {
    let mut score: i32 = 100;
    for b in blockers {
        score -= if b.hard {
            match b.severity {
                crate::policy::Severity::Critical => 25,
                crate::policy::Severity::High => 15,
                crate::policy::Severity::Medium => 8,
                crate::policy::Severity::Low => 3,
                crate::policy::Severity::Informational => 0,
            }
        } else {
            3
        };
    }
    score -= uncertainties.iter().map(|u| if u.severity == "high" { 10 } else { 5 }).sum::<i32>();
    score -= decisions.iter().filter(|d| !d.passed).count() as i32 * 8;
    score.clamp(0, 100) as u8
}

pub fn verdict(
    score: u8,
    blockers: &[Blocker],
    uncertainties: &[Uncertainty],
    decisions: &[PolicyDecision],
    has_evidence: bool,
) -> Verdict {
    if blockers.iter().any(|b| b.hard) {
        return Verdict::Blocked;
    }
    if !has_evidence || decisions.iter().any(|d| !d.passed && d.effect == "insufficient-evidence") {
        return Verdict::InsufficientEvidence;
    }
    if decisions.iter().any(|d| !d.passed && d.effect == "observing") {
        return Verdict::Observing;
    }
    if score == 100 && uncertainties.is_empty() {
        Verdict::Ready
    } else if score >= 90 {
        Verdict::ReadyWithUncertainty
    } else {
        Verdict::Quiescent
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn no_evidence_is_never_ready() {
        assert_eq!(verdict(100, &[], &[], &[], false), Verdict::InsufficientEvidence);
    }
}
