FROM python:3.12-slim AS runtime
ENV PYTHONDONTWRITEBYTECODE=1 PYTHONUNBUFFERED=1
WORKDIR /app
COPY pyproject.toml ./
RUN pip install --no-cache-dir .
COPY decomproof_api ./decomproof_api
RUN useradd --create-home --uid 10001 decomproof && chown -R decomproof:decomproof /app
USER decomproof
EXPOSE 8000
CMD ["uvicorn", "decomproof_api.main:app", "--host", "0.0.0.0", "--port", "8000"]
