"""Main FastAPI application for Grok microservice."""

from dotenv import load_dotenv
from fastapi import FastAPI

from grok_service import __version__
from grok_service.routers import screening_router

# Load .env file
load_dotenv()

app = FastAPI(
    title="Grok Service",
    description="Grok microservice for X Talent Pool",
    version=__version__,
    docs_url="/docs",
    redoc_url="/redoc",
    openapi_url="/api/v1/openapi.json",
)

# Include routers
app.include_router(screening_router)


@app.get("/health")
async def health_check() -> dict[str, str]:
    """Health check endpoint."""
    return {"status": "healthy", "version": __version__}
