import { ApiError, EmptyState } from "@/components/empty-state";
import { Header } from "@/components/header";
import { listTargets } from "@/lib/api";
import type { Target } from "@/lib/types";

async function loadProjects(): Promise<{ targets: Target[]; error?: string }> {
  try {
    return { targets: await listTargets() };
  } catch (error) {
    return { targets: [], error: String(error) };
  }
}

export default async function Projects() {
  const { targets, error } = await loadProjects();
  const projects = [...new Set(targets.map((target) => target.project_id))];

  return (
    <>
      <Header
        title="Projects"
        subtitle="Repositories and decommission targets grouped by project."
      />
      {error ? (
        <ApiError message={error} />
      ) : (
        <div className="grid">
          <div className="card span12">
            {projects.length ? (
              <table className="table">
                <thead>
                  <tr>
                    <th>Project ID</th>
                    <th>Targets</th>
                  </tr>
                </thead>
                <tbody>
                  {projects.map((id) => (
                    <tr key={id}>
                      <td>{id}</td>
                      <td>{targets.filter((target) => target.project_id === id).length}</td>
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
