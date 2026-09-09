use anyhow::{Context, Result};
use chrono::Utc;
use clap::{Parser, Subcommand, ValueEnum};
use decomproof_core::{
    config::Config,
    ingest::{ingest_access_log, ingest_jsonl, ingest_otel_json, ingest_prometheus},
    lifecycle::{LifecycleState, LifecycleStore},
    proof::{RetirementProof, Revision},
    report,
    scanner::scan_workspace,
    target::Target,
    verify,
};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command as ProcessCommand,
};

#[derive(Parser)]
#[command(name = "decomproof", version, about = "Evidence-backed software decommissioning")]
struct Cli {
    #[arg(long, global = true, default_value = ".decomproof.yml")]
    config: PathBuf,
    #[arg(long, global = true)]
    json: bool,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum IngestSource {
    GenericJson,
    AccessLog,
    OtelJson,
    Prometheus,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum ReportFormat {
    Terminal,
    Json,
    Markdown,
    Html,
}

#[derive(Subcommand)]
enum Command {
    Init,
    Doctor,
    Analyze {
        target: String,
        #[arg(long, default_value = ".")]
        root: PathBuf,
        #[arg(long = "evidence")]
        evidence: Vec<PathBuf>,
        #[arg(long, default_value = "retirement.proof.json")]
        output: PathBuf,
    },
    Scan {
        target: String,
        #[arg(long, default_value = ".")]
        root: PathBuf,
    },
    Ingest {
        target: String,
        #[arg(long, value_enum, default_value = "generic-json")]
        source: IngestSource,
        file: PathBuf,
    },
    Proof {
        file: PathBuf,
        #[arg(long, value_enum, default_value = "terminal")]
        format: ReportFormat,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    Evidence {
        file: PathBuf,
        #[arg(long)]
        signal: Option<String>,
    },
    Graph {
        file: PathBuf,
    },
    Blockers {
        file: PathBuf,
    },
    Verify {
        target: String,
        #[arg(long = "evidence")]
        evidence: Vec<PathBuf>,
        #[arg(long)]
        record_lifecycle: bool,
        #[arg(long, default_value = ".decomproof/lifecycle.json")]
        state_file: PathBuf,
    },
    Status {
        target: String,
        #[arg(long, default_value = ".decomproof/lifecycle.json")]
        state_file: PathBuf,
    },
    Lifecycle {
        target: String,
        #[arg(long)]
        set: Option<String>,
        #[arg(long)]
        note: Option<String>,
        #[arg(long, default_value = ".decomproof/lifecycle.json")]
        state_file: PathBuf,
        #[arg(long, default_value = "retirement.proof.json")]
        proof: PathBuf,
    },
    History {
        target: String,
        #[arg(long, default_value = ".")]
        root: PathBuf,
    },
    Diff {
        old: PathBuf,
        new: PathBuf,
    },
    Config,
    Demo {
        #[arg(long, default_value_t = 0)]
        stage: u8,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Init => init(&cli.config),
        Command::Doctor => doctor(&cli.config),
        Command::Config => {
            println!("{}", serde_json::to_string_pretty(&Config::load(&cli.config)?)?);
            Ok(())
        }
        Command::Scan { target, root } => {
            let target: Target = target.parse()?;
            println!("{}", serde_json::to_string_pretty(&scan_workspace(&root, &target))?);
            Ok(())
        }
        Command::Ingest { target, source, file } => {
            let target: Target = target.parse()?;
            let evidence = match source {
                IngestSource::GenericJson => ingest_jsonl(&file, &target)?,
                IngestSource::AccessLog => ingest_access_log(&file, &target)?,
                IngestSource::OtelJson => ingest_otel_json(&file, &target)?,
                IngestSource::Prometheus => ingest_prometheus(&file, &target)?,
            };
            println!("{}", serde_json::to_string_pretty(&evidence)?);
            Ok(())
        }
        Command::Analyze { target, root, evidence, output } => {
            analyze(&cli.config, &target, &root, &evidence, &output, cli.json)
        }
        Command::Proof { file, format, output } => {
            let proof = load_proof(&file)?;
            let content = match if cli.json { ReportFormat::Json } else { format } {
                ReportFormat::Terminal => report::terminal(&proof),
                ReportFormat::Json => proof.canonical_json(),
                ReportFormat::Markdown => report::markdown(&proof),
                ReportFormat::Html => report::html(&proof),
            };
            if let Some(path) = output {
                fs::write(path, content)?;
            } else {
                print!("{content}");
            }
            Ok(())
        }
        Command::Evidence { file, signal } => {
            let proof = load_proof(&file)?;
            let evidence: Vec<_> = proof
                .evidence
                .iter()
                .filter(|item| signal.as_ref().map(|expected| &item.signal == expected).unwrap_or(true))
                .collect();
            println!("{}", serde_json::to_string_pretty(&evidence)?);
            Ok(())
        }
        Command::Graph { file } => {
            let proof = load_proof(&file)?;
            println!("{}", serde_json::to_string_pretty(&proof.result().graph)?);
            Ok(())
        }
        Command::Blockers { file } => {
            let proof = load_proof(&file)?;
            println!("{}", serde_json::to_string_pretty(&proof.blockers)?);
            Ok(())
        }
        Command::Verify { target, evidence, record_lifecycle, state_file } => {
            let target: Target = target.parse()?;
            let mut all = Vec::new();
            for file in evidence {
                all.extend(ingest_jsonl(&file, &target)?);
            }
            let result = verify::verify(&all);
            println!("{}", serde_json::to_string_pretty(&result)?);
            if result.verdict == "verified" && record_lifecycle {
                let mut store = LifecycleStore::load(&state_file)?;
                store.transition(
                    &target,
                    LifecycleState::Verified,
                    Utc::now(),
                    Some("post-removal verification passed".into()),
                )?;
                store.save(&state_file)?;
            }
            if result.verdict == "regression-detected" || result.verdict == "insufficient-evidence" {
                std::process::exit(2);
            }
            Ok(())
        }
        Command::Status { target, state_file } => {
            let target: Target = target.parse()?;
            let mut store = LifecycleStore::load(&state_file)?;
            let record = store.record_for(&target, Utc::now()).clone();
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&record)?);
            } else {
                println!(
                    "Target: {}\nLifecycle: {}\nUpdated: {}",
                    record.target, record.state, record.updated_at
                );
            }
            Ok(())
        }
        Command::Lifecycle { target, set, note, state_file, proof } => {
            let target: Target = target.parse()?;
            let mut store = LifecycleStore::load(&state_file)?;
            if let Some(raw) = set {
                let next: LifecycleState = raw.parse()?;
                if next == LifecycleState::Verified {
                    anyhow::bail!("VERIFIED can only be recorded by `decomproof verify --record-lifecycle`");
                }
                if next == LifecycleState::Ready {
                    let proof = load_proof(&proof).context("READY requires a generated proof")?;
                    if proof.target.stable_id() != target.stable_id() {
                        anyhow::bail!("proof target {} does not match {}", proof.target, target);
                    }
                    if !matches!(
                        proof.verdict,
                        decomproof_core::policy::Verdict::Ready
                            | decomproof_core::policy::Verdict::ReadyWithUncertainty
                    ) {
                        anyhow::bail!("cannot enter READY: proof verdict is {:?}", proof.verdict);
                    }
                }
                store.transition(&target, next, Utc::now(), note)?;
                store.save(&state_file)?;
            }
            println!("{}", serde_json::to_string_pretty(store.record_for(&target, Utc::now()))?);
            Ok(())
        }
        Command::History { target, root } => {
            let target: Target = target.parse()?;
            git_history(&root, &target.id)
        }
        Command::Diff { old, new } => diff(&old, &new),
        Command::Demo { stage } => {
            if stage > 6 {
                anyhow::bail!("demo stage must be 0..=6");
            }
            let root = PathBuf::from(format!("examples/atlascommerce/stages/stage-{stage}"));
            if stage >= 5 {
                let evidence = if stage == 5 {
                    root.join("cleanup-leftovers.jsonl")
                } else {
                    root.join("runtime.jsonl")
                };
                let target: Target = "service:legacy-export".parse()?;
                println!(
                    "{}",
                    serde_json::to_string_pretty(&verify::verify(&ingest_jsonl(&evidence, &target)?))?
                );
                Ok(())
            } else {
                analyze(
                    &cli.config,
                    "service:legacy-export",
                    &root,
                    &[root.join("runtime.jsonl")],
                    &PathBuf::from("retirement.proof.json"),
                    cli.json,
                )
            }
        }
    }
}

