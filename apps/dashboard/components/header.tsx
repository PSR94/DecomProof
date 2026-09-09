import type { Proof, Target } from "@/lib/types";

export function Header({ title, subtitle, proof, target }: { title: string; subtitle: string; proof?: Proof; target?: Target }) {
  return <><div className="eyebrow">Decommission evidence</div><div className="split"><div><h1>{title}</h1><div className="muted">{subtitle}</div></div>{proof ? <span className={`status ${proof.verdict}`}>{proof.verdict}</span> : target ? <span className="status">{target.lifecycle_state}</span> : null}</div></>;
}
