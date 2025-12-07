"""Candidate scoring service using Grok AI with collections search."""

import json
import logging
from datetime import datetime, timezone

from xai_sdk import Client
from xai_sdk.chat import system, user
from xai_sdk.tools import collections_search

from grok_service.config import get_settings
from grok_service.models import (
    CandidateScoringResult,
    JobInfo,
    ScoringBreakdown,
)

logger = logging.getLogger(__name__)


SCORING_SYSTEM_PROMPT = """\
You are an expert technical recruiter AI assistant for xAI, a company building AI systems \
to understand the universe. You have access to a candidate's documents (resume, social media \
analysis, etc.) through a collection search tool.

Your task is to evaluate how well a candidate fits a specific job opening. You must:

1. Search the candidate's collection to find relevant information about their background
2. Analyze their skills, experience, and qualifications against the job requirements
3. Provide an objective, evidence-based assessment

Be direct, factual, and critical. Do not inflate scores. xAI values:
- Engineering excellence and deep technical skills
- Initiative and ability to work independently
- Strong communication skills
- Work ethic and prioritization abilities
- Hands-on contributors who deliver results

When scoring, use the full 0-100 range:
- 90-100: Exceptional fit, rare candidate
- 75-89: Strong fit, should definitely interview
- 60-74: Good fit, worth considering
- 45-59: Moderate fit, some gaps
- 30-44: Weak fit, significant concerns
- 0-29: Poor fit, does not meet requirements

You MUST respond with valid JSON only. No markdown, no explanations outside JSON."""


class CandidateScoringService:
    """Service for scoring candidates using collection search with Grok AI."""

    def __init__(self) -> None:
        """Initialize the scoring service."""
        self.settings = get_settings()
        self.client = Client()

    def score_candidate(
        self,
        talent_id: str,
        collection_id: str,
        job: JobInfo,
        candidate_name: str,
        candidate_title: str = "",
        candidate_skills: str = "",
    ) -> CandidateScoringResult:
        """
        Score a candidate's fit for a job using their collection documents.

        This method uses the collections_search tool to query the candidate's
        uploaded documents (resume, social media analysis, etc.) and generate
        a comprehensive fit score.

        Args:
            talent_id: The talent's unique identifier
            collection_id: The xAI collection ID containing candidate documents
            job: Job information to score against
            candidate_name: Candidate's name for context
            candidate_title: Candidate's current job title
            candidate_skills: Comma-separated list of candidate skills

        Returns:
            CandidateScoringResult with scores and recommendations
        """
        logger.info("=" * 70)
        logger.info("SCORING: Starting candidate scoring")
        logger.info("=" * 70)
        logger.info("Talent ID: %s", talent_id)
        logger.info("Collection ID: %s", collection_id)
        logger.info("Job: %s at %s", job.title, job.company_name)
        logger.info("Candidate: %s (%s)", candidate_name, candidate_title)
        logger.info("=" * 70)

        # Create chat with collections_search tool
        chat = self.client.chat.create(
            model="grok-4-1-fast",
            tools=[
                collections_search(
                    collection_ids=[collection_id],
                    limit=10,
                    instructions=(
                        "Search for information about the candidate's skills, experience, "
                        "projects, education, and any analysis results. Look for evidence "
                        "of technical abilities, leadership, communication skills, and "
                        "relevant domain expertise."
                    ),
                    retrieval_mode="hybrid",
                ),
            ],
            messages=[system(SCORING_SYSTEM_PROMPT)],
        )

        # Build the scoring prompt
        scoring_prompt = f"""\
Evaluate this candidate for the following job:

**Job Details:**
- Title: {job.title}
- Company: {job.company_name}
- Description: {job.description}
- Required Skills: {job.skills_required or "Not specified"}
- Experience Level: {job.experience_level or "Not specified"}
- Location: {job.location or "Not specified"} ({job.location_type})

**Candidate Overview:**
- Name: {candidate_name}
- Current Title: {candidate_title or "Unknown"}
- Listed Skills: {candidate_skills or "Unknown"}

Please search the candidate's collection to find their resume, social media analysis, \
and any other relevant documents. Then provide a comprehensive evaluation.

Respond with this exact JSON structure:
{{
    "overall_score": <0-100 integer>,
    "breakdown": {{
        "skills_match": <0-100 integer>,
        "experience_fit": <0-100 integer>,
        "culture_fit": <0-100 integer>,
        "overall_impression": <0-100 integer>
    }},
    "strengths": ["strength 1", "strength 2", ...],
    "concerns": ["concern 1", "concern 2", ...],
    "recommendation": "<strong_yes|yes|maybe|no>",
    "summary": "<2-3 sentence summary of candidate fit>"
}}"""

        chat.append(user(scoring_prompt))

        try:
            # Run the agentic loop - Grok will search the collection and analyze
            response = chat.run()

            logger.info(
                "SCORING: Response received (%d tokens)",
                response.usage.completion_tokens if response.usage else 0,
            )

            # Parse the JSON response
            content = response.content.strip()

            # Clean up markdown if present
            if content.startswith("```json"):
                content = content[7:]
            if content.startswith("```"):
                content = content[3:]
            if content.endswith("```"):
                content = content[:-3]
            content = content.strip()

            logger.debug("SCORING: Raw response: %s", content)

            data = json.loads(content)

            # Build result
            breakdown = ScoringBreakdown(
                skills_match=float(data.get("breakdown", {}).get("skills_match", 0)),
                experience_fit=float(
                    data.get("breakdown", {}).get("experience_fit", 0)
                ),
                culture_fit=float(data.get("breakdown", {}).get("culture_fit", 0)),
                overall_impression=float(
                    data.get("breakdown", {}).get("overall_impression", 0)
                ),
            )

            result = CandidateScoringResult(
                talent_id=talent_id,
                job_id=job.id,
                overall_score=float(data.get("overall_score", 0)),
                breakdown=breakdown,
                strengths=data.get("strengths", []),
                concerns=data.get("concerns", []),
                recommendation=data.get("recommendation", "maybe"),
                summary=data.get("summary", ""),
                timestamp=datetime.now(timezone.utc).isoformat(),
            )

            logger.info("=" * 70)
            logger.info("SCORING: Completed successfully")
            logger.info("Overall Score: %.1f", result.overall_score)
            logger.info("Recommendation: %s", result.recommendation)
            logger.info("Summary: %s", result.summary)
            logger.info("=" * 70)

            return result

        except json.JSONDecodeError as e:
            logger.error("SCORING: JSON parse error: %s", e)
            logger.error(
                "SCORING: Raw content: %s", content if "content" in dir() else "N/A"
            )
            # Return a default result on parse error
            return CandidateScoringResult(
                talent_id=talent_id,
                job_id=job.id,
                overall_score=0.0,
                breakdown=ScoringBreakdown(),
                strengths=[],
                concerns=["Failed to parse scoring response"],
                recommendation="no",
                summary="Scoring failed due to response parsing error.",
                timestamp=datetime.now(timezone.utc).isoformat(),
            )
        except Exception as e:
            logger.error("SCORING: Error during scoring: %s", e)
            raise
