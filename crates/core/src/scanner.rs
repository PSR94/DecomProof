use crate::{evidence::{Confidence,Evidence},target::Target};
use chrono::Utc;
use regex::Regex;
use std::{fs,path::Path};
use walkdir::WalkDir;

pub fn scan_workspace(root:&Path,target:&Target)->Vec<Evidence>{
 let mut out=Vec::new(); let needle=target.id.to_ascii_lowercase();
 for entry in WalkDir::new(root).into_iter().filter_map(Result::ok).filter(|e|e.file_type().is_file()) {
  let path=entry.path(); if ignored(path){continue} let Ok(text)=fs::read_to_string(path) else{continue}; let lower=text.to_ascii_lowercase();
  if !lower.contains(&needle){continue}
  let rel=path.strip_prefix(root).unwrap_or(path).display().to_string();
  for (signal,quality) in classify(path,&text,target) {
   let mut ev=Evidence::build(&signal,"workspace-scanner",target,Utc::now(),Some(rel.clone()),text.as_bytes());ev.confidence=quality;ev.normalized.insert("dependency".into(),rel.clone().into());ev.normalized.insert("active".into(),true.into());out.push(ev);
  }
 }
 out.sort_by(|a,b|a.id.cmp(&b.id));out.dedup_by(|a,b|a.id==b.id);out
}
fn ignored(path:&Path)->bool{path.components().any(|c|matches!(c.as_os_str().to_str(),Some(".git"|"target"|"node_modules"|".next"|"dist")))}
fn classify(path:&Path,text:&str,target:&Target)->Vec<(String,Confidence)>{
 let mut s=Vec::new();let p=path.to_string_lossy().to_ascii_lowercase();
 if p.ends_with(".tf"){s.push(("infra.resource".into(),Confidence::High));}
 if p.ends_with(".yaml")||p.ends_with(".yml"){ if text.contains("kind: CronJob")||text.contains("schedule:"){s.push(("schedule.reference".into(),Confidence::High));} if text.contains("kind: Deployment")||text.contains("kind: Service")||text.contains("apiVersion:"){s.push(("infra.resource".into(),Confidence::Medium));} if text.contains("openapi:")||text.contains("swagger:"){s.push(("contract.public".into(),Confidence::High));} if p.contains(".github/workflows"){s.push(("ci.reference".into(),Confidence::High));}}
 if p.ends_with(".sql"){if Regex::new(r"(?i)\b(insert\s+into|update|delete\s+from)\b").unwrap().is_match(text){s.push(("database.write.static".into(),Confidence::Medium));}else{s.push(("database.reference".into(),Confidence::Medium));}}
 if p.ends_with(".py") && (text.contains("__all__")||text.contains("import ")){s.push(("sdk.export".into(),Confidence::Medium));}
 if p.ends_with(".ts")||p.ends_with(".tsx")||p.ends_with(".js")||p.ends_with(".jsx"){s.push(("static.reference".into(),Confidence::Medium));}
 if p.ends_with(".md")||p.ends_with(".mdx"){s.push(("documentation.reference".into(),Confidence::Low));}
 if target.kind=="event"||target.kind=="topic"{s.push(("event.reference".into(),Confidence::Medium));}
 if s.is_empty(){s.push(("static.text-reference".into(),Confidence::Low));}s
}

#[cfg(test)] mod tests {use super::*;use std::fs;#[test]fn finds_k8s_cron_dependency(){let d=tempfile::tempdir().unwrap();fs::write(d.path().join("cron.yaml"),"apiVersion: batch/v1\nkind: CronJob\nschedule: '0 2 * * *'\nargs: ['legacy-export']").unwrap();let t:Target="service:legacy-export".parse().unwrap();let e=scan_workspace(d.path(),&t);assert!(e.iter().any(|x|x.signal=="schedule.reference"));}}
