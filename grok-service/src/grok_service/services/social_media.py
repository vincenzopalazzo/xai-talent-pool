"""Social media analysis service using Grok AI."""

import json
import logging
import re
from datetime import datetime, timezone

from pydantic import BaseModel
from xai_sdk import Client
from xai_sdk.chat import system, user

from grok_service.config import get_settings
from grok_service.models import (
    PlatformProfile,
    SocialMediaAnalysisResult,
    SocialMediaInput,
)
from grok_service.services.collections import CollectionService

logger = logging.getLogger(__name__)


class PlatformAnalysis(BaseModel):
    """Parsed result from a single platform analysis."""

    handle: str | None = None
    url: str | None = None
    verified: bool = False
    bio: str | None = None
    highlights: list[str] = []
    skills: list[str] = []
    experience_signals: list[str] = []
    red_flags: list[str] = []
    recruiter_notes: list[str] = []


PLATFORM_CONFIGS = {
    "GitHub": {
        "focus": "technical projects, repositories, code contributions, programming languages, and open-source activity",
        "key_signals": "commit frequency, stars, forks, pull requests, issues, code quality",
    },
    "LinkedIn": {
        "focus": "professional experience, job history, education, skills, endorsements, and recommendations",
        "key_signals": "career progression, title changes, company quality, endorsement count",
    },
    "X": {
        "focus": "professional tweets, thought leadership, industry engagement, tech discussions, and online presence",
        "key_signals": "follower count, engagement quality, tech content ratio, community standing",
    },
    "GitLab": {
        "focus": "technical projects, merge requests, contributions, and DevOps-related activity",
        "key_signals": "project activity, contribution patterns, code review involvement",
    },
    "StackOverflow": {
        "focus": "technical questions answered, reputation score, areas of expertise, and community contributions",
        "key_signals": "reputation score, badges, top tags, answer quality",
    },
}


SYSTEM_PROMPT = """\
You are an evidence-first recruiter research assistant.
Only provide verifiable facts from publicly available information.
Do NOT guess, infer private details, or fill gaps with assumptions.
If something cannot be confirmed, write 'Unknown' or 'Not publicly indicated'.
Be concise and professional.
When referring to a claim that would normally require a source, include a URL if you have one.
Do not add motivational language, filler, or generic advice unless explicitly asked.

You MUST respond with valid JSON only. No markdown, no explanations outside JSON."""


def extract_handle_from_url(url: str | None, platform: str) -> str | None:
    """Extract the handle/username from a social media URL."""
    if not url:
        return None

    patterns = {
        "X": [
            r"(?:twitter\.com|x\.com)/(@?\w+)",
            r"(?:twitter\.com|x\.com)/intent/user\?screen_name=(\w+)",
        ],
        "GitHub": [r"github\.com/([^/\?]+)"],
        "GitLab": [r"gitlab\.com/([^/\?]+)"],
        "LinkedIn": [r"linkedin\.com/in/([^/\?]+)"],
        "StackOverflow": [r"stackoverflow\.com/users/\d+/([^/\?]+)"],
    }

    for pattern in patterns.get(platform, []):
        match = re.search(pattern, url, re.IGNORECASE)
        if match:
            handle = match.group(1)
            return handle.lstrip("@")
    return None


