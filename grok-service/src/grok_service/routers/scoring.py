"""Candidate scoring API router."""

import logging

from fastapi import APIRouter

from grok_service.models import (
    CandidateScoringRequest,
    CandidateScoringResponse,
)
from grok_service.services.scoring import CandidateScoringService

logger = logging.getLogger(__name__)

router = APIRouter(prefix="/api/v1/scoring", tags=["scoring"])


@router.post("/candidate", response_model=CandidateScoringResponse)
async def score_candidate(
    request: CandidateScoringRequest,
) -> CandidateScoringResponse:
    """
    Score a candidate's fit for a specific job.

    This endpoint uses Grok AI with the collections_search tool to analyze
    all documents in the candidate's collection (resume, social media analysis,
    etc.) and generate a comprehensive fit score.

    The scoring considers:
    - Skills match: How well the candidate's skills align with requirements
    - Experience fit: Relevance and level of experience
    - Culture fit: Alignment with xAI's values and work style
    - Overall impression: Holistic assessment of the candidate

    Returns a recommendation (strong_yes, yes, maybe, no) along with
    detailed breakdown and key strengths/concerns.

    Args:
        request: CandidateScoringRequest with talent, job, and collection info

    Returns:
        CandidateScoringResponse with scoring result or error
    """
    logger.info("=" * 70)
    logger.info("SCORING API: Received request")
    logger.info("=" * 70)
    logger.info("Talent ID: %s", request.talent_id)
    logger.info("Collection ID: %s", request.collection_id)
    logger.info("Job ID: %s", request.job.id)
    logger.info("Job Title: %s", request.job.title)
    logger.info("Candidate: %s", request.candidate_name)
    logger.info("=" * 70)

    try:
        service = CandidateScoringService()
        result = service.score_candidate(
            talent_id=request.talent_id,
            collection_id=request.collection_id,
            job=request.job,
            candidate_name=request.candidate_name,
            candidate_title=request.candidate_title,
            candidate_skills=request.candidate_skills,
        )

        logger.info("=" * 70)
        logger.info("SCORING API: Completed successfully")
        logger.info("Score: %.1f", result.overall_score)
        logger.info("Recommendation: %s", result.recommendation)
        logger.info("=" * 70)

        return CandidateScoringResponse(
            success=True,
            result=result,
            error=None,
        )

    except Exception as e:
        logger.error("SCORING API: Error: %s", e)
        return CandidateScoringResponse(
            success=False,
            result=None,
            error=f"Candidate scoring failed: {e}",
        )
