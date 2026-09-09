export function EmptyState({ title = "No proof data yet", detail = "Run DecomProof analysis and upload the generated proof to the API." }: { title?: string; detail?: string }) {
  return <div className="empty"><strong>{title}</strong><p>{detail}</p></div>;
}

export function ApiError({ message }: { message: string }) {
  return <div className="empty error"><strong>API unavailable</strong><p>{message}</p><p className="mono">DECOMPROOF_API_URL</p></div>;
}
