from typing import Annotated

from fastapi import APIRouter, Depends, HTTPException, Query, status
from sqlalchemy import desc, select
from sqlalchemy.orm import Session

from . import models, schemas, service
from .db import get_db

router = APIRouter(prefix="/api/v1")
DbSession = Annotated[Session, Depends(get_db)]


@router.post("/projects", response_model=schemas.ProjectView, status_code=status.HTTP_201_CREATED)
def create_project(payload: schemas.ProjectCreate, db: DbSession):
    existing = db.scalar(select(models.Project).where(models.Project.name == payload.name))
    if existing:
        raise HTTPException(409, "project already exists")
    project = models.Project(name=payload.name)
    db.add(project)
    db.commit()
    db.refresh(project)
    return project


@router.get("/projects", response_model=list[schemas.ProjectView])
def list_projects(
    db: DbSession,
    limit: int = Query(50, ge=1, le=200),
    offset: int = Query(0, ge=0),
):
    statement = select(models.Project).order_by(models.Project.name).offset(offset).limit(limit)
    return list(db.scalars(statement))


@router.get("/targets", response_model=list[schemas.TargetView])
def list_targets(
    db: DbSession,
    project_id: int | None = None,
    limit: int = Query(100, ge=1, le=500),
):
    statement = select(models.Target).order_by(models.Target.stable_id).limit(limit)
    if project_id is not None:
        statement = statement.where(models.Target.project_id == project_id)
    return list(db.scalars(statement))


@router.get("/targets/{target_id}", response_model=schemas.TargetView)
def get_target(target_id: int, db: DbSession):
    target = db.get(models.Target, target_id)
    if not target:
        raise HTTPException(404, "target not found")
    return target


@router.post(
    "/proofs/{project_name}",
    response_model=schemas.AnalysisView,
    status_code=status.HTTP_201_CREATED,
)
def upload_proof(project_name: str, proof: schemas.ProofEnvelope, db: DbSession):
    if proof.schema_ != "decomproof/v1":
        raise HTTPException(422, "unsupported proof schema")
    run = service.persist_proof(db, project_name, proof)
    return schemas.AnalysisView(
        id=run.id,
        target_id=run.target_id,
        generated_at=run.generated_at,
        revision_commit=run.revision_commit,
        revision_branch=run.revision_branch,
        verdict=run.verdict,
        score=run.score,
        policy_version=run.policy_version,
    )


@router.get("/analysis-runs", response_model=list[schemas.AnalysisView])
def list_runs(
    db: DbSession,
    target_id: int | None = None,
    limit: int = Query(50, ge=1, le=200),
):
    statement = (
        select(models.AnalysisRun)
        .order_by(desc(models.AnalysisRun.generated_at))
        .limit(limit)
    )
    if target_id is not None:
        statement = statement.where(models.AnalysisRun.target_id == target_id)
    rows = db.scalars(statement)
    return [
        schemas.AnalysisView(
            id=row.id,
            target_id=row.target_id,
            generated_at=row.generated_at,
            revision_commit=row.revision_commit,
            revision_branch=row.revision_branch,
            verdict=row.verdict,
            score=row.score,
            policy_version=row.policy_version,
        )
        for row in rows
    ]


@router.get("/analysis-runs/{run_id}/proof")
def get_proof(run_id: int, db: DbSession):
    run = db.get(models.AnalysisRun, run_id)
    if not run:
        raise HTTPException(404, "analysis run not found")
    return run.proof_json


@router.get("/analysis-runs/{run_id}/evidence")
def get_evidence(run_id: int, db: DbSession, signal: str | None = None):
    statement = (
        select(models.EvidenceItem)
        .where(models.EvidenceItem.analysis_id == run_id)
        .order_by(models.EvidenceItem.evidence_id)
    )
    if signal:
        statement = statement.where(models.EvidenceItem.signal == signal)
    return [item.payload for item in db.scalars(statement)]


@router.post("/targets/{target_id}/lifecycle")
def update_lifecycle(
    target_id: int,
    payload: schemas.LifecycleUpdate,
    db: DbSession,
):
    target = db.get(models.Target, target_id)
    if not target:
        raise HTTPException(404, "target not found")
    target.lifecycle_state = payload.state
    event = models.LifecycleEvent(
        target_id=target.id,
        state=payload.state,
        note=payload.note,
    )
    db.add(event)
    db.commit()
    return {"target_id": target.id, "state": target.lifecycle_state}


@router.get("/targets/{target_id}/history")
def history(target_id: int, db: DbSession):
    analyses = list(
        db.scalars(
            select(models.AnalysisRun)
            .where(models.AnalysisRun.target_id == target_id)
            .order_by(desc(models.AnalysisRun.generated_at))
        )
    )
    lifecycle = list(
        db.scalars(
            select(models.LifecycleEvent)
            .where(models.LifecycleEvent.target_id == target_id)
            .order_by(desc(models.LifecycleEvent.occurred_at))
        )
    )
    return {
        "analyses": [
            {
                "id": item.id,
                "at": item.generated_at,
                "score": item.score,
                "verdict": item.verdict,
            }
            for item in analyses
        ],
        "lifecycle": [
            {"at": item.occurred_at, "state": item.state, "note": item.note}
            for item in lifecycle
        ],
    }


@router.post(
    "/targets/{target_id}/verification",
    status_code=status.HTTP_201_CREATED,
)
def create_verification(
    target_id: int,
    payload: schemas.VerificationCreate,
    db: DbSession,
):
    if not db.get(models.Target, target_id):
        raise HTTPException(404, "target not found")
    run = models.VerificationRun(
        target_id=target_id,
        verdict=payload.verdict,
        payload=payload.payload,
    )
    db.add(run)
    db.commit()
    db.refresh(run)
    return {"id": run.id, "target_id": target_id, "verdict": run.verdict}
