"""Grok AI service for resume analysis."""

import json
import logging

from pydantic import BaseModel
from xai_sdk import Client
from xai_sdk.chat import system, user

from grok_service.config import get_settings

logger = logging.getLogger(__name__)


class ExperienceSummary(BaseModel):
    """Summary of a single work experience."""

    company: str
    role: str
    duration: str | None = None
    summary: str


class ResumeAnalysis(BaseModel):
    """Result of resume analysis by Grok."""

    experiences: list[ExperienceSummary]
    urls: dict[str, str | None]  # linkedin, x, github, gitlab


SYSTEM_PROMPT = """\
You are an expert resume analyzer. \
Your task is to analyze resumes and extract structured information.

When given a resume, you must:
1. Extract and summarize each work experience mentioned
2. Find any URLs or profile links (LinkedIn, X/Twitter, GitHub, GitLab)

You MUST respond with valid JSON in this exact format:
{
    "experiences": [
        {
            "company": "Company Name",
            "role": "Job Title",
            "duration": "Start - End (e.g., Jan 2020 - Dec 2022)",
            "summary": "Brief 1-2 sentence summary of responsibilities and achievements"
        }
    ],
    "urls": {
        "linkedin": "https://linkedin.com/in/... or null if not found",
        "x": "https://x.com/... or https://twitter.com/... or null if not found",
        "github": "https://github.com/... or null if not found",
        "gitlab": "https://gitlab.com/... or null if not found"
    }
}

Important:
- List experiences in chronological order (most recent first)
- Keep summaries concise but informative
- If a URL is not found, use null
- Only return the JSON, no additional text"""


class GrokService:
    """Service for interacting with Grok AI."""

    def __init__(self) -> None:
        """Initialize the Grok service."""
        self.settings = get_settings()
        self.client = Client()

    def analyze_resume(self, resume_text: str) -> ResumeAnalysis:
        """
        Analyze a resume and extract experiences and URLs.

        Args:
            resume_text: Plain text content of the resume

        Returns:
            ResumeAnalysis with experiences and URLs
        """
        chat = self.client.chat.create(
            model="grok-3",
            messages=[system(SYSTEM_PROMPT)],
        )

        prompt = f"""\
Please analyze this resume and extract the work experiences and profile URLs.

Resume content:
---
{resume_text}
---

Respond with the JSON structure only."""

        chat.append(user(prompt))

        logger.info("=" * 70)
        logger.info("GROK AI: Sending request to Grok...")
        logger.info("=" * 70)

        response = chat.sample()

        logger.info("=" * 70)
        logger.info("GROK AI: RAW RESPONSE FROM GROK")
        logger.info("=" * 70)
        logger.info(response.content)
        logger.info("=" * 70)

        # Parse the JSON response
        try:
            # Clean up response - remove markdown code blocks if present
            content = response.content.strip()
            if content.startswith("```json"):
                content = content[7:]
            if content.startswith("```"):
                content = content[3:]
            if content.endswith("```"):
                content = content[:-3]
            content = content.strip()

            logger.info("GROK AI: CLEANED JSON")
            logger.info("=" * 70)
            logger.info(content)
            logger.info("=" * 70)

            data = json.loads(content)

            logger.info("GROK AI: PARSED DATA")
            logger.info("=" * 70)
            logger.info(json.dumps(data, indent=2))
            logger.info("=" * 70)

            return ResumeAnalysis(
                experiences=[
                    ExperienceSummary(**exp) for exp in data.get("experiences", [])
                ],
                urls=data.get("urls", {}),
            )
        except json.JSONDecodeError as e:
            logger.error("=" * 70)
            logger.error("GROK AI: JSON PARSE ERROR: %s", e)
            logger.error("Raw response: %s", response.content)
            logger.error("=" * 70)
            # Return empty result on parse failure
            return ResumeAnalysis(
                experiences=[],
                urls={"linkedin": None, "x": None, "github": None, "gitlab": None},
            )
