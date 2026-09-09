import Link from "next/link";
import { latestProof, listTargets } from "@/lib/api";
import { ApiError, EmptyState } from "@/components/empty-state";
import { EvidenceMatrix } from "@/components/evidence-matrix";
import { Header } from "@/components/header";

export default async function Overview() {
  const state = await latestProof();
  if (state.error) return <><Header title="Overview" subtitle="Removal readiness across the latest collected evidence."/><div className="grid"><div className="span12"><ApiError message={state.error}/></div></div></>;
  if (!state.proof) return <><Header title="Overview" subtitle="Removal readiness across the latest collected evidence." target={state.target}/><div className="grid"><div className="span12"><EmptyState/></div></div></>;
  const p = state.proof;
  return <><Header title={state.target?.stable_id ?? `${p.target.kind}:${p.target.id}`} subtitle={`Generated ${new Date(p.generated_at).toLocaleString()} · ${p.revision.branch}@${p.revision.commit.slice(0, 10)}`} proof={p}/><div className="grid"><div className="card span4"><div className="muted">Readiness</div><div className="metric">{p.score}<span className="muted">/100</span></div><div className="bar"><span style={{width:`${p.score}%`}}/></div></div><div className="card span4"><div className="muted">Hard blockers</div><div className="metric">{p.blockers.filter(b=>b.hard).length}</div></div><div className="card span4"><div className="muted">Uncertainties</div><div className="metric">{p.uncertainties.length}</div></div><div className="card span8"><h2>Evidence matrix</h2><EvidenceMatrix proof={p}/></div><div className="card span4"><h2>Verdict rationale</h2>{p.verdict_rationale.map(x=><p key={x} className="muted">{x}</p>)}<Link className="button" href="/proof">Open proof artifact</Link></div></div></>;
}
