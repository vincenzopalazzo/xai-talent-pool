"""Job matching API router."""

import asyncio
import logging
from concurrent.futures import ThreadPoolExecutor

from fastapi import APIRouter

from grok_service.models import (
    JobMatchingRequest,
    JobMatchingResponse,
)
from grok_service.services.job_matching import JobMatchingService

logger = logging.getLogger(__name__)

router = APIRouter(prefix="/api/v1/matching", tags=["matching"])

# Thread pool for running sync code
_executor = ThreadPoolExecutor(max_workers=4)


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

        # Run synchronous xai_sdk code in a thread pool to not block
        loop = asyncio.get_event_loop()
        result = await asyncio.wait_for(
            loop.run_in_executor(
                _executor,
                service.match_candidates_to_job,
                request.job_id,
                request.job_title,
                request.job_description,
                request.company_name,
                request.skills_required,
                request.experience_level,
                request.candidates,
                request.top_n,
            ),
            timeout=240.0,  # 4 minute timeout
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

    except asyncio.TimeoutError:
        logger.error("JOB MATCHING API: Request timed out after 240 seconds")
        return JobMatchingResponse(
            success=False,
            result=None,
            error="Job matching timed out. The AI search is taking too long.",
        )

    except Exception as e:
        logger.error("JOB MATCHING API: Error: %s", e)
        return JobMatchingResponse(
            success=False,
            result=None,
            error=f"Job matching failed: {e}",
        )
