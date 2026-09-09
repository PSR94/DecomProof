from fastapi.testclient import TestClient

from decomproof_api.main import app

client = TestClient(app)


def test_health():
    response = client.get("/healthz")
    assert response.status_code == 200
    assert response.json() == {"status": "ok"}


def test_project_round_trip():
    response = client.post("/api/v1/projects", json={"name": "atlas-test"})
    assert response.status_code in (201, 409)
    listed = client.get("/api/v1/projects").json()
    assert any(project["name"] == "atlas-test" for project in listed)


def test_rejects_unknown_proof_schema():
    proof = {
        "schema": "decomproof/v999",
        "target": {"kind": "service", "id": "x"},
        "target_fingerprint": "sha256:" + "0" * 64,
        "revision": {"commit": "abc", "branch": "main"},
        "generated_at": "2026-09-09T20:00:00Z",
        "policy_version": "decomproof-policy/v1",
        "score": 0,
        "verdict": "blocked",
        "evidence": [],
        "blockers": [],
        "uncertainties": [],
    }
    assert client.post("/api/v1/proofs/test", json=proof).status_code == 422