fn init(path: &Path) -> Result<()> {
    if path.exists() {
        anyhow::bail!("{} already exists", path.display());
    }
    fs::write(
        path,
        "project:\n  name: my-project\npolicy:\n  runtime_minimum_window_days: 30\n  external_minimum_window_days: 45\n  evidence_maximum_age_hours: 24\n  scheduled_minimum_cycles: 3\n  unidentified_consumers_block: true\n  public_api_requires_deprecation: true\n  api_minimum_sunset_days: 30\n  data_zero_writes_days: 30\nprivacy:\n  hash_consumer_ids: true\n",
    )?;
    println!("created {}", path.display());
    Ok(())
}

fn doctor(path: &Path) -> Result<()> {
    let _ = Config::load(path).with_context(|| format!("loading {}", path.display()))?;
    println!(
        "configuration: ok\nnetwork integrations: disabled unless explicitly configured\nread-only analysis: enabled"
    );
    Ok(())
}

fn analyze(
    config_path: &Path,
    target: &str,
    root: &Path,
    evidence_files: &[PathBuf],
    output: &Path,
    json: bool,
) -> Result<()> {
    let config = Config::load_or_default_if_missing(config_path)
        .with_context(|| format!("loading {}", config_path.display()))?;
    let target: Target = target.parse()?;
    let mut evidence = scan_workspace(root, &target);
    for file in evidence_files {
        evidence.extend(ingest_jsonl(file, &target)?);
    }
    let proof = RetirementProof::analyze(
        target,
        evidence,
        &config,
        Revision { commit: git(&["rev-parse", "HEAD"]), branch: git(&["branch", "--show-current"]) },
        Utc::now(),
    );
    fs::write(output, proof.canonical_json())?;
    if json {
        print!("{}", proof.canonical_json());
    } else {
        print!("{}", report::terminal(&proof));
    }
    if matches!(
        proof.verdict,
        decomproof_core::policy::Verdict::Blocked | decomproof_core::policy::Verdict::InsufficientEvidence
    ) {
        std::process::exit(2);
    }
    Ok(())
}

