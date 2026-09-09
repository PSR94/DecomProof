import uuid

from fastapi import FastAPI, Request
from fastapi.responses import JSONResponse
from sqlalchemy.exc import SQLAlchemyError

from . import models
from .db import engine
from .routes import router

app = FastAPI(title="DecomProof API", version="0.1.0", docs_url="/docs")
app.include_router(router)


@app.on_event("startup")
def create_dev_schema() -> None:
    # Production deployments should apply migrations. This keeps local SQLite evaluation frictionless.
    models.Base.metadata.create_all(bind=engine)


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