class SocialMediaService:
    """Service for analyzing social media profiles using Grok AI."""

    def __init__(self) -> None:
        """Initialize the social media service."""
        self.settings = get_settings()
        self.client = Client()

    def _analyze_platform(
        self,
        person_name: str,
        email: str | None,
        platform: str,
        url: str | None,
    ) -> PlatformAnalysis:
        """Analyze a person's presence on a specific platform."""
        config = PLATFORM_CONFIGS.get(platform, {})
        focus = config.get("focus", "general professional presence")
        key_signals = config.get("key_signals", "overall activity and engagement")

        chat = self.client.chat.create(
            model="grok-3-mini",
            reasoning_effort="high",
            messages=[system(SYSTEM_PROMPT)],
        )

        url_hint = f"\nKnown profile URL: {url}" if url else ""
        email_hint = f"\nEmail hint: {email}" if email else ""

        prompt = f"""\
Research the {platform} profile for: {person_name}
{url_hint}{email_hint}

Focus scope: {focus}
Key signals to look for: {key_signals}

Rules:
- Output MUST be factual and concise
- Do NOT speculate
- If you cannot verify something from public info, write null or empty array
- Prefer short bullet points
- Do not repeat the instructions

Respond with this exact JSON structure:
{{
    "handle": "username on {platform} or null if not found",
    "url": "full profile URL or null if not found",
    "verified": true/false,
    "bio": "profile bio/summary or null",
    "highlights": ["key achievements or notable items"],
    "skills": ["technical skills or expertise identified"],
    "experience_signals": ["seniority or experience indicators"],
    "red_flags": ["professional concerns if any, empty if none"],
    "recruiter_notes": ["3-5 key takeaways for a recruiter"]
}}"""

        chat.append(user(prompt))

        logger.info("=" * 70)
        logger.info("SOCIAL MEDIA: Analyzing %s for %s", platform, person_name)
        logger.info("=" * 70)

        try:
            response = chat.sample()

            logger.info(
                "SOCIAL MEDIA: %s response (%d tokens, %d reasoning)",
                platform,
                response.usage.completion_tokens,
                response.usage.reasoning_tokens,
            )

            # Parse the JSON response
            content = response.content.strip()
            if content.startswith("```json"):
                content = content[7:]
            if content.startswith("```"):
                content = content[3:]
            if content.endswith("```"):
                content = content[:-3]
            content = content.strip()

            logger.debug("SOCIAL MEDIA: Raw response: %s", content)

            data = json.loads(content)
            return PlatformAnalysis(
                handle=data.get("handle"),
                url=data.get("url"),
                verified=data.get("verified", False),
                bio=data.get("bio"),
                highlights=data.get("highlights", []),
                skills=data.get("skills", []),
                experience_signals=data.get("experience_signals", []),
                red_flags=data.get("red_flags", []),
                recruiter_notes=data.get("recruiter_notes", []),
            )
        except json.JSONDecodeError as e:
            logger.error("SOCIAL MEDIA: JSON parse error for %s: %s", platform, e)
            return PlatformAnalysis()
        except Exception as e:
            logger.error("SOCIAL MEDIA: Error analyzing %s: %s", platform, e)
            return PlatformAnalysis()

    def analyze_profiles(
        self,
        talent_id: str,
        name: str,
        email: str | None,
        social_urls: SocialMediaInput,
        platforms_to_search: list[str] | None = None,
        collection_id: str | None = None,
    ) -> SocialMediaAnalysisResult:
        """
        Analyze all provided social media profiles for a candidate.

        Args:
            talent_id: Unique identifier for the talent
            name: Full name of the candidate
            email: Email address if available (used as search hint)
            social_urls: Social media URLs to analyze
            platforms_to_search: List of platforms to search if URL not provided
            collection_id: Optional collection ID to store the analysis result

        Returns:
            SocialMediaAnalysisResult with profiles and extracted X handle
        """
        profiles: list[PlatformProfile] = []
        x_handle: str | None = None
        all_skills: set[str] = set()

        if platforms_to_search is None:
            platforms_to_search = ["X", "GitHub", "LinkedIn"]

        # Map of social_urls fields to platform names
        url_mapping = [
            ("x", "X", social_urls.x),
            ("github", "GitHub", social_urls.github),
            ("linkedin", "LinkedIn", social_urls.linkedin),
            ("gitlab", "GitLab", social_urls.gitlab),
            ("stackoverflow", "StackOverflow", social_urls.stackoverflow),
        ]

        for field_name, platform, url in url_mapping:
            # Only analyze if we have a URL or it's a platform we want to search
            if url or platform in platforms_to_search:
                logger.info("SOCIAL MEDIA: Analyzing %s profile...", platform)

                analysis = self._analyze_platform(name, email, platform, url)

                # Extract X handle specifically
                if platform == "X":
                    # First try from URL
                    x_handle = extract_handle_from_url(url, "X")
                    # If not from URL, try from analysis
                    if not x_handle and analysis.handle:
                        x_handle = analysis.handle.lstrip("@")

                # Collect skills
                all_skills.update(analysis.skills)

                # Generate TLDR for this platform
                tldr = self._generate_platform_tldr(platform, analysis)

                # Build profile object
                profile = PlatformProfile(
                    platform=platform,
                    handle=analysis.handle,
                    url=analysis.url or url,
                    verified=analysis.verified,
                    bio=analysis.bio,
                    tldr=tldr,
                    highlights=analysis.highlights,
                    skills=analysis.skills,
                    experience_signals=analysis.experience_signals,
                    red_flags=analysis.red_flags,
                    recruiter_notes=analysis.recruiter_notes,
                )
                profiles.append(profile)

        # Generate overall summary and TLDR
        summary = self._generate_summary(name, profiles)
        tldr = self._generate_overall_tldr(name, profiles)

        # Generate timestamp
        timestamp = datetime.now(timezone.utc).isoformat()

        result = SocialMediaAnalysisResult(
            talent_id=talent_id,
            timestamp=timestamp,
            x_handle=x_handle,
            tldr=tldr,
            profiles=profiles,
            combined_skills=list(all_skills),
            summary=summary,
        )

        # Upload to collection if collection_id is provided
        if collection_id:
            try:
                collection_service = CollectionService()
                document_name = f"social_media_analysis_{timestamp}.json"
                json_data = result.model_dump_json().encode("utf-8")

                logger.info(
                    "SOCIAL MEDIA: Uploading analysis to collection %s", collection_id
                )
                collection_service.upload_document(
                    collection_id=collection_id,
                    name=document_name,
                    data=json_data,
                )
            except Exception as e:
                logger.error(
                    "SOCIAL MEDIA: Failed to upload analysis to collection: %s", e
                )
                # We don't fail the whole request if upload fails, just log it

        return result

    def _generate_summary(
        self, name: str, profiles: list[PlatformProfile]
    ) -> str | None:
        """Generate an overall summary from all profile analyses."""
        if not profiles:
            return None

        # Collect all recruiter notes
        all_notes = []
        for profile in profiles:
            for note in profile.recruiter_notes:
                all_notes.append(f"[{profile.platform}] {note}")

        if not all_notes:
            return None

        # Create a concise summary
        chat = self.client.chat.create(
            model="grok-4-1-fast-non-reasoning",
            messages=[
                system(
                    "You are a concise recruiter assistant. "
                    "Synthesize the following notes into a 2-3 sentence summary. "
                    "Focus on the most important hiring signals."
                )
            ],
        )

        notes_text = "\n".join(all_notes)
        chat.append(user(f"Candidate: {name}\n\nNotes:\n{notes_text}"))

        try:
            response = chat.sample()
            return response.content.strip()
        except Exception as e:
            logger.error("SOCIAL MEDIA: Error generating summary: %s", e)
            return None

    def _generate_platform_tldr(
        self, platform: str, analysis: "PlatformAnalysis"
    ) -> str | None:
        """Generate a TLDR summary for a specific platform analysis."""
        if not analysis.recruiter_notes and not analysis.highlights:
            return None

        chat = self.client.chat.create(
            model="grok-4-1-fast-non-reasoning",
            messages=[
                system(
                    "You are a concise recruiter assistant. "
                    "Create a TLDR (2-3 sentences) summary of the candidate's profile. "
                    "Focus on the most important signals for hiring. "
                    "Be direct and factual."
                )
            ],
        )

        # Build context for TLDR
        context_parts = []
        if analysis.bio:
            context_parts.append(f"Bio: {analysis.bio}")
        if analysis.highlights:
            context_parts.append(
                f"Key highlights: {'; '.join(analysis.highlights[:3])}"
            )
        if analysis.experience_signals:
            context_parts.append(
                f"Experience signals: {'; '.join(analysis.experience_signals[:2])}"
            )
        if analysis.recruiter_notes:
            context_parts.append(
                f"Recruiter notes: {'; '.join(analysis.recruiter_notes[:2])}"
            )

        if not context_parts:
            return None

        context = "\n".join(context_parts)
        chat.append(user(f"Platform: {platform}\n\n{context}\n\nTLDR:"))

        try:
            response = chat.sample()
            return response.content.strip()
        except Exception as e:
            logger.error("SOCIAL MEDIA: Error generating platform TLDR: %s", e)
            return None

    def _generate_overall_tldr(
        self, name: str, profiles: list[PlatformProfile]
    ) -> str | None:
        """Generate an overall TLDR summary from all platform analyses."""
        if not profiles:
            return None

        # Collect key insights from all platforms
        all_insights = []
        for profile in profiles:
            if profile.tldr:
                all_insights.append(f"[{profile.platform}] {profile.tldr}")
            elif profile.recruiter_notes:
                all_insights.append(
                    f"[{profile.platform}] {'; '.join(profile.recruiter_notes[:2])}"
                )

        if not all_insights:
            return None

        chat = self.client.chat.create(
            model="grok-4-1-fast-non-reasoning",
            messages=[
                system(
                    "You are a concise recruiter assistant. "
                    "Create a TLDR summary (3-4 sentences) of the candidate based on "
                    "their social media presence across all platforms. "
                    "Focus on the most important hiring signals: skills, experience level, "
                    "and red flags. Be direct and factual."
                )
            ],
        )

        insights_text = "\n".join(all_insights)
        chat.append(
            user(
                f"Candidate: {name}\n\nPlatform Insights:\n{insights_text}\n\nOverall TLDR:"
            )
        )

        try:
            response = chat.sample()
            return response.content.strip()
        except Exception as e:
            logger.error("SOCIAL MEDIA: Error generating overall TLDR: %s", e)
            return None
