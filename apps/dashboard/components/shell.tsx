import Link from "next/link";
import type { ReactNode } from "react";

const links = [
  ["Overview", "/"], ["Projects", "/projects"], ["Decommission candidates", "/candidates"], ["Evidence matrix", "/evidence"],
  ["Blockers", "/blockers"], ["Uncertainty", "/uncertainty"], ["Runtime usage", "/runtime"], ["Dependency graph", "/graph"],
  ["Observation timeline", "/timeline"], ["Scheduled consumers", "/schedules"], ["Data evidence", "/data"], ["Events", "/events"],
  ["API / SDK contracts", "/contracts"], ["Infrastructure", "/infrastructure"], ["Retirement plan", "/retirement-plan"], ["Proof artifact", "/proof"],
  ["Post-removal verification", "/verification"], ["Historical candidates", "/history"], ["Configuration", "/configuration"], ["AtlasCommerce demo", "/atlascommerce"],
];

export function Shell({ children }: { children: ReactNode }) {
  return <div className="shell"><aside className="sidebar"><div className="brand">Decom<span>Proof</span></div><nav className="nav">{links.map(([label, href]) => <Link key={href} href={href}>{label}</Link>)}</nav></aside><main>{children}</main></div>;
}
