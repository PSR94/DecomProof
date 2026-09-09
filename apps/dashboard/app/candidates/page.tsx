import Link from "next/link";

import { ApiError, EmptyState } from "@/components/empty-state";
import { Header } from "@/components/header";
import { listTargets } from "@/lib/api";
import type { Target } from "@/lib/types";

async function loadCandidates(): Promise<{ targets: Target[]; error?: string }> {
  try {
    return { targets: await listTargets() };
  } catch (error) {
    return { targets: [], error: String(error) };
  }
}

export default async function Candidates() {
  const { targets, error } = await loadCandidates();

  return (
    <>
      <Header
        title="Decommission candidates"
        subtitle="Targets being observed through the retirement lifecycle."
      />
      {error ? (
        <ApiError message={error} />
      ) : (
        <div className="grid">
          <div className="card span12">
            {targets.length ? (
              <table className="table">
                <thead>
                  <tr>
                    <th>Target</th>
                    <th>Kind</th>
                    <th>Lifecycle</th>
                  </tr>
                </thead>
                <tbody>
                  {targets.map((target) => (
                    <tr key={target.id}>
                      <td>
                        <Link className="mono" href={`/candidates/${target.id}`}>
                          {target.stable_id}
                        </Link>
                      </td>
                      <td>{target.kind}</td>
                      <td>{target.lifecycle_state}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            ) : (
              <EmptyState />
            )}
          </div>
        </div>
      )}
    </>
  );
}
