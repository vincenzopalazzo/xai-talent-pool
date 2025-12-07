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
You are a technical recruiter evaluating candidates. Use the collections_search tool to find candidate documents, then score them.

Scoring scale (0-100):
- 90-100: Exceptional fit
- 75-89: Strong fit
- 60-74: Good fit
- 45-59: Moderate fit
- 30-44: Weak fit
- 0-29: Poor fit

Respond with JSON only, no markdown."""


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

        # Validate collection_id
        if not collection_id:
            logger.error("SCORING: No collection_id provided!")
            raise ValueError("collection_id is required for scoring")

        logger.info("SCORING: Querying ONLY collection: %s", collection_id)

        # Create chat with collections_search tool for THIS candidate's collection only
        # max_turns limits the agentic loop to prevent infinite tool calls
        chat = self.client.chat.create(
            model="grok-4-1-fast",
            tools=[
                collections_search(
                    collection_ids=[collection_id],  # Single collection for this candidate
                    limit=5,  # Reduced limit for faster response
                    instructions="Search this candidate's resume and profile documents.",
                    retrieval_mode="hybrid",
                ),
            ],
            messages=[system(SCORING_SYSTEM_PROMPT)],
            max_turns=2,  # Reduced turns: 1 for search, 1 for response
        )

        # Build the scoring prompt
        scoring_prompt = f"""\
Search the collection for candidate "{candidate_name}" documents and evaluate for this job:

Job: {job.title} at {job.company_name}
Required: {job.skills_required or "Not specified"}
Level: {job.experience_level or "Not specified"}

Return JSON:
{{"overall_score": 0-100, "breakdown": {{"skills_match": 0-100, "experience_fit": 0-100, "culture_fit": 0-100, "overall_impression": 0-100}}, "strengths": ["..."], "concerns": ["..."], "recommendation": "strong_yes|yes|maybe|no", "summary": "2-3 sentences"}}"""

        chat.append(user(scoring_prompt))

        try:
            logger.info("SCORING: Sending request to Grok API...")
            # Sample from the chat - Grok will search the collection and analyze
            response = chat.sample()
            logger.info("SCORING: Response received from Grok API")

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
