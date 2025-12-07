"""Screening API router."""

import json
import logging

from fastapi import APIRouter, File, Form, HTTPException, UploadFile

from grok_service.models import (
    ExperienceSummary,
    ProfileUrls,
    ScreeningResponse,
    ScreeningResult,
    TalentInfo,
)
from grok_service.services import GrokService, extract_text_from_pdf

logger = logging.getLogger(__name__)

router = APIRouter(prefix="/api/v1/screening", tags=["screening"])


@router.post("/initial", response_model=ScreeningResponse)
async def initial_screening(
    talent_info: str = Form(..., description="JSON string of TalentInfo"),
    resume: UploadFile = File(..., description="Resume PDF file"),
) -> ScreeningResponse:
    """
    Perform initial screening of a talent based on their info and resume.

    Analyzes the resume using Grok AI to extract:
    - Summary of each work experience
    - Profile URLs (LinkedIn, X, GitHub, GitLab)

    Args:
        talent_info: JSON string containing talent information
        resume: PDF file of the talent's resume

    Returns:
        ScreeningResponse with experiences and profile URLs
    """
    # Validate content type
    if resume.content_type != "application/pdf":
        raise HTTPException(
            status_code=400,
            detail=f"Invalid file type: {resume.content_type}. "
            "Only PDF files are accepted.",
        )

    # Parse talent info
    try:
        talent_data = json.loads(talent_info)
        talent = TalentInfo(**talent_data)
    except json.JSONDecodeError as e:
        raise HTTPException(
            status_code=400,
            detail=f"Invalid JSON in talent_info: {e}",
        )
    except Exception as e:
        raise HTTPException(
            status_code=400,
            detail=f"Invalid talent_info structure: {e}",
        )

    # Read and extract text from PDF
    try:
        resume_content = await resume.read()
        resume_text = extract_text_from_pdf(resume_content)
    except Exception as e:
        logger.error(f"Failed to extract text from PDF: {e}")
        raise HTTPException(
            status_code=400,
            detail=f"Failed to parse PDF: {e}",
        )

    if not resume_text.strip():
        raise HTTPException(
            status_code=400,
            detail="Could not extract any text from the PDF. "
            "The file may be empty or image-based.",
        )

    # Log extracted PDF text
    logger.info("=" * 70)
    logger.info("SCREENING: Received request")
    logger.info("=" * 70)
    logger.info("Talent ID: %s", talent.id)
    logger.info("Talent Name: %s", talent.name)
    logger.info("PDF Size: %d bytes", len(resume_content))
    logger.info("Extracted text length: %d characters", len(resume_text))
    logger.info("=" * 70)
    logger.info("EXTRACTED PDF TEXT:")
    logger.info("=" * 70)
    logger.info(resume_text[:2000] if len(resume_text) > 2000 else resume_text)
    if len(resume_text) > 2000:
        logger.info("... (truncated, total %d chars)", len(resume_text))
    logger.info("=" * 70)

    # Analyze resume with Grok
    try:
        grok_service = GrokService()
        analysis = grok_service.analyze_resume(resume_text)
    except Exception as e:
        logger.error(f"Grok analysis failed: {e}")
        return ScreeningResponse(
            success=False,
            result=None,
            error=f"AI analysis failed: {e}",
        )

    # Build response
    experiences = [
        ExperienceSummary(
            company=exp.company,
            role=exp.role,
            duration=exp.duration,
            summary=exp.summary,
        )
        for exp in analysis.experiences
    ]

    urls = ProfileUrls(
        linkedin=analysis.urls.get("linkedin"),
        x=analysis.urls.get("x"),
        github=analysis.urls.get("github"),
        gitlab=analysis.urls.get("gitlab"),
    )

    result = ScreeningResult(
        talent_id=talent.id,
        experiences=experiences,
        urls=urls,
    )

    # Log result
    logger.info("=" * 60)
    logger.info("SCREENING RESULT")
    logger.info("=" * 60)
    logger.info("Talent ID: %s", result.talent_id)
    logger.info("Experiences (%d):", len(result.experiences))
    for i, exp in enumerate(result.experiences, 1):
        logger.info("  %d. %s at %s", i, exp.role, exp.company)
        if exp.duration:
            logger.info("     Duration: %s", exp.duration)
        logger.info("     Summary: %s", exp.summary)
    logger.info("Profile URLs:")
    logger.info("  LinkedIn: %s", result.urls.linkedin or "Not found")
    logger.info("  X:        %s", result.urls.x or "Not found")
    logger.info("  GitHub:   %s", result.urls.github or "Not found")
    logger.info("  GitLab:   %s", result.urls.gitlab or "Not found")
    logger.info("=" * 60)

    return ScreeningResponse(
        success=True,
        result=result,
        error=None,
    )
