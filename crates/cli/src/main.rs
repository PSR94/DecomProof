use anyhow::{Context, Result};
use chrono::Utc;
use clap::{Parser, Subcommand, ValueEnum};
use decomproof_core::{config::Config,ingest::{ingest_access_log,ingest_jsonl,ingest_otel_json,ingest_prometheus},lifecycle::{LifecycleState,LifecycleStore},proof::{RetirementProof,Revision},report,scanner::scan_workspace,target::Target,verify};
use std::{fs,path::{Path,PathBuf},process::Command as ProcessCommand};

#[derive(Parser)]#[command(name="decomproof",version,about="Evidence-backed software decommissioning")]
struct Cli{#[arg(long,global=true,default_value=".decomproof.yml")]config:PathBuf,#[arg(long,global=true)]json:bool,#[command(subcommand)]command:Command}
#[derive(Debug,Clone,Copy,ValueEnum)]enum IngestSource{GenericJson,AccessLog,OtelJson,Prometheus}
#[derive(Subcommand)]enum Command{
 Init,Doctor,
 Analyze{target:String,#[arg(long,default_value=".")]root:PathBuf,#[arg(long="evidence")]evidence:Vec<PathBuf>,#[arg(long,default_value="retirement.proof.json")]output:PathBuf},
 Scan{target:String,#[arg(long,default_value=".")]root:PathBuf},
 Ingest{target:String,#[arg(long,value_enum,default_value="generic-json")]source:IngestSource,file:PathBuf},
 Proof{file:PathBuf},Graph{file:PathBuf},Blockers{file:PathBuf},
 Verify{target:String,#[arg(long="evidence")]evidence:Vec<PathBuf>},
 Status{target:String,#[arg(long,default_value=".decomproof/lifecycle.json")]state_file:PathBuf},
 Lifecycle{target:String,#[arg(long)]set:Option<String>,#[arg(long)]note:Option<String>,#[arg(long,default_value=".decomproof/lifecycle.json")]state_file:PathBuf,#[arg(long,default_value="retirement.proof.json")]proof:PathBuf},
 Demo{#[arg(long,default_value_t=0)]stage:u8}
}
fn main()->Result<()>{let cli=Cli::parse();match cli.command{
 Command::Init=>init(&cli.config),Command::Doctor=>doctor(&cli.config),
 Command::Scan{target,root}=>{let t:Target=target.parse()?;println!("{}",serde_json::to_string_pretty(&scan_workspace(&root,&t))?);Ok(())},
 Command::Ingest{target,source,file}=>{let t:Target=target.parse()?;let e=match source{IngestSource::GenericJson=>ingest_jsonl(&file,&t)?,IngestSource::AccessLog=>ingest_access_log(&file,&t)?,IngestSource::OtelJson=>ingest_otel_json(&file,&t)?,IngestSource::Prometheus=>ingest_prometheus(&file,&t)?};println!("{}",serde_json::to_string_pretty(&e)?);Ok(())},
 Command::Analyze{target,root,evidence,output}=>analyze(&cli.config,&target,&root,&evidence,&output,cli.json),
 Command::Proof{file}=>{let p=load_proof(&file)?;if cli.json{print!("{}",p.canonical_json())}else{print!("{}",report::terminal(&p))}Ok(())},
 Command::Graph{file}=>{let p=load_proof(&file)?;println!("{}",serde_json::to_string_pretty(&p.result().graph)?);Ok(())},
 Command::Blockers{file}=>{let p=load_proof(&file)?;println!("{}",serde_json::to_string_pretty(&p.blockers)?);Ok(())},
 Command::Verify{target,evidence}=>{let t:Target=target.parse()?;let mut all=vec![];for f in evidence{all.extend(ingest_jsonl(&f,&t)?)}let result=verify::verify(&all);println!("{}",serde_json::to_string_pretty(&result)?);if result.verdict=="regression-detected"||result.verdict=="insufficient-evidence"{std::process::exit(2)}Ok(())},
 Command::Status{target,state_file}=>{let t:Target=target.parse()?;let mut store=LifecycleStore::load(&state_file)?;let record=store.record_for(&t,Utc::now()).clone();if cli.json{println!("{}",serde_json::to_string_pretty(&record)?)}else{println!("Target: {}\nLifecycle: {}\nUpdated: {}",record.target,record.state,record.updated_at)}Ok(())},
 Command::Lifecycle{target,set,note,state_file,proof}=>{let t:Target=target.parse()?;let mut store=LifecycleStore::load(&state_file)?;if let Some(raw)=set{let next:LifecycleState=raw.parse()?;if next==LifecycleState::Ready{let p=load_proof(&proof).context("READY requires a generated proof")?;if !matches!(p.verdict,decomproof_core::policy::Verdict::Ready|decomproof_core::policy::Verdict::ReadyWithUncertainty){anyhow::bail!("cannot enter READY: proof verdict is {:?}",p.verdict)}}store.transition(&t,next,Utc::now(),note)?;store.save(&state_file)?;}let record=store.record_for(&t,Utc::now());println!("{}",serde_json::to_string_pretty(record)?);Ok(())},
 Command::Demo{stage}=>{if stage>6{anyhow::bail!("demo stage must be 0..=6")}let root=PathBuf::from(format!("examples/atlascommerce/stages/stage-{stage}"));if stage>=5{let evidence=if stage==5{root.join("cleanup-leftovers.jsonl")}else{root.join("runtime.jsonl")};let t:Target="service:legacy-export".parse()?;println!("{}",serde_json::to_string_pretty(&verify::verify(&ingest_jsonl(&evidence,&t)?))?);Ok(())}else{analyze(&cli.config,"service:legacy-export",&root,&[root.join("runtime.jsonl")],&PathBuf::from("retirement.proof.json"),cli.json)}}}}
fn init(path:&Path)->Result<()>{if path.exists(){anyhow::bail!("{} already exists",path.display())}fs::write(path,"project:\n  name: my-project\npolicy:\n  runtime_minimum_window_days: 30\n  external_minimum_window_days: 45\n  evidence_maximum_age_hours: 24\n  scheduled_minimum_cycles: 3\n  unidentified_consumers_block: true\n  public_api_requires_deprecation: true\n  api_minimum_sunset_days: 30\n  data_zero_writes_days: 30\nprivacy:\n  hash_consumer_ids: true\n")?;println!("created {}",path.display());Ok(())}
fn doctor(path:&Path)->Result<()>{let _=Config::load(path).with_context(||format!("loading {}",path.display()))?;println!("configuration: ok\nnetwork integrations: disabled unless explicitly configured\nread-only analysis: enabled");Ok(())}
fn analyze(config_path:&Path,target:&str,root:&Path,evidence_files:&[PathBuf],output:&Path,json:bool)->Result<()>{let cfg=Config::load(config_path).unwrap_or_default();let t:Target=target.parse()?;let mut ev=scan_workspace(root,&t);for f in evidence_files{ev.extend(ingest_jsonl(f,&t)?)}let p=RetirementProof::analyze(t,ev,&cfg,Revision{commit:git(&["rev-parse","HEAD"]),branch:git(&["branch","--show-current"])},Utc::now());fs::write(output,p.canonical_json())?;if json{print!("{}",p.canonical_json())}else{print!("{}",report::terminal(&p))}if matches!(p.verdict,decomproof_core::policy::Verdict::Blocked|decomproof_core::policy::Verdict::InsufficientEvidence){std::process::exit(2)}Ok(())}
fn git(args:&[&str])->String{ProcessCommand::new("git").args(args).output().ok().filter(|o|o.status.success()).and_then(|o|String::from_utf8(o.stdout).ok()).map(|s|s.trim().to_string()).filter(|s|!s.is_empty()).unwrap_or_else(||"unknown".into())}
fn load_proof(path:&Path)->Result<RetirementProof>{Ok(serde_json::from_str(&fs::read_to_string(path)?)?)}
