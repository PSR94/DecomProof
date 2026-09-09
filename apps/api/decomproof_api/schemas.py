from datetime import datetime
from typing import Any, Literal

from pydantic import BaseModel, ConfigDict, Field

Verdict = Literal[
    "blocked",
    "insufficient-evidence",
    "observing",
    "quiescent",
    "ready-with-uncertainty",
    "ready",
    "removed",
    "verified",
]


class ProjectCreate(BaseModel):
    name: str = Field(min_length=1, max_length=200)


class ProjectView(BaseModel):
    model_config = ConfigDict(from_attributes=True)
    id: int
    name: str
    created_at: datetime


class TargetView(BaseModel):
    model_config = ConfigDict(from_attributes=True)
    id: int
    project_id: int
    stable_id: str
    kind: str
    target_key: str
    fingerprint: str
    lifecycle_state: str


class ProofEnvelope(BaseModel):
    schema_: str = Field(alias="schema")
    target: dict[str, Any]
    target_fingerprint: str
    revision: dict[str, str]
    generated_at: datetime
    policy_version: str
    score: int = Field(ge=0, le=100)
    verdict: Verdict
    evidence: list[dict[str, Any]]
    blockers: list[dict[str, Any]]
    uncertainties: list[dict[str, Any]]

    model_config = ConfigDict(populate_by_name=True, extra="allow")


class AnalysisView(BaseModel):
    id: int
    target_id: int
    generated_at: datetime
    revision_commit: str
    revision_branch: str
    verdict: str
    score: int
    policy_version: str


class LifecycleUpdate(BaseModel):
    state: Literal[
        "ACTIVE",
        "DEPRECATED",
        "OBSERVING",
        "QUIESCENT",
        "READY",
        "REMOVED",
        "VERIFIED",
    ]
    note: str | None = Field(default=None, max_length=2000)


class VerificationCreate(BaseModel):
    verdict: Literal[
        "insufficient-evidence",
        "partial-cleanup",
        "regression-detected",
        "verified",
    ]
    payload: dict[str, Any]
