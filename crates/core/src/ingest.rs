use crate::{
    evidence::{Confidence, Consumer, ConsumerIdentity, Evidence, ObservationWindow},
    target::Target,
};
use chrono::{DateTime, Utc};
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use std::{collections::BTreeMap, fs, path::Path};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IngestError {
    #[error("read evidence: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid json line {line}: {source}")]
    Json { line: usize, source: serde_json::Error },
    #[error("invalid OTLP JSON: {0}")]
    Otel(serde_json::Error),
}
#[derive(Debug, Deserialize)]
struct Generic {
    signal: String,
    #[serde(default = "source")]
    source: String,
    observed_at: DateTime<Utc>,
    #[serde(default)]
    window_start: Option<DateTime<Utc>>,
    #[serde(default)]
    window_end: Option<DateTime<Utc>>,
    #[serde(default)]
    count: Option<u64>,
    #[serde(default)]
    active: Option<bool>,
    #[serde(default)]
    cycles: Option<u64>,
    #[serde(default)]
    consumer_type: Option<String>,
    #[serde(default)]
    consumer_id: Option<String>,
    #[serde(default)]
    consumer_identification: Option<String>,
    #[serde(flatten)]
    extra: BTreeMap<String, serde_json::Value>,
}
fn source() -> String {
    "generic-json".into()
}

pub fn ingest_jsonl(path: &Path, target: &Target) -> Result<Vec<Evidence>, IngestError> {
    let raw = fs::read_to_string(path)?;
    let mut out = Vec::new();
    for (i, line) in raw.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let row: Generic =
            serde_json::from_str(line).map_err(|source| IngestError::Json { line: i + 1, source })?;
        let mut ev = Evidence::build(
            &row.signal,
            &row.source,
            target,
            row.observed_at,
            Some(path.display().to_string()),
            line.as_bytes(),
        );
        ev.confidence = Confidence::High;
        if let (Some(a), Some(b)) = (row.window_start, row.window_end) {
            ev.observation_window = Some(ObservationWindow { start: a, end: b, gaps_seconds: 0 });
        }
        if let Some(c) = row.count {
            ev.normalized.insert("count".into(), c.into());
        }
        if let Some(a) = row.active {
            ev.normalized.insert("active".into(), a.into());
        }
        if let Some(c) = row.cycles {
            ev.normalized.insert("cycles".into(), c.into());
        }
        for (k, v) in row.extra {
            ev.observed.insert(k, v);
        }
        if let Some(id) = row.consumer_id {
            let ident = match row.consumer_identification.as_deref() {
                Some("identified") => ConsumerIdentity::Identified,
                Some("partial") => ConsumerIdentity::Partial,
                _ => ConsumerIdentity::Unknown,
            };
            ev.consumer = Some(Consumer {
                identity: ident,
                consumer_type: row.consumer_type.unwrap_or_else(|| "unknown".into()),
                id: hash_identifier(&id),
            });
        }
        out.push(ev);
    }
    Ok(out)
}

