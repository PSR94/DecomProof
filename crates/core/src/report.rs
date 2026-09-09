use crate::proof::RetirementProof;

pub fn markdown(p:&RetirementProof)->String{let mut s=format!("# DecomProof Removal Gate\n\n**Target:** `{}`  \n**Readiness:** {}/100  \n**Verdict:** `{:?}`\n\n## Evidence matrix\n\n| Signal | Items |\n|---|---:|\n",p.target,p.score,p.verdict);for(k,v)in&p.signals{s.push_str(&format!("| `{}` | {} |\n",k,v));}s.push_str("\n## Blockers\n\n");if p.blockers.is_empty(){s.push_str("No known blockers.\n")}else{for b in&p.blockers{s.push_str(&format!("- **{:?}** {} — {}\n",b.severity,b.kind,b.message));}}s.push_str("\n## Uncertainty\n\n");if p.uncertainties.is_empty(){s.push_str("No recorded uncertainty above policy threshold.\n")}else{for u in&p.uncertainties{s.push_str(&format!("- **{}** {}\n",u.kind,u.message));}}s}

pub fn terminal(p:&RetirementProof)->String{format!("DecomProof\n\nTarget: {}\nRemoval Readiness: {} / 100\nVerdict: {:?}\n\nBlockers: {}\nUncertainties: {}\n",p.target,p.score,p.verdict,p.blockers.len(),p.uncertainties.len())}
