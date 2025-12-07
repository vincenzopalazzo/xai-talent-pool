"""Job matching service using Grok AI with cross-collection search."""

import json
import logging
from datetime import datetime, timezone

from xai_sdk import Client
from xai_sdk.chat import system, user
from xai_sdk.tools import collections_search

from grok_service.config import get_settings
from grok_service.models import (
    CandidateMatch,
    JobMatchingResult,
    TalentForMatching,
)

logger = logging.getLogger(__name__)


MATCHING_SYSTEM_PROMPT = """\
You are a technical recruiter matching candidates to a job.
Search the candidate collections to find relevant documents, then score each candidate.
Be objective and evidence-based. Return valid JSON only."""


class JobMatchingService:
    """Service for matching candidates to jobs using cross-collection search."""

    def __init__(self) -> None:
        """Initialize the job matching service."""
        self.settings = get_settings()
        self.client = Client()

    def match_candidates_to_job(
        self,
        job_id: str,
        job_title: str,
        job_description: str,
        company_name: str,
        skills_required: str,
        experience_level: str,
        candidates: list[TalentForMatching],
        top_n: int = 10,
    ) -> JobMatchingResult:
        """
        Match candidates to a job by searching their collections.

        Args:
            job_id: The job's unique identifier
            job_title: Job title
            job_description: Job description
            company_name: Company name
            skills_required: Required skills (comma-separated)
            experience_level: Required experience level
            candidates: List of candidates to evaluate
            top_n: Number of top candidates to return

        Returns:
            JobMatchingResult with ranked candidates
        """
        logger.info("=" * 70)
        logger.info("JOB MATCHING: Starting")
        logger.info("=" * 70)
        logger.info("Job ID: %s", job_id)
        logger.info("Job: %s at %s", job_title, company_name)
        logger.info("Candidates to evaluate: %d", len(candidates))
        logger.info("=" * 70)

        if not candidates:
            logger.warning("JOB MATCHING: No candidates to evaluate")
            return JobMatchingResult(
                job_id=job_id,
                matches=[],
                total_evaluated=0,
                timestamp=datetime.now(timezone.utc).isoformat(),
            )

        # Get collection IDs from all candidates
        collection_ids = [c.collection_id for c in candidates if c.collection_id]

        if not collection_ids:
            logger.warning("JOB MATCHING: No candidates have collections")
            return JobMatchingResult(
                job_id=job_id,
                matches=[],
                total_evaluated=0,
                timestamp=datetime.now(timezone.utc).isoformat(),
            )

        logger.info("JOB MATCHING: Searching %d collections", len(collection_ids))

        # Build candidate context
        candidate_context = "\n".join(
            [
                f"- {c.name} (ID: {c.id}): {c.title}, Skills: {c.skills}"
                for c in candidates
            ]
        )

        # Create chat with cross-collection search
        chat = self.client.chat.create(
            model="grok-4-1-fast",
            tools=[
                collections_search(
                    collection_ids=collection_ids,
                    limit=20,  # Get more results for cross-collection
                    instructions=(
                        f"Search for candidates matching this job: {job_title}. "
                        f"Required skills: {skills_required}. "
                        "Find resumes and profiles that best match these requirements."
                    ),
                    retrieval_mode="hybrid",
                ),
            ],
            messages=[system(MATCHING_SYSTEM_PROMPT)],
            max_turns=2,
        )

        matching_prompt = f"""\
Match candidates to this job and rank the top {top_n}:

JOB:
- Title: {job_title}
- Company: {company_name}
- Description: {job_description[:500]}
- Required Skills: {skills_required or "Not specified"}
- Experience: {experience_level or "Not specified"}

CANDIDATES:
{candidate_context}

Search the collections to find each candidate's resume/profile, then score and rank them.

Return JSON array (top {top_n} only, sorted by score descending):
[{{"talent_id": "id", "talent_name": "name", "score": 0-100, "match_reasons": ["reason1", "reason2"], "concerns": ["concern1"], "summary": "1-2 sentences"}}]"""

        chat.append(user(matching_prompt))

        try:
            logger.info("JOB MATCHING: Sending request to Grok API...")
            response = chat.sample()
            logger.info("JOB MATCHING: Response received")

            # Parse response
            content = response.content.strip()
            if content.startswith("```json"):
                content = content[7:]
            if content.startswith("```"):
                content = content[3:]
            if content.endswith("```"):
                content = content[:-3]
            content = content.strip()

            logger.debug("JOB MATCHING: Raw response: %s", content)

            data = json.loads(content)

            # Build matches
            matches = []
            for rank, item in enumerate(data[:top_n], 1):
                # Find candidate info
                candidate = next(
                    (c for c in candidates if c.id == item.get("talent_id")), None
                )

                match = CandidateMatch(
                    talent_id=item.get("talent_id", ""),
                    talent_name=item.get(
                        "talent_name", candidate.name if candidate else ""
                    ),
                    talent_title=candidate.title if candidate else "",
                    score=float(item.get("score", 0)),
                    rank=rank,
                    match_reasons=item.get("match_reasons", []),
                    concerns=item.get("concerns", []),
                    summary=item.get("summary", ""),
                )
                matches.append(match)

            logger.info("JOB MATCHING: Found %d matches", len(matches))
            for m in matches[:3]:
                logger.info("  #%d: %s (%.1f)", m.rank, m.talent_name, m.score)

            return JobMatchingResult(
                job_id=job_id,
                matches=matches,
                total_evaluated=len(candidates),
                timestamp=datetime.now(timezone.utc).isoformat(),
            )

        except json.JSONDecodeError as e:
            logger.error("JOB MATCHING: JSON parse error: %s", e)
            logger.error(
                "JOB MATCHING: Raw content: %s",
                content if "content" in dir() else "N/A",
            )
            return JobMatchingResult(
                job_id=job_id,
                matches=[],
                total_evaluated=len(candidates),
                timestamp=datetime.now(timezone.utc).isoformat(),
            )
        except Exception as e:
            logger.error("JOB MATCHING: Error: %s", e)
            raise
