use chrono::{TimeZone, Utc};
use decomproof_core::{
    config::Config,
    ingest::ingest_jsonl,
    policy::Verdict,
    proof::{RetirementProof, Revision},
    scanner::scan_workspace,
    target::Target,
};
use std::path::PathBuf;

fn stage(n: u8) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../../examples/atlascommerce/stages/stage-{n}"))
}
fn analyze(n: u8) -> RetirementProof {
    let root = stage(n);
    let target: Target = "service:legacy-export".parse().unwrap();
    let mut evidence = scan_workspace(&root, &target);
    evidence.extend(ingest_jsonl(&root.join("runtime.jsonl"), &target).unwrap());
    RetirementProof::analyze(
        target,
        evidence,
        &Config::default(),
        Revision { commit: "fixture".into(), branch: "test".into() },
        Utc.with_ymd_and_hms(2026, 9, 9, 21, 0, 0).unwrap(),
    )
}
#[test]
fn stage_0_is_blocked_by_real_fixture_evidence() {
    let proof = analyze(0);
    assert_eq!(proof.verdict, Verdict::Blocked);
    assert!(proof.blockers.iter().any(|b| b.kind == "active-runtime-consumer"));
    assert!(proof.blockers.iter().any(|b| b.kind == "active-schedule"));
    assert!(proof.blockers.iter().any(|b| b.kind == "public-contract"));
}
#[test]
fn stage_4_is_ready_after_dependencies_and_windows_clear() {
    let proof = analyze(4);
    assert_eq!(proof.verdict, Verdict::Ready);
    assert_eq!(proof.score, 100);
    assert!(proof.blockers.is_empty());
    assert!(proof.policy_decisions.iter().all(|d| d.passed));
}
