import type { AnalysisRun, Proof, Target } from "./types";

const API = process.env.DECOMPROOF_API_URL ?? "http://localhost:8000/api/v1";

async function get<T>(path: string): Promise<T> {
  const response = await fetch(`${API}${path}`, { cache: "no-store" });
  if (!response.ok) throw new Error(`DecomProof API ${response.status} for ${path}`);
  return response.json() as Promise<T>;
}

export const listTargets = () => get<Target[]>("/targets?limit=200");
export const getTarget = (id: number) => get<Target>(`/targets/${id}`);
export const listRuns = (targetId?: number) => get<AnalysisRun[]>(`/analysis-runs?limit=50${targetId ? `&target_id=${targetId}` : ""}`);
export const getProof = (runId: number) => get<Proof>(`/analysis-runs/${runId}/proof`);

export async function latestProof(targetId?: number): Promise<{ target?: Target; run?: AnalysisRun; proof?: Proof; error?: string }> {
  try {
    const targets = await listTargets();
    const target = targetId ? targets.find((t) => t.id === targetId) : targets[0];
    if (!target) return {};
    const runs = await listRuns(target.id);
    const run = runs[0];
    if (!run) return { target };
    return { target, run, proof: await getProof(run.id) };
  } catch (error) {
    return { error: error instanceof Error ? error.message : "API unavailable" };
  }
}
