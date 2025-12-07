"""Tests for health check endpoint."""

from fastapi.testclient import TestClient

from grok_service.main import app


def test_health_check():
    """Test health check endpoint returns healthy status."""
    client = TestClient(app)
    response = client.get("/health")
    assert response.status_code == 200
    data = response.json()
    assert data["status"] == "healthy"
    assert "version" in data
