use chrono::{Duration, TimeZone, Utc};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use decomproof_core::{
    config::Config,
    evidence::{Confidence, Evidence, ObservationWindow},
    graph::build,
    ingest::ingest_jsonl,
    policy::evaluate,
    proof::{RetirementProof, Revision},
    scanner::scan_workspace,
    target::Target,
    temporal::infer_pattern,
};
use std::{fmt::Write as _, fs};

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

fn graph_evidence(seed: &Evidence, count: usize) -> Vec<Evidence> {
    (0..count)
        .map(|i| {
            let mut e = seed.clone();
            e.id = format!("ev_{i:020x}");
            e.normalized
                .insert("dependency".into(), format!("src/{i}.ts").into());
            e
        })
        .collect()
}

fn write_source_fixture(dir: &std::path::Path, count: usize) {
    for i in 0..count {
        fs::write(
            dir.join(format!("file-{i}.ts")),
            format!("export const x{i} = 'legacy-export';"),
        )
        .unwrap();
    }
}

fn bench_core(c: &mut Criterion) {
    let (target, evidence) = evidence_fixture();

    let graph_1k = graph_evidence(&evidence[0], 1_000);
    c.bench_function("graph_build_1k", |b| {
        b.iter(|| build(black_box(&target.stable_id()), black_box(&graph_1k)));
    });

    let graph_10k = graph_evidence(&evidence[0], 10_000);
    c.bench_function("graph_build_10k_stress", |b| {
        b.iter(|| build(black_box(&target.stable_id()), black_box(&graph_10k)));
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

    let dir_500 = tempfile::tempdir().unwrap();
    write_source_fixture(dir_500.path(), 500);
    c.bench_function("source_scan_500_files", |b| {
        b.iter(|| scan_workspace(black_box(dir_500.path()), black_box(&target)))
    });

    let dir_5k = tempfile::tempdir().unwrap();
    write_source_fixture(dir_5k.path(), 5_000);
    c.bench_function("source_scan_5k_files_stress", |b| {
        b.iter(|| scan_workspace(black_box(dir_5k.path()), black_box(&target)))
    });

    let telemetry_dir = tempfile::tempdir().unwrap();
    let telemetry_path = telemetry_dir.path().join("telemetry-100k.jsonl");
    let mut telemetry = String::with_capacity(14_000_000);
    for i in 0..100_000 {
        writeln!(
            telemetry,
            r#"{{"signal":"runtime.http.requests","source":"stress","observed_at":"2026-09-09T20:00:00Z","count":0,"active":false,"request":{i}}}"#
        )
        .unwrap();
    }
    fs::write(&telemetry_path, telemetry).unwrap();
    c.bench_function("jsonl_ingest_100k_stress", |b| {
        b.iter(|| ingest_jsonl(black_box(telemetry_path.as_path()), black_box(&target)).unwrap())
    });
}

criterion_group!(benches, bench_core);
criterion_main!(benches);
