FROM python:3.12-slim
WORKDIR /app
COPY pyproject.toml ./
RUN pip install --no-cache-dir .
COPY atlascommerce ./atlascommerce
RUN useradd --create-home --uid 10001 atlas && mkdir -p /data && chown -R atlas:atlas /app /data
USER atlas
EXPOSE 8080
CMD ["uvicorn", "atlascommerce.main:app", "--host", "0.0.0.0", "--port", "8080"]
