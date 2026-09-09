import type { Proof } from "@/lib/types";

const families = ["static", "runtime", "schedule", "database", "event", "contract", "sdk", "infra"];

export function EvidenceMatrix({ proof }: { proof: Proof }) {
  return <table className="table"><thead><tr><th>Family</th><th>Status</th><th>Items</th><th>Freshest</th><th>Window</th></tr></thead><tbody>{families.map((family) => {
    const items = proof.evidence.filter((e) => e.signal.startsWith(family));
    const blocked = proof.blockers.some((b) => b.evidence_ids.some((id) => items.some((e) => e.id === id)));
    const freshest = items.map((e) => e.observed_at).sort().at(-1);
    const windows = items.flatMap((e) => e.observation_window ? [Math.floor((Date.parse(e.observation_window.end)-Date.parse(e.observation_window.start))/86400000)] : []);
    return <tr key={family}><td className="signal">{family}</td><td className={blocked ? "severity-high" : ""}>{items.length ? (blocked ? "BLOCKED" : "CLEAR") : "NO EVIDENCE"}</td><td>{items.length}</td><td>{freshest ? new Date(freshest).toLocaleString() : "—"}</td><td>{windows.length ? `${Math.max(...windows)}d` : "current"}</td></tr>;
  })}</tbody></table>;
}
