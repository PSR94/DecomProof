from datetime import datetime

from sqlalchemy import select
from sqlalchemy.orm import Session

from . import models
from .schemas import ProofEnvelope


def get_or_create_project(db: Session, name: str) -> models.Project:
    project = db.scalar(select(models.Project).where(models.Project.name == name))
    if project:
        return project
    project = models.Project(name=name)
    db.add(project)
    db.flush()
    return project


def parse_dt(value: object) -> datetime:
    if isinstance(value, datetime):
        return value
    if isinstance(value, str):
        return datetime.fromisoformat(value.replace("Z", "+00:00"))
    raise ValueError("evidence observed_at must be an ISO datetime")


def persist_proof(db: Session, project_name: str, proof: ProofEnvelope) -> models.AnalysisRun:
    target_data = proof.target
    stable_id = f"{target_data['kind']}:{target_data['id']}"
    project = get_or_create_project(db, project_name)
    target = db.scalar(select(models.Target).where(models.Target.project_id == project.id, models.Target.stable_id == stable_id))
    if not target:
        target = models.Target(project_id=project.id, stable_id=stable_id, kind=str(target_data["kind"]), target_key=str(target_data["id"]), fingerprint=proof.target_fingerprint)
        db.add(target)
        db.flush()
    else:
        target.fingerprint = proof.target_fingerprint

    raw = proof.model_dump(by_alias=True, mode="json")
    run = models.AnalysisRun(target_id=target.id, generated_at=proof.generated_at, revision_commit=proof.revision.get("commit", "unknown"), revision_branch=proof.revision.get("branch", "unknown"), verdict=proof.verdict, score=proof.score, policy_version=proof.policy_version, proof_json=raw)
    db.add(run)
    db.flush()
    for item in proof.evidence:
        db.add(models.EvidenceItem(analysis_id=run.id, evidence_id=str(item.get("id", "unknown")), signal=str(item.get("signal", "unknown")), source=str(item.get("source", "unknown")), observed_at=parse_dt(item.get("observed_at")), confidence=str(item.get("confidence", "unknown")), payload=item))
    db.commit()
    db.refresh(run)
    return run