pub fn ingest_access_log(path: &Path, target: &Target) -> Result<Vec<Evidence>, IngestError> {
    let raw = fs::read_to_string(path)?;
    let common =
        Regex::new(r#"^(\S+) \S+ \S+ \[([^\]]+)\] \"(\S+) ([^ ]+) [^\"]+\" (\d{3})"#).expect("valid regex");
    let mut matched = Vec::new();
    for line in raw.lines() {
        if !line.to_ascii_lowercase().contains(&target.id.to_ascii_lowercase()) {
            continue;
        }
        if let Some(c) = common.captures(line) {
            let ts = DateTime::parse_from_str(&c[2], "%d/%b/%Y:%H:%M:%S %z")
                .map(|x| x.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            matched.push((
                ts,
                c[1].to_string(),
                c[3].to_string(),
                c[4].to_string(),
                c[5].to_string(),
                line.to_string(),
            ));
        } else if let Ok(v) = serde_json::from_str::<Value>(line) {
            let path_value =
                v.get("path").or_else(|| v.get("url.path")).and_then(Value::as_str).unwrap_or("");
            if path_value.to_ascii_lowercase().contains(&target.id.to_ascii_lowercase()) {
                let ts = v
                    .get("timestamp")
                    .and_then(Value::as_str)
                    .and_then(|s| s.parse::<DateTime<Utc>>().ok())
                    .unwrap_or_else(Utc::now);
                let consumer = v
                    .get("service.name")
                    .or_else(|| v.get("client_id"))
                    .and_then(Value::as_str)
                    .unwrap_or("unknown")
                    .to_string();
                matched.push((
                    ts,
                    consumer,
                    v.get("method").and_then(Value::as_str).unwrap_or("UNKNOWN").into(),
                    path_value.into(),
                    v.get("status").map(|x| x.to_string()).unwrap_or_default(),
                    line.into(),
                ));
            }
        }
    }
    if matched.is_empty() {
        return Ok(vec![]);
    }
    matched.sort_by_key(|x| x.0);
    let start = matched.first().unwrap().0;
    let end = matched.last().unwrap().0;
    let consumer = &matched[0].1;
    let mut ev = Evidence::build(
        "runtime.http.requests",
        "access-log",
        target,
        end,
        Some(path.display().to_string()),
        raw.as_bytes(),
    );
    ev.confidence = Confidence::High;
    ev.observation_window = Some(ObservationWindow { start, end, gaps_seconds: 0 });
    ev.normalized.insert("count".into(), (matched.len() as u64).into());
    ev.normalized.insert("active".into(), true.into());
    ev.observed
        .insert("methods".into(), Value::Array(matched.iter().map(|x| Value::String(x.2.clone())).collect()));
    ev.consumer = Some(Consumer {
        identity: if consumer == "unknown" { ConsumerIdentity::Unknown } else { ConsumerIdentity::Partial },
        consumer_type: "access-log-client".into(),
        id: hash_identifier(consumer),
    });
    Ok(vec![ev])
}

pub fn ingest_otel_json(path: &Path, target: &Target) -> Result<Vec<Evidence>, IngestError> {
    let raw = fs::read_to_string(path)?;
    let root: Value = serde_json::from_str(&raw).map_err(IngestError::Otel)?;
    let mut hits: Vec<(DateTime<Utc>, String)> = Vec::new();
    let resources = root.get("resourceSpans").and_then(Value::as_array).cloned().unwrap_or_default();
    for rs in resources {
        let service = resource_attr(&rs, "service.name").unwrap_or_else(|| "unknown".into());
        for scope in rs.get("scopeSpans").and_then(Value::as_array).into_iter().flatten() {
            for span in scope.get("spans").and_then(Value::as_array).into_iter().flatten() {
                let hay = serde_json::to_string(span).unwrap_or_default().to_ascii_lowercase();
                if !hay.contains(&target.id.to_ascii_lowercase()) {
                    continue;
                }
                let ts = span
                    .get("startTimeUnixNano")
                    .and_then(Value::as_str)
                    .and_then(|s| s.parse::<i64>().ok())
                    .and_then(|n| {
                        DateTime::<Utc>::from_timestamp(n / 1_000_000_000, (n % 1_000_000_000) as u32)
                    })
                    .unwrap_or_else(Utc::now);
                hits.push((ts, service.clone()));
            }
        }
    }
    if hits.is_empty() {
        return Ok(vec![]);
    }
    hits.sort_by_key(|x| x.0);
    let mut ev = Evidence::build(
        "runtime.otel.spans",
        "otel-json",
        target,
        hits.last().unwrap().0,
        Some(path.display().to_string()),
        raw.as_bytes(),
    );
    ev.confidence = Confidence::High;
    ev.observation_window = Some(ObservationWindow {
        start: hits.first().unwrap().0,
        end: hits.last().unwrap().0,
        gaps_seconds: 0,
    });
    ev.normalized.insert("count".into(), (hits.len() as u64).into());
    ev.normalized.insert("active".into(), true.into());
    let consumer = &hits[0].1;
    ev.consumer = Some(Consumer {
        identity: if consumer == "unknown" {
            ConsumerIdentity::Unknown
        } else {
            ConsumerIdentity::Identified
        },
        consumer_type: "service.name".into(),
        id: hash_identifier(consumer),
    });
    Ok(vec![ev])
}

pub fn ingest_prometheus(path: &Path, target: &Target) -> Result<Vec<Evidence>, IngestError> {
    let raw = fs::read_to_string(path)?;
    let mut count = 0f64;
    for line in raw.lines() {
        let l = line.trim();
        if l.starts_with('#') || !l.to_ascii_lowercase().contains(&target.id.to_ascii_lowercase()) {
            continue;
        }
        if let Some(value) = l.split_whitespace().last().and_then(|v| v.parse::<f64>().ok()) {
            count += value;
        }
    }
    if count == 0.0 {
        return Ok(vec![]);
    }
    let mut ev = Evidence::build(
        "metric.usage",
        "prometheus-text",
        target,
        Utc::now(),
        Some(path.display().to_string()),
        raw.as_bytes(),
    );
    ev.confidence = Confidence::Medium;
    ev.normalized.insert("count".into(), serde_json::json!(count));
    ev.normalized.insert("active".into(), true.into());
    Ok(vec![ev])
}

fn resource_attr(rs: &Value, key: &str) -> Option<String> {
    rs.get("resource")?
        .get("attributes")?
        .as_array()?
        .iter()
        .find(|a| a.get("key").and_then(Value::as_str) == Some(key))
        .and_then(|a| a.get("value")?.get("stringValue")?.as_str().map(str::to_string))
}
pub fn hash_identifier(raw: &str) -> String {
    use sha2::{Digest, Sha256};
    format!("sha256:{}", &hex::encode(Sha256::digest(raw.as_bytes()))[..20])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn access_log_hashes_consumer() {
        let d = tempfile::tempdir().unwrap();
        let f = d.path().join("access.log");
        fs::write(&f, "10.0.0.7 - - [09/Sep/2026:20:00:00 +0000] \"POST /legacy-export HTTP/1.1\" 200 10\n")
            .unwrap();
        let t: Target = "service:legacy-export".parse().unwrap();
        let e = ingest_access_log(&f, &t).unwrap();
        assert!(e[0].consumer.as_ref().unwrap().id.starts_with("sha256:"));
    }
}
