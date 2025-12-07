"""API router for GRPO candidate ranking."""

import logging
from typing import List, Optional

from fastapi import APIRouter, Depends, HTTPException
from pydantic import BaseModel, Field
from xai_sdk import Client

from grok_service.config import get_settings
from grok_service.services.ranking import GRPORankingService

logger = logging.getLogger(__name__)

router = APIRouter(prefix="/api/v1/ranking", tags=["ranking"])


# Request/Response Models
class MatchFactorsResponse(BaseModel):
    skills_match: float = Field(..., ge=0.0, le=1.0)
    experience_match: float = Field(..., ge=0.0, le=1.0)
    location_match: float = Field(..., ge=0.0, le=1.0)
    title_match: float = Field(..., ge=0.0, le=1.0)
    overall_fit: float = Field(..., ge=0.0, le=1.0)


class RankedCandidateResponse(BaseModel):
    candidate: dict
    rank_score: float = Field(..., ge=0.0, le=1.0)
    rank_position: int = Field(..., ge=1)
    confidence: float = Field(..., ge=0.0, le=1.0)
    match_factors: MatchFactorsResponse
    feedback_score: Optional[float] = Field(None, ge=-1.0, le=1.0)


class RankCandidatesRequest(BaseModel):
    job: dict = Field(..., description="Job posting data")
    candidates: List[dict] = Field(..., description="List of candidate/talent data")
    feedback_data: Optional[List[dict]] = Field(None, description="RLHF feedback data")
    use_feedback: bool = Field(True, description="Whether to incorporate feedback")


class UpdateWeightsRequest(BaseModel):
    feedback_batch: List[dict] = Field(
        ..., description="Batch of feedback for weight updates"
    )


class RankingStatsResponse(BaseModel):
    total_candidates: int
    avg_confidence: float
    model_version: str
    current_weights: dict


# Dependency: Get ranking service
def get_ranking_service() -> GRPORankingService:
    """Create and return a GRPO ranking service instance."""
    settings = get_settings()
    xai_client = Client(api_key=settings.xai_api_key)
    return GRPORankingService(xai_client)


@router.post("/rank", response_model=List[RankedCandidateResponse])
async def rank_candidates(
    request: RankCandidatesRequest,
    ranking_service: GRPORankingService = Depends(get_ranking_service),
):
    """
    Rank candidates for a job using GRPO algorithm with RLHF feedback.

    This endpoint:
    1. Computes match scores based on skills, experience, location, title
    2. Applies RLHF feedback adjustments (upvotes/downvotes)
    3. Uses GRPO group-relative ranking to determine final order
    4. Returns ranked candidates with confidence scores
    """
    try:
        ranked = ranking_service.rank_candidates(
            job=request.job,
            candidates=request.candidates,
            feedback_data=request.feedback_data,
            use_feedback=request.use_feedback,
        )

        response = []
        for item in ranked:
            response.append(
                RankedCandidateResponse(
                    candidate=item["candidate"],
                    rank_score=item["rank_score"],
                    rank_position=item["rank_position"],
                    confidence=item["confidence"],
                    match_factors=MatchFactorsResponse(**item["match_factors"]),
                    feedback_score=item.get("feedback_score"),
                )
            )

        logger.info(
            f"Ranked {len(response)} candidates for job {request.job.get('id')} "
            f"(feedback: {request.use_feedback})"
        )

        return response

    except Exception as e:
        logger.error(f"Error ranking candidates: {e}", exc_info=True)
        raise HTTPException(status_code=500, detail=f"Ranking failed: {str(e)}")


@router.post("/update-weights")
async def update_weights(
    request: UpdateWeightsRequest,
    ranking_service: GRPORankingService = Depends(get_ranking_service),
):
    """
    Update GRPO model weights based on feedback batch.

    This endpoint implements the learning component of RLHF:
    - Adjusts feature weights based on which candidates received positive feedback
    - Updates the feedback weight parameter
    - Implements gradient descent to optimize ranking quality over time

    Use this endpoint after collecting a batch of recruiter feedback.
    """
    try:
        ranking_service.update_weights_from_feedback(request.feedback_batch)

        feedback_count = len(request.feedback_batch)
        return {
            "status": "success",
            "message": f"Updated weights from {feedback_count} feedback samples",
            "current_weights": ranking_service.weights,
            "model_version": ranking_service.model_version,
        }

    except Exception as e:
        logger.error(f"Error updating weights: {e}", exc_info=True)
        raise HTTPException(status_code=500, detail=f"Weight update failed: {str(e)}")


@router.get("/stats", response_model=RankingStatsResponse)
async def get_ranking_stats(
    ranking_service: GRPORankingService = Depends(get_ranking_service),
):
    """Get current ranking model statistics and configuration."""
    return RankingStatsResponse(
        total_candidates=0,  # Would come from external API
        avg_confidence=0.0,  # Would be computed from recent rankings
        model_version=ranking_service.model_version,
        current_weights=ranking_service.weights,
    )


@router.get("/health")
async def health_check():
    """Health check endpoint for ranking service."""
    return {"status": "healthy", "service": "grpo-ranking"}
