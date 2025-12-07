"""Social media analysis API router."""

import logging

from fastapi import APIRouter

from grok_service.models import (
    SocialMediaAnalysisRequest,
    SocialMediaAnalysisResponse,
)
from grok_service.services.social_media import SocialMediaService

logger = logging.getLogger(__name__)

router = APIRouter(prefix="/api/v1/social", tags=["social-media"])


@router.post("/analyze", response_model=SocialMediaAnalysisResponse)
async def analyze_social_media(
    request: SocialMediaAnalysisRequest,
) -> SocialMediaAnalysisResponse:
    """
    Analyze social media profiles for a candidate.

    Uses Grok AI to search and analyze the candidate's presence across
    multiple social media platforms (X, GitHub, LinkedIn, GitLab, StackOverflow).

    The analysis extracts:
    - Profile handles and URLs
    - Skills and expertise signals
    - Experience/seniority indicators
    - Key recruiter takeaways

    The X handle (if found) is returned as the recommended system nickname.

    Args:
        request: SocialMediaAnalysisRequest with candidate info and URLs

    Returns:
        SocialMediaAnalysisResponse with profile analyses and X handle
    """
    logger.info("=" * 70)
    logger.info("SOCIAL MEDIA ANALYSIS: Received request")
    logger.info("=" * 70)
    logger.info("Talent ID: %s", request.talent_id)
    logger.info("Name: %s", request.name)
    logger.info("Email: %s", request.email or "Not provided")
    logger.info("URLs provided:")
    logger.info("  X:            %s", request.social_urls.x or "Not provided")
    logger.info("  GitHub:       %s", request.social_urls.github or "Not provided")
    logger.info("  LinkedIn:     %s", request.social_urls.linkedin or "Not provided")
    logger.info("  GitLab:       %s", request.social_urls.gitlab or "Not provided")
    logger.info(
        "  StackOverflow: %s", request.social_urls.stackoverflow or "Not provided"
    )
    logger.info("Platforms to search: %s", request.platforms_to_search)
    logger.info("=" * 70)

    try:
        service = SocialMediaService()
        result = service.analyze_profiles(
            talent_id=request.talent_id,
            name=request.name,
            email=request.email,
            social_urls=request.social_urls,
            platforms_to_search=request.platforms_to_search,
            collection_id=request.collection_id,
        )
    except Exception as e:
        logger.error("SOCIAL MEDIA ANALYSIS: Error: %s", e)
        return SocialMediaAnalysisResponse(
            success=False,
            result=None,
            error=f"Social media analysis failed: {e}",
        )

    # Log results
    logger.info("=" * 70)
    logger.info("SOCIAL MEDIA ANALYSIS: Results")
    logger.info("=" * 70)
    logger.info("X Handle: %s", result.x_handle or "Not found")
    logger.info("Profiles analyzed: %d", len(result.profiles))
    for profile in result.profiles:
        logger.info("  %s:", profile.platform)
        logger.info("    Handle: %s", profile.handle or "Unknown")
        logger.info(
            "    Skills: %s",
            ", ".join(profile.skills[:5]) if profile.skills else "None",
        )
        logger.info("    Notes: %d", len(profile.recruiter_notes))
    logger.info("Combined skills: %d", len(result.combined_skills))
    logger.info(
        "Summary: %s",
        (
            (result.summary[:100] + "...")
            if result.summary and len(result.summary) > 100
            else result.summary
        ),
    )
    logger.info("=" * 70)

    return SocialMediaAnalysisResponse(
        success=True,
        result=result,
        error=None,
    )
