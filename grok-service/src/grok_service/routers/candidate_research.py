"""API router for candidate research endpoints."""

import logging
from datetime import datetime, timezone

from fastapi import APIRouter, HTTPException
from pydantic import BaseModel, Field

from grok_service.services.candidate_research import (
    CandidateResearchService,
    Platform,
    PlatformReport,
)
from grok_service.services.collections import CollectionService

logger = logging.getLogger(__name__)

router = APIRouter(prefix="/api/v1/candidate-research", tags=["Candidate Research"])


class ResearchRequest(BaseModel):
    """Request model for candidate research."""

    name: str = Field(
        ..., description="Full name of the person to research", min_length=1
    )
    email: str | None = Field(
        None, description="Email address (optional, helps with disambiguation)"
    )
    profile_url: str | None = Field(
        None, description="Direct profile URL for accurate research (optional)"
    )
    # Optional collection storage
    collection_id: str | None = Field(
        None, description="xAI Collection ID to store the report (optional)"
    )
    old_document_id: str | None = Field(
        None, description="Old document ID to replace when updating (optional)"
    )


class ResearchResponse(BaseModel):
    """Response model for candidate research."""

    success: bool = Field(..., description="Whether the research was successful")
    report: PlatformReport | None = Field(None, description="Research report")
    document_id: str | None = Field(
        None, description="Document ID if stored in collection"
    )
    error: str | None = Field(None, description="Error message if failed")


class PlatformInfo(BaseModel):
    """Information about a supported platform."""

    id: str = Field(..., description="Platform identifier")
    name: str = Field(..., description="Display name")
    focus: str = Field(..., description="What this platform focuses on")


class PlatformsResponse(BaseModel):
    """Response listing supported platforms."""

    platforms: list[PlatformInfo] = Field(
        ..., description="List of supported platforms"
    )


class AllPlatformsResponse(BaseModel):
    """Response for research across all platforms."""

    success: bool = Field(..., description="Whether the research was successful")
    reports: list[PlatformReport] = Field(
        default_factory=list, description="Research reports for each platform"
    )
    error: str | None = Field(None, description="Error message if failed")


@router.get("/platforms", response_model=PlatformsResponse)
async def get_platforms() -> PlatformsResponse:
    """Get list of supported platforms for candidate research."""
    platforms = CandidateResearchService.get_supported_platforms()
    return PlatformsResponse(platforms=[PlatformInfo(**p) for p in platforms])


@router.post("/github", response_model=ResearchResponse)
async def research_github(request: ResearchRequest) -> ResearchResponse:
    """Research a candidate on GitHub."""
    return await _search_platform(
        request.name,
        Platform.GITHUB,
        request.email,
        request.profile_url,
        request.collection_id,
        request.old_document_id,
    )


@router.post("/linkedin", response_model=ResearchResponse)
async def research_linkedin(request: ResearchRequest) -> ResearchResponse:
    """Research a candidate on LinkedIn."""
    return await _search_platform(
        request.name,
        Platform.LINKEDIN,
        request.email,
        request.profile_url,
        request.collection_id,
        request.old_document_id,
    )


@router.post("/twitter", response_model=ResearchResponse)
async def research_twitter(request: ResearchRequest) -> ResearchResponse:
    """Research a candidate on Twitter/X."""
    return await _search_platform(
        request.name,
        Platform.TWITTER,
        request.email,
        request.profile_url,
        request.collection_id,
        request.old_document_id,
    )


@router.post("/stackoverflow", response_model=ResearchResponse)
async def research_stackoverflow(request: ResearchRequest) -> ResearchResponse:
    """Research a candidate on Stack Overflow."""
    return await _search_platform(
        request.name,
        Platform.STACKOVERFLOW,
        request.email,
        request.profile_url,
        request.collection_id,
        request.old_document_id,
    )


@router.post("/{platform}", response_model=ResearchResponse)
async def research_platform(
    platform: str, request: ResearchRequest
) -> ResearchResponse:
    """
    Research a candidate on a specific platform.

    Platform must be one of: github, linkedin, twitter, stackoverflow
    """
    try:
        platform_enum = Platform(platform.lower())
    except ValueError:
        valid = "github, linkedin, twitter, stackoverflow"
        raise HTTPException(
            status_code=400,
            detail=f"Invalid platform: {platform}. Must be one of: {valid}",
        )

    return await _search_platform(
        request.name,
        platform_enum,
        request.email,
        request.profile_url,
        request.collection_id,
        request.old_document_id,
    )


@router.post("", response_model=AllPlatformsResponse)
async def research_all_platforms(request: ResearchRequest) -> AllPlatformsResponse:
    """
    Research a candidate across all supported platforms.

    Note: This endpoint may take a while as it queries all platforms sequentially.
    Consider using individual platform endpoints for faster results.
    """
    try:
        logger.info("Starting research for %s across all platforms", request.name)
        service = CandidateResearchService()
        reports = service.search_all_platforms(request.name, request.email)

        logger.info(
            "Completed research for %s - %d reports generated",
            request.name,
            len(reports),
        )

        return AllPlatformsResponse(success=True, reports=reports)

    except Exception as e:
        logger.exception("Failed to research candidate across all platforms: %s", e)
        return AllPlatformsResponse(success=False, error=str(e))


async def _search_platform(
    person_name: str,
    platform: Platform,
    email: str | None = None,
    profile_url: str | None = None,
    collection_id: str | None = None,
    old_document_id: str | None = None,
) -> ResearchResponse:
    """Search a specific platform, optionally storing in collection."""
    try:
        logger.info("Starting %s research for: %s", platform.value, person_name)
        if profile_url:
            logger.info("Using profile URL: %s", profile_url)
        service = CandidateResearchService()
        report = service.search_platform(person_name, platform, email, profile_url)

        # Check if report indicates an error (TLDR starts with "Error:")
        is_error_report = report.tldr.startswith("Error:") if report.tldr else False
        if is_error_report:
            logger.warning(
                "%s research returned error report for %s: %s",
                platform.value,
                person_name,
                report.tldr,
            )
            # Return as failure so Rust client knows research failed
            return ResearchResponse(
                success=False,
                report=report,  # Still include report for error details
                error=report.tldr,
            )

        logger.info("Completed %s research for: %s", platform.value, person_name)

        document_id = None

        # If collection_id provided, store the report in the collection
        if collection_id:
            logger.info(
                "Storing %s report in collection %s", platform.value, collection_id
            )
            collection_service = CollectionService()

            # Convert report to JSON bytes
            report_json = report.model_dump_json(indent=2)
            report_bytes = report_json.encode("utf-8")

            # Upload to collection with timestamp in name
            timestamp = datetime.now(timezone.utc).strftime("%Y%m%d_%H%M%S")
            safe_name = person_name.replace(" ", "_")
            doc_name = f"{safe_name}_{platform.value}_report_{timestamp}.json"
            document_id = collection_service.upload_document(
                collection_id=collection_id,
                name=doc_name,
                data=report_bytes,
                old_document_id=old_document_id,
            )

            logger.info(
                "Stored %s report for %s (doc_id: %s)",
                platform.value,
                person_name,
                document_id,
            )

        return ResearchResponse(success=True, report=report, document_id=document_id)

    except Exception as e:
        logger.exception("Failed to research candidate on %s: %s", platform.value, e)
        return ResearchResponse(success=False, error=str(e))
