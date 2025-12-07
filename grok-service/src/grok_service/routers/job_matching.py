"""Job matching API router."""

import logging

from fastapi import APIRouter

from grok_service.models import (
    JobMatchingRequest,
    JobMatchingResponse,
)
from grok_service.services.job_matching import JobMatchingService

logger = logging.getLogger(__name__)

router = APIRouter(prefix="/api/v1/matching", tags=["matching"])


@router.post("/job", response_model=JobMatchingResponse)
async def match_candidates_to_job(
    request: JobMatchingRequest,
) -> JobMatchingResponse:
    """
    Match candidates to a job posting.

    This endpoint searches across all candidate collections to find
    the best matches for a job. It returns ranked candidates with
    scores and explanations.

    Args:
        request: JobMatchingRequest with job info and candidate list

    Returns:
        JobMatchingResponse with ranked matches
    """
    logger.info("=" * 70)
    logger.info("JOB MATCHING API: Received request")
    logger.info("=" * 70)
    logger.info("Job ID: %s", request.job_id)
    logger.info("Job Title: %s", request.job_title)
    logger.info("Candidates: %d", len(request.candidates))
    logger.info("Top N: %d", request.top_n)
    logger.info("=" * 70)

    try:
        service = JobMatchingService()
        result = service.match_candidates_to_job(
            job_id=request.job_id,
            job_title=request.job_title,
            job_description=request.job_description,
            company_name=request.company_name,
            skills_required=request.skills_required,
            experience_level=request.experience_level,
            candidates=request.candidates,
            top_n=request.top_n,
        )

        logger.info("=" * 70)
        logger.info("JOB MATCHING API: Completed successfully")
        logger.info("Matches found: %d", len(result.matches))
        logger.info("=" * 70)

        return JobMatchingResponse(
            success=True,
            result=result,
            error=None,
        )

    except Exception as e:
        logger.error("JOB MATCHING API: Error: %s", e)
        return JobMatchingResponse(
            success=False,
            result=None,
            error=f"Job matching failed: {e}",
        )
