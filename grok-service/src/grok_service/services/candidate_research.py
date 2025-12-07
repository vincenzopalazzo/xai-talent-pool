"""Candidate research service using Grok AI."""

import logging
from datetime import datetime, timezone
from enum import Enum

from pydantic import BaseModel, Field
from xai_sdk import Client
from xai_sdk.chat import system, user
from xai_sdk.tools import web_search, x_search

from grok_service.config import get_settings

logger = logging.getLogger(__name__)


class Platform(str, Enum):
    """Supported platforms for candidate research."""

    GITHUB = "github"
    LINKEDIN = "linkedin"
    TWITTER = "twitter"
    STACKOVERFLOW = "stackoverflow"


# Platform-specific focus areas
PLATFORM_FOCUS = {
    Platform.GITHUB: (
        "technical projects, repositories, code contributions, "
        "programming languages, and open-source activity"
    ),
    Platform.LINKEDIN: (
        "professional experience, job history, education, "
        "skills, endorsements, and recommendations"
    ),
    Platform.TWITTER: (
        "professional tweets, thought leadership, "
        "industry engagement, and online presence"
    ),
    Platform.STACKOVERFLOW: (
        "technical questions answered, reputation score, "
        "areas of expertise, and community contributions"
    ),
}

SYSTEM_PROMPT = (
    "You are an evidence-first recruiter research assistant with search tools. "
    "Use web_search for web platforms (GitHub, LinkedIn, etc.) "
    "and x_search for X/Twitter. "
    "Always search for the specific profile URL if provided. "
    "Only provide verifiable facts from publicly available information. "
    "Do NOT guess, infer private details, or fill gaps with assumptions. "
    "If unconfirmed, write 'Unknown' or 'Not publicly indicated'. "
    "Be concise and professional. Use bullet points. "
    "Include source URLs from your search results. "
    "Do not add motivational language, filler, or generic advice."
)


class IdentityMatch(BaseModel):
    """Identity match information."""

    name: str | None = Field(None, description="Primary profile name")
    handle_or_url: str | None = Field(None, description="Handle or profile URL")
    location: str | None = Field(None, description="Location if listed")
    current_role: str | None = Field(None, description="Current role if listed")
    disambiguation_notes: str | None = Field(
        None, description="Notes if multiple matches"
    )


class PlatformReport(BaseModel):
    """Research report for a specific platform."""

    platform: str = Field(..., description="Platform name")
    person_name: str = Field(..., description="Name of the person researched")
    created_at: str = Field(
        default_factory=lambda: datetime.now(timezone.utc).isoformat(),
        description="Timestamp when the report was created",
    )
    tldr: str = Field("", description="One-sentence TLDR summary for quick display")
    identity_match: IdentityMatch = Field(
        default_factory=IdentityMatch, description="Identity match information"
    )
    evidence_based_highlights: list[str] = Field(
        default_factory=list, description="Key factual highlights"
    )
    professional_experience: list[str] = Field(
        default_factory=list, description="Professional experience"
    )
    skills_and_signals: list[str] = Field(
        default_factory=list, description="Skills and technical signals"
    )
    notable_work: list[str] = Field(
        default_factory=list, description="Notable work or contributions"
    )
    seniority_signals: list[str] = Field(
        default_factory=list, description="Signals of seniority or impact"
    )
    red_flags: list[str] = Field(default_factory=list, description="Red flags if any")
    recruiter_takeaways: list[str] = Field(
        default_factory=list, description="Key takeaways for recruiters"
    )
    open_questions: list[str] = Field(
        default_factory=list, description="Questions for human verification"
    )
    raw_content: str = Field("", description="Raw markdown content from Grok")
    completion_tokens: int = Field(0, description="Tokens used for completion")
    reasoning_tokens: int = Field(0, description="Tokens used for reasoning")


def _build_research_prompt(
    person_name: str,
    platform: Platform,
    email: str | None = None,
    profile_url: str | None = None,
) -> str:
    """Build the research prompt for a specific platform."""
    platform_name = platform.value.title()
    if platform == Platform.STACKOVERFLOW:
        platform_name = "Stack Overflow"
    elif platform == Platform.TWITTER:
        platform_name = "Twitter/X"

    focus = PLATFORM_FOCUS[platform]

    # Build email hint if provided
    email_hint = ""
    if email:
        email_hint = f"""
Email hint: {email}
(Use the email domain to help verify employer/organization affiliation)
"""

    # Build profile URL hint if provided - this is the key improvement
    profile_hint = ""
    if profile_url:
        profile_hint = f"""
IMPORTANT - Direct Profile URL: {profile_url}
Use web_search to browse this specific URL and extract information.
Do NOT search for other profiles with similar names - focus on this exact profile.
"""

    # Determine which tool to use based on platform
    tool_instruction = ""
    if platform == Platform.TWITTER:
        tool_instruction = (
            "Use x_search to search for this person's X/Twitter posts and profile. "
            "Use web_search for additional context if needed."
        )
    else:
        tool_instruction = (
            f"Use web_search to find this person's {platform_name} profile "
            "and professional presence."
        )

    return f"""
You are generating a {platform_name}-specific recruiting research note for:
{person_name}.
{email_hint}{profile_hint}
SEARCH INSTRUCTION: {tool_instruction}

Rules:
- Output MUST be factual and concise.
- Do NOT speculate.
- Do NOT include generic coaching or platform tips unless explicitly asked.
- If you cannot verify something from public info, write: Unknown.
- Prefer short bullet points over paragraphs.
- Do not repeat the instructions.
- Do not use marketing language.

Focus scope for {platform_name}:
{focus}

Write the report in Markdown using this exact structure:

## TLDR
(One sentence summary of this person's {platform_name} presence for recruiters)

## Identity Match
- Primary likely profile(s):
  - Name:
  - Handle/URL:
  - Location (if explicitly listed):
  - Current role (if explicitly listed):
- Disambiguation notes (only if needed):

## Evidence-Based Highlights
-

## Professional Experience (publicly stated)
-

## Skills & Technical/Domain Signals
-

## Notable Work / Content / Contributions
-

## Signals of Seniority / Impact
-

## Red Flags (public, professional)
- Only include if clearly evidenced in public professional context.
- Otherwise write: Unknown.

## Recruiter Takeaways (factual synthesis)
- 3-6 bullets that summarize only what is supported above.

## Open Questions
- 3-6 bullets phrased as questions for a human recruiter to verify.

If no reliable public information is found for {platform_name}, output:
- "Unknown" under each section and keep the headings.
"""


