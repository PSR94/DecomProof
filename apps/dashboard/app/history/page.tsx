import { EmptyState } from "@/components/empty-state";
import { Header } from "@/components/header";
import { listRuns } from "@/lib/api";

export default async function Page() {
  let runs: Awaited<ReturnType<typeof listRuns>> = [];
  try {
    runs = await listRuns();
  } catch {
    // Empty state is intentional when the optional API is unavailable.
  }

  return (
    <>
      <Header
        title="Historical candidates"
        subtitle="Previous analysis snapshots for trend and audit."
      />
      <div className="grid">
        <div className="card span12">
          {runs.length ? (
            <table className="table">
              <thead>
                <tr>
                  <th>Time</th>
                  <th>Target ID</th>
                  <th>Score</th>
                  <th>Verdict</th>
                  <th>Revision</th>
                </tr>
              </thead>
              <tbody>
                {runs.map((run) => (
                  <tr key={run.id}>
                    <td>{new Date(run.generated_at).toLocaleString()}</td>
                    <td>{run.target_id}</td>
                    <td>{run.score}</td>
                    <td>{run.verdict}</td>
                    <td className="mono">{run.revision_commit.slice(0, 10)}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          ) : (
            <EmptyState />
          )}
        </div>
      </div>
    </>
  );
}
