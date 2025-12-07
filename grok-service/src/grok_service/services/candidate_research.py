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


# Platform-specific focus areas optimized for AI/ML talent recruiting
PLATFORM_FOCUS = {
    Platform.GITHUB: {
        "focus": (
            "repositories, code contributions, programming languages, "
            "open-source activity, and collaboration patterns"
        ),
        "signals": [
            "AI/ML projects (PyTorch, TensorFlow, JAX, transformers)",
            "Systems programming (Rust, C++, CUDA)",
            "Code quality and documentation practices",
            "Contribution frequency and consistency",
            "Collaboration style (PRs, code reviews, issues)",
            "Project complexity and scale",
        ],
    },
    Platform.LINKEDIN: {
        "focus": (
            "career trajectory, company caliber, role progression, "
            "education, and professional network"
        ),
        "signals": [
            "Tenure patterns (job-hopping vs stability)",
            "Company tier (FAANG, top startups, research labs)",
            "Role progression (IC track vs management)",
            "Education (top CS programs, PhD research)",
            "Publications or patents",
            "Leadership and team size managed",
        ],
    },
    Platform.TWITTER: {
        "focus": (
            "technical discussions, AI community engagement, "
            "thought leadership, and professional tone"
        ),
        "signals": [
            "AI/ML technical discussions and insights",
            "Engagement with AI research papers",
            "Interactions with xAI, OpenAI, Anthropic content",
            "Professional vs personal tone balance",
            "Community reputation and follower quality",
            "Original technical content vs retweets",
        ],
    },
    Platform.STACKOVERFLOW: {
        "focus": (
            "technical expertise depth, problem-solving ability, "
            "and community standing"
        ),
        "signals": [
            "Reputation score and badges",
            "Top tags (ML, deep-learning, python, etc.)",
            "Answer quality and acceptance rate",
            "Question complexity handled",
            "Teaching ability in explanations",
            "Breadth vs depth of expertise",
        ],
    },
}

SYSTEM_PROMPT = """You are an AI talent researcher for xAI's recruiting team.
Your job is to help recruiters evaluate candidates for AI/ML engineering roles.

TOOLS:
- Use web_search for GitHub, LinkedIn, StackOverflow profiles
- Use x_search for X/Twitter profiles and posts

EVALUATION LENS (xAI context):
- Technical depth in AI/ML, systems programming, or infrastructure
- Evidence of building and shipping at scale
- Clear technical communication
- Collaborative work style
- Intellectual curiosity and continuous learning

RULES:
- Only report verifiable facts from search results
- Include source URLs for all claims
- Write "Unknown" if information cannot be confirmed
- Be direct and concise - recruiters are busy
- No speculation, no filler, no generic advice
- Highlight both strengths AND concerns objectively"""


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

    platform_config = PLATFORM_FOCUS[platform]
    focus = platform_config["focus"]
    signals = "\n".join(f"  - {s}" for s in platform_config["signals"])

    # Build context hints
    context_hints = []
    if profile_url:
        context_hints.append(f"PROFILE URL: {profile_url}")
        context_hints.append("Search this exact URL first.")
    if email:
        domain = email.split("@")[-1] if "@" in email else ""
        context_hints.append(f"EMAIL: {email}")
        if domain:
            context_hints.append(f"(Domain '{domain}' may indicate employer)")

    context_block = "\n".join(context_hints) if context_hints else ""

    # Tool instruction based on platform
    if platform == Platform.TWITTER:
        tool_hint = "Use x_search to find posts and profile. Use web_search too."
    else:
        tool_hint = f"Use web_search to find their {platform_name} profile."

    return f"""Research {person_name} on {platform_name} for xAI recruiting.

{context_block}

SEARCH: {tool_hint}

WHAT TO LOOK FOR on {platform_name}:
{focus}

KEY SIGNALS TO EVALUATE:
{signals}

OUTPUT FORMAT (Markdown):

## TLDR
One sentence: [Strength level: Strong/Moderate/Weak/Unknown] - [Key finding]

## Profile Match
- Name:
- URL/Handle:
- Location:
- Current Role:
- Confidence: [High/Medium/Low] (explain if ambiguous)

## Technical Signals
- [List specific technical evidence found]

## Experience & Impact
- [Career level, company caliber, scope of work]

## Strengths (for xAI)
- [Top 3 strengths relevant to AI/ML roles]

## Concerns
- [Any red flags or gaps - write "None identified" if clean]

## Recruiter Action Items
- [2-3 specific things to verify or discuss in interview]

If profile not found, write "Profile not found on {platform_name}" and skip sections."""


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
                "focus": PLATFORM_FOCUS[p]["focus"],
            }
            for p in Platform
        ]
