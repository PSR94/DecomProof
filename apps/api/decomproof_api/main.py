import uuid
from contextlib import asynccontextmanager

from fastapi import FastAPI, Request
from fastapi.responses import JSONResponse
from sqlalchemy.exc import SQLAlchemyError

from . import models
from .db import engine
from .routes import router


@asynccontextmanager
async def lifespan(_app: FastAPI):
    # Production deployments should apply migrations. create_all is non-destructive
    # and keeps local SQLite evaluation and tests frictionless.
    models.Base.metadata.create_all(bind=engine)
    yield


app = FastAPI(
    title="DecomProof API",
    version="0.1.0",
    docs_url="/docs",
    lifespan=lifespan,
)
app.include_router(router)


@app.middleware("http")
async def request_id(request: Request, call_next):
    request_id = request.headers.get("x-request-id", str(uuid.uuid4()))
    response = await call_next(request)
    response.headers["x-request-id"] = request_id
    return response


@app.get("/healthz")
def healthz():
    return {"status": "ok"}


@app.get("/readyz")
def readyz():
    try:
        with engine.connect() as conn:
            conn.exec_driver_sql("SELECT 1")
        return {"status": "ready"}
    except SQLAlchemyError as exc:
        # Do not leak connection details from readiness failures.
        return JSONResponse(
            status_code=503,
            content={"status": "not-ready", "reason": type(exc).__name__},
        )