def _extract_tldr(content: str) -> str:
    """Extract the TLDR section from the markdown response."""
    import re

    # Look for ## TLDR section
    tldr_pattern = r"##\s*TLDR\s*\n+(.+?)(?=\n##|\Z)"
    match = re.search(tldr_pattern, content, re.IGNORECASE | re.DOTALL)

    if match:
        tldr = match.group(1).strip()
        # Remove any leading/trailing parentheses or extra whitespace
        tldr = re.sub(r"^\(|\)$", "", tldr).strip()
        # Take only the first line if multiple lines
        tldr = tldr.split("\n")[0].strip()
        return tldr

    return ""


class CandidateResearchService:
    """Service for researching candidates across platforms using Grok AI."""

    def __init__(self) -> None:
        """Initialize the candidate research service."""
        self.settings = get_settings()
        self.client = Client()

    def search_platform(
        self,
        person_name: str,
        platform: Platform,
        email: str | None = None,
        profile_url: str | None = None,
    ) -> PlatformReport:
        """
        Research a candidate on a specific platform.

        Args:
            person_name: Full name of the person to research
            platform: The platform to search
            email: Optional email address for disambiguation
            profile_url: Optional direct profile URL for accurate research

        Returns:
            PlatformReport with research results
        """
        platform_display = platform.value.title()
        if platform == Platform.STACKOVERFLOW:
            platform_display = "Stack Overflow"
        elif platform == Platform.TWITTER:
            platform_display = "Twitter/X"

        logger.info("Searching %s for: %s", platform_display, person_name)
        if profile_url:
            logger.info("Using direct profile URL: %s", profile_url)

        # Select appropriate tools based on platform
        # - x_search: for Twitter/X research (search posts, users, threads)
        # - web_search: for web-based platforms (GitHub, LinkedIn, StackOverflow)
        if platform == Platform.TWITTER:
            tools = [x_search(), web_search()]
            logger.info("Using x_search + web_search tools for Twitter research")
        else:
            tools = [web_search()]
            logger.info("Using web_search tool for %s research", platform_display)

        chat = self.client.chat.create(
            model="grok-4-1-fast",
            messages=[system(SYSTEM_PROMPT)],
            tools=tools,
        )

        prompt = _build_research_prompt(person_name, platform, email, profile_url)
        chat.append(user(prompt))

        logger.info("=" * 70)
        logger.info(
            "CANDIDATE RESEARCH: Sending request to Grok for %s...", platform_display
        )
        logger.info("=" * 70)

        response = chat.sample()

        logger.info("=" * 70)
        logger.info("CANDIDATE RESEARCH: Response received for %s", platform_display)
        completion_tokens = getattr(response.usage, "completion_tokens", 0)
        reasoning_tokens = getattr(response.usage, "reasoning_tokens", 0)
        logger.info(
            "Completion tokens: %d, Reasoning tokens: %d",
            completion_tokens,
            reasoning_tokens,
        )
        logger.info("=" * 70)

        # Parse TLDR from the response
        tldr = _extract_tldr(response.content)
        logger.info("Extracted TLDR: %s", tldr)

        return PlatformReport(
            platform=platform_display,
            person_name=person_name,
            tldr=tldr,
            raw_content=response.content,
            completion_tokens=completion_tokens,
            reasoning_tokens=reasoning_tokens,
        )

    def search_all_platforms(
        self, person_name: str, email: str | None = None
    ) -> list[PlatformReport]:
        """
        Research a candidate across all supported platforms.

        Args:
            person_name: Full name of the person to research
            email: Optional email address for disambiguation

        Returns:
            List of PlatformReports for each platform
        """
        reports = []
        for platform in Platform:
            report = self.search_platform(person_name, platform, email)
            reports.append(report)
        return reports

    @staticmethod
    def get_supported_platforms() -> list[dict[str, str]]:
        """Get list of supported platforms with their descriptions."""
        return [
            {
                "id": p.value,
                "name": (
                    p.value.title() if p != Platform.STACKOVERFLOW else "Stack Overflow"
                ),
                "focus": PLATFORM_FOCUS[p],
            }
            for p in Platform
        ]
