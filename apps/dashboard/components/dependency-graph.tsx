"use client";

import { Background, Controls, ReactFlow, type Edge, type Node } from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import type { Proof } from "@/lib/types";

export function DependencyGraph({ proof }: { proof: Proof }) {
  const targetId = `${proof.target.kind}:${proof.target.id}`;
  const nodes: Node[] = [{ id: targetId, position: { x: 390, y: 180 }, data: { label: targetId }, style: { background: "#14251f", color: "#eef3f8", border: "1px solid #6ee7b7", borderRadius: 8 } }];
  const edges: Edge[] = [];
  proof.evidence.filter((e) => typeof e.normalized.dependency === "string").slice(0, 20).forEach((e, index) => {
    const dep = String(e.normalized.dependency); const id = `${e.id}:${index}`;
    nodes.push({ id, position: { x: 40 + (index % 4) * 210, y: 20 + Math.floor(index / 4) * 85 }, data: { label: dep }, style: { background: "#111c2e", color: "#cbd5e1", border: "1px solid #334155", borderRadius: 8, width: 180, fontSize: 11 } });
    edges.push({ id: e.id, source: id, target: targetId, label: e.signal, style: { stroke: "#64748b" }, labelStyle: { fill: "#94a3b8", fontSize: 9 } });
  });
  return <div style={{ height: 520 }}><ReactFlow nodes={nodes} edges={edges} fitView><Background/><Controls/></ReactFlow></div>;
}
