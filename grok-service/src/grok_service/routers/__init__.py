"""API routers for Grok service."""

from grok_service.routers.candidate_research import router as candidate_research_router
from grok_service.routers.collections import router as collections_router
from grok_service.routers.screening import router as screening_router

__all__ = ["candidate_research_router", "collections_router", "screening_router"]
