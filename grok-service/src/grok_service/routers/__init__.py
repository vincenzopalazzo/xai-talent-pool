"""API routers for Grok service."""

from grok_service.routers.collections import router as collections_router
from grok_service.routers.job_matching import router as job_matching_router
from grok_service.routers.ranking import router as ranking_router
from grok_service.routers.scoring import router as scoring_router
from grok_service.routers.screening import router as screening_router
from grok_service.routers.social_media import router as social_media_router

__all__ = [
    "collections_router",
    "job_matching_router",
    "ranking_router",
    "scoring_router",
    "screening_router",
    "social_media_router",
]
