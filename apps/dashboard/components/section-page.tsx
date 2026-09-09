import { latestProof } from "@/lib/api";
import type { Evidence } from "@/lib/types";
import { ApiError, EmptyState } from "./empty-state";
import { Header } from "./header";

export async function SectionPage({ title, subtitle, prefixes = [] }: { title: string; subtitle: string; prefixes?: string[] }) {
  const state = await latestProof();
  if (state.error) return <><Header title={title} subtitle={subtitle}/><div className="grid"><div className="span12"><ApiError message={state.error}/></div></div></>;
  if (!state.proof) return <><Header title={title} subtitle={subtitle} target={state.target}/><div className="grid"><div className="span12"><EmptyState/></div></div></>;
  const evidence = prefixes.length ? state.proof.evidence.filter((e) => prefixes.some((p) => e.signal.startsWith(p))) : state.proof.evidence;
  return <><Header title={title} subtitle={subtitle} proof={state.proof}/><div className="grid"><div className="card span12"><table className="table"><thead><tr><th>Signal</th><th>Source</th><th>Observed</th><th>Confidence</th><th>Artifact</th></tr></thead><tbody>{evidence.map((e: Evidence) => <tr key={e.id}><td className="signal">{e.signal}</td><td>{e.source}</td><td>{new Date(e.observed_at).toLocaleString()}</td><td>{e.confidence}</td><td className="mono">{e.artifact ?? "—"}</td></tr>)}</tbody></table>{evidence.length === 0 ? <EmptyState title="No matching evidence" detail="No evidence in the latest proof matches this source family."/> : null}</div></div></>;
}