fn git(args: &[&str]) -> String {
    ProcessCommand::new("git")
        .args(args)
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "unknown".into())
}

fn git_history(root: &Path, needle: &str) -> Result<()> {
    let output = ProcessCommand::new("git")
        .arg("-C")
        .arg(root)
        .args(["log", "--all", "--date=iso-strict", "--format=%H%x09%aI%x09%s", "-S", needle, "--"])
        .output()
        .context("running git history search")?;
    if !output.status.success() {
        anyhow::bail!("git history search failed");
    }
    let rows = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| {
            let mut parts = line.splitn(3, '\t');
            serde_json::json!({
                "commit": parts.next().unwrap_or(""),
                "at": parts.next().unwrap_or(""),
                "subject": parts.next().unwrap_or("")
            })
        })
        .collect::<Vec<_>>();
    println!("{}", serde_json::to_string_pretty(&rows)?);
    Ok(())
}

fn diff(old: &Path, new: &Path) -> Result<()> {
    let before = load_proof(old)?;
    let after = load_proof(new)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "target": after.target.stable_id(),
            "score": {
                "from": before.score,
                "to": after.score,
                "delta": after.score as i16 - before.score as i16
            },
            "verdict": {
                "from": format!("{:?}", before.verdict),
                "to": format!("{:?}", after.verdict)
            },
            "blockers": {
                "from": before.blockers.len(),
                "to": after.blockers.len()
            },
            "uncertainties": {
                "from": before.uncertainties.len(),
                "to": after.uncertainties.len()
            }
        }))?
    );
    Ok(())
}

fn load_proof(path: &Path) -> Result<RetirementProof> {
    Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}
