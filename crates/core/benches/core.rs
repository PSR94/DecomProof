use chrono::{Duration, TimeZone, Utc};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use decomproof_core::{
    config::Config,
    evidence::{Confidence, Evidence, ObservationWindow},
    graph::build,
    policy::evaluate,
    proof::{RetirementProof, Revision},
    scanner::scan_workspace,
    target::Target,
    temporal::infer_pattern,
};
use std::fs;

fn evidence_fixture() -> (Target, Vec<Evidence>) {
    let t: Target = "service:legacy-export".parse().unwrap();
    let end = Utc.with_ymd_and_hms(2026, 9, 9, 20, 0, 0).unwrap();
    let mut e = Evidence::build("runtime.http.requests", "bench", &t, end, None, b"bench");
    e.confidence = Confidence::High;
    e.observation_window = Some(ObservationWindow {
        start: end - Duration::days(50),
        end,
        gaps_seconds: 0,
    });
    e.normalized.insert("count".into(), 0.into());
    e.normalized.insert("active".into(), false.into());
    (t, vec![e])
}

fn bench_core(c: &mut Criterion) {
    let (target, evidence) = evidence_fixture();
    c.bench_function("graph_build_1k", |b| {
        let many: Vec<_> = (0..1000)
            .map(|i| {
                let mut e = evidence[0].clone();
                e.id = format!("ev_{i:020x}");
                e.normalized
                    .insert("dependency".into(), format!("src/{i}.ts").into());
                e
            })
            .collect();
        b.iter(|| build(black_box(&target.stable_id()), black_box(&many)));
    });
    c.bench_function("policy_evaluate", |b| {
        b.iter(|| {
            evaluate(
                black_box(&Config::default().policy),
                black_box(&evidence),
                Utc.with_ymd_and_hms(2026, 9, 9, 20, 0, 0).unwrap(),
            )
        })
    });
    c.bench_function("temporal_pattern_1k", |b| {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let xs: Vec<_> = (0..1000)
            .map(|i| start + Duration::hours(i * 24))
            .collect();
        b.iter(|| infer_pattern(black_box(xs.clone())));
    });
    c.bench_function("proof_serialization", |b| {
        let p = RetirementProof::analyze(
            target.clone(),
            evidence.clone(),
            &Config::default(),
            Revision {
                commit: "abc".into(),
                branch: "main".into(),
            },
            Utc.with_ymd_and_hms(2026, 9, 9, 20, 0, 0).unwrap(),
        );
        b.iter(|| black_box(p.canonical_json()));
    });

    let dir = tempfile::tempdir().unwrap();
    for i in 0..500 {
        fs::write(
            dir.path().join(format!("file-{i}.ts")),
            format!("export const x{i} = 'legacy-export';"),
        )
        .unwrap();
    }
    c.bench_function("source_scan_500_files", |b| {
        b.iter(|| scan_workspace(black_box(dir.path()), black_box(&target)))
    });
}
criterion_group!(benches, bench_core);
criterion_main!(benches);
