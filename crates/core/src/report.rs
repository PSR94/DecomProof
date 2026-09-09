use crate::proof::RetirementProof;

pub fn markdown(p: &RetirementProof) -> String {
    let mut s=format!("# DecomProof Removal Gate\n\n**Target:** `{}`  \n**Readiness:** {}/100  \n**Verdict:** `{:?}`\n\n## Evidence matrix\n\n| Signal | Items |\n|---|---:|\n",p.target,p.score,p.verdict);
    for (k, v) in &p.signals {
        s.push_str(&format!("| `{}` | {} |\n", k, v));
    }
    s.push_str("\n## Blockers\n\n");
    if p.blockers.is_empty() {
        s.push_str("No known blockers.\n")
    } else {
        for b in &p.blockers {
            s.push_str(&format!("- **{:?}** {} — {}\n", b.severity, b.kind, b.message));
        }
    }
    s.push_str("\n## Uncertainty\n\n");
    if p.uncertainties.is_empty() {
        s.push_str("No recorded uncertainty above policy threshold.\n")
    } else {
        for u in &p.uncertainties {
            s.push_str(&format!("- **{}** {}\n", u.kind, u.message));
        }
    }
    s
}

pub fn terminal(p: &RetirementProof) -> String {
    format!("DecomProof\n\nTarget: {}\nRemoval Readiness: {} / 100\nVerdict: {:?}\n\nBlockers: {}\nUncertainties: {}\n",p.target,p.score,p.verdict,p.blockers.len(),p.uncertainties.len())
}

pub fn html(p: &RetirementProof) -> String {
    let blockers = if p.blockers.is_empty() {
        "<p>No known blockers.</p>".into()
    } else {
        format!(
            "<ul>{}</ul>",
            p.blockers
                .iter()
                .map(|b| format!(
                    "<li><strong>{:?}</strong> {} — {}</li>",
                    b.severity,
                    esc(&b.kind),
                    esc(&b.message)
                ))
                .collect::<String>()
        )
    };
    let uncertainty = if p.uncertainties.is_empty() {
        "<p>No recorded uncertainty above policy threshold.</p>".into()
    } else {
        format!(
            "<ul>{}</ul>",
            p.uncertainties
                .iter()
                .map(|u| format!("<li><strong>{}</strong> {}</li>", esc(&u.kind), esc(&u.message)))
                .collect::<String>()
        )
    };
    format!("<!doctype html><html><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>DecomProof {}</title><style>body{{font:15px system-ui;max-width:900px;margin:40px auto;padding:0 20px;color:#172033}}code{{font-family:ui-monospace,monospace}}.score{{font-size:42px;font-weight:750}}table{{border-collapse:collapse;width:100%}}td,th{{padding:8px;border-bottom:1px solid #d8dee9;text-align:left}}</style></head><body><h1>DecomProof Removal Gate</h1><p><code>{}</code></p><div class=\"score\">{}/100</div><p>Verdict: <strong>{:?}</strong></p><h2>Signals</h2><table><tr><th>Signal</th><th>Items</th></tr>{}</table><h2>Blockers</h2>{}<h2>Uncertainty</h2>{}<h2>Revision</h2><p><code>{}@{}</code></p></body></html>",esc(&p.target.stable_id()),esc(&p.target.stable_id()),p.score,p.verdict,p.signals.iter().map(|(k,v)|format!("<tr><td><code>{}</code></td><td>{}</td></tr>",esc(k),v)).collect::<String>(),blockers,uncertainty,esc(&p.revision.branch),esc(&p.revision.commit))
}
fn esc(raw: &str) -> String {
    raw.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}
