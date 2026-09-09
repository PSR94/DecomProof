from datetime import datetime

from sqlalchemy import DateTime, ForeignKey, Integer, JSON, String, Text, UniqueConstraint
from sqlalchemy.orm import Mapped, mapped_column, relationship

from .db import Base


class Project(Base):
    __tablename__ = "projects"
    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(String(200), unique=True, index=True)
    created_at: Mapped[datetime] = mapped_column(DateTime(timezone=True), default=datetime.utcnow)
    targets: Mapped[list["Target"]] = relationship(back_populates="project", cascade="all, delete-orphan")


class Target(Base):
    __tablename__ = "targets"
    __table_args__ = (UniqueConstraint("project_id", "stable_id"),)
    id: Mapped[int] = mapped_column(primary_key=True)
    project_id: Mapped[int] = mapped_column(ForeignKey("projects.id", ondelete="CASCADE"), index=True)
    stable_id: Mapped[str] = mapped_column(String(500), index=True)
    kind: Mapped[str] = mapped_column(String(80), index=True)
    target_key: Mapped[str] = mapped_column(String(420))
    fingerprint: Mapped[str] = mapped_column(String(80))
    lifecycle_state: Mapped[str] = mapped_column(String(40), default="ACTIVE")
    project: Mapped[Project] = relationship(back_populates="targets")
    analyses: Mapped[list["AnalysisRun"]] = relationship(back_populates="target", cascade="all, delete-orphan")


class AnalysisRun(Base):
    __tablename__ = "analysis_runs"
    id: Mapped[int] = mapped_column(primary_key=True)
    target_id: Mapped[int] = mapped_column(ForeignKey("targets.id", ondelete="CASCADE"), index=True)
    generated_at: Mapped[datetime] = mapped_column(DateTime(timezone=True), index=True)
    revision_commit: Mapped[str] = mapped_column(String(100))
    revision_branch: Mapped[str] = mapped_column(String(200))
    verdict: Mapped[str] = mapped_column(String(60), index=True)
    score: Mapped[int] = mapped_column(Integer)
    policy_version: Mapped[str] = mapped_column(String(100))
    proof_json: Mapped[dict] = mapped_column(JSON)
    target: Mapped[Target] = relationship(back_populates="analyses")
    evidence: Mapped[list["EvidenceItem"]] = relationship(back_populates="analysis", cascade="all, delete-orphan")


class EvidenceItem(Base):
    __tablename__ = "evidence_items"
    id: Mapped[int] = mapped_column(primary_key=True)
    analysis_id: Mapped[int] = mapped_column(ForeignKey("analysis_runs.id", ondelete="CASCADE"), index=True)
    evidence_id: Mapped[str] = mapped_column(String(80), index=True)
    signal: Mapped[str] = mapped_column(String(180), index=True)
    source: Mapped[str] = mapped_column(String(180), index=True)
    observed_at: Mapped[datetime] = mapped_column(DateTime(timezone=True), index=True)
    confidence: Mapped[str] = mapped_column(String(30))
    payload: Mapped[dict] = mapped_column(JSON)
    analysis: Mapped[AnalysisRun] = relationship(back_populates="evidence")


class LifecycleEvent(Base):
    __tablename__ = "lifecycle_events"
    id: Mapped[int] = mapped_column(primary_key=True)
    target_id: Mapped[int] = mapped_column(ForeignKey("targets.id", ondelete="CASCADE"), index=True)
    state: Mapped[str] = mapped_column(String(40), index=True)
    occurred_at: Mapped[datetime] = mapped_column(DateTime(timezone=True), default=datetime.utcnow)
    note: Mapped[str | None] = mapped_column(Text, nullable=True)


class VerificationRun(Base):
    __tablename__ = "verification_runs"
    id: Mapped[int] = mapped_column(primary_key=True)
    target_id: Mapped[int] = mapped_column(ForeignKey("targets.id", ondelete="CASCADE"), index=True)
    created_at: Mapped[datetime] = mapped_column(DateTime(timezone=True), default=datetime.utcnow)
    verdict: Mapped[str] = mapped_column(String(60), index=True)
    payload: Mapped[dict] = mapped_column(JSON)
