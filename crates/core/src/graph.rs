use crate::evidence::Evidence;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub kind: String,
    pub label: String,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub relation: String,
    pub evidence_id: String,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DependencyGraph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

pub fn build(target: &str, evidence: &[Evidence]) -> DependencyGraph {
    let mut nodes = BTreeSet::new();
    let mut edges = BTreeSet::new();
    nodes.insert(Node { id: target.into(), kind: "target".into(), label: target.into() });
    for e in evidence {
        if let Some(dep) = e.normalized.get("dependency").and_then(|v| v.as_str()) {
            nodes.insert(Node { id: dep.into(), kind: e.signal.clone(), label: dep.into() });
            edges.insert(Edge {
                from: dep.into(),
                to: target.into(),
                relation: e.signal.clone(),
                evidence_id: e.id.clone(),
            });
        }
    }
    DependencyGraph { nodes: nodes.into_iter().collect(), edges: edges.into_iter().collect() }
}
