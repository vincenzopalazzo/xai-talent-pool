"""Pydantic models for Grok service."""

from pydantic import BaseModel, EmailStr, Field


class TalentInfo(BaseModel):
    """Talent information for screening."""

    id: str = Field(..., description="Unique talent identifier")
    name: str = Field(..., description="Full name of the talent")
    email: EmailStr = Field(..., description="Email address")
    handle: str = Field(..., description="X handle")
    skills: str = Field(..., description="Comma-separated list of skills")
    title: str = Field(..., description="Professional title")
    location: str | None = Field(None, description="Location")
    experience: str = Field(..., description="Experience level")
    bio: str | None = Field(None, description="Biography")


class ExperienceSummary(BaseModel):
    """Summary of a work experience from resume."""

    company: str = Field(..., description="Company name")
    role: str = Field(..., description="Job title/role")
    duration: str | None = Field(
        None, description="Duration (e.g., Jan 2020 - Dec 2022)"
    )
    summary: str = Field(..., description="Brief summary of responsibilities")


class ProfileUrls(BaseModel):
    """Profile URLs extracted from resume."""

    linkedin: str | None = Field(None, description="LinkedIn profile URL")
    x: str | None = Field(None, description="X (Twitter) profile URL")
    github: str | None = Field(None, description="GitHub profile URL")
    gitlab: str | None = Field(None, description="GitLab profile URL")


class ScreeningResult(BaseModel):
    """Result of the initial screening."""

    talent_id: str = Field(..., description="ID of the screened talent")
    experiences: list[ExperienceSummary] = Field(
        default_factory=list, description="List of work experiences from resume"
    )
    urls: ProfileUrls = Field(
        default_factory=ProfileUrls, description="Profile URLs found in resume"
    )


class ScreeningResponse(BaseModel):
    """Response model for screening endpoint."""

    success: bool = Field(..., description="Whether the screening was successful")
    result: ScreeningResult | None = Field(None, description="Screening result")
    error: str | None = Field(None, description="Error message if failed")


# Collection models


class CreateCollectionRequest(BaseModel):
    """Request to create a collection for a talent."""

    talent_id: str = Field(..., description="Unique talent identifier")
    talent_name: str = Field(..., description="Talent name for collection naming")


class CollectionInfo(BaseModel):
    """Collection information."""

    collection_id: str = Field(..., description="The xAI collection ID")
    collection_name: str = Field(..., description="The collection name")


class CollectionResponse(BaseModel):
    """Response model for collection endpoints."""

    success: bool = Field(..., description="Whether the operation was successful")
    collection: CollectionInfo | None = Field(None, description="Collection info")
    error: str | None = Field(None, description="Error message if failed")


# Document models


class UploadDocumentRequest(BaseModel):
    """Request to upload a document to a collection."""

    collection_id: str = Field(..., description="The collection ID to upload to")
    document_name: str = Field(..., description="Name of the document")
    old_document_id: str | None = Field(
        None, description="Previous document ID to delete"
    )


class DocumentInfo(BaseModel):
    """Document information."""

    document_id: str = Field(..., description="The document ID in the collection")
    document_name: str = Field(..., description="The document name")


class DocumentResponse(BaseModel):
    """Response model for document endpoints."""

    success: bool = Field(..., description="Whether the operation was successful")
    document: DocumentInfo | None = Field(None, description="Document info")
    error: str | None = Field(None, description="Error message if failed")


class SocialMediaInput(ProfileUrls):
    """Input URLs for social media analysis."""

    stackoverflow: str | None = Field(None, description="StackOverflow profile URL")


class PlatformProfile(BaseModel):
    """Analysis result for a single social media platform."""

    platform: str = Field(..., description="Platform name (X, LinkedIn, etc.)")
    handle: str | None = Field(None, description="Username/handle on the platform")
    url: str | None = Field(None, description="Profile URL")
    verified: bool = Field(False, description="Whether the profile is verified")
    bio: str | None = Field(None, description="Profile bio/summary")
    tldr: str | None = Field(None, description="TLDR summary for this platform")
    highlights: list[str] = Field(default_factory=list, description="Key highlights")
    skills: list[str] = Field(default_factory=list, description="Identified skills")
    experience_signals: list[str] = Field(
        default_factory=list, description="Signals of experience/seniority"
    )
    red_flags: list[str] = Field(
        default_factory=list, description="Potential red flags"
    )
    recruiter_notes: list[str] = Field(
        default_factory=list, description="Notes for recruiters"
    )


class SocialMediaAnalysisResult(BaseModel):
    """Result of social media analysis."""

    talent_id: str = Field(..., description="Talent ID")
    timestamp: str = Field(..., description="Analysis timestamp (ISO 8601)")
    x_handle: str | None = Field(None, description="X handle (system nickname)")
    tldr: str | None = Field(None, description="Overall TLDR summary")
    profiles: list[PlatformProfile] = Field(
        default_factory=list, description="Analyzed profiles"
    )
    combined_skills: list[str] = Field(
        default_factory=list, description="Combined skills from all platforms"
    )
    summary: str | None = Field(None, description="Overall summary")


class SocialMediaAnalysisRequest(BaseModel):
    """Request for social media analysis."""

    talent_id: str = Field(..., description="Talent ID")
    collection_id: str | None = Field(
        None, description="Collection ID to store the analysis"
    )
    name: str = Field(..., description="Candidate name")
    email: str | None = Field(None, description="Candidate email")
    social_urls: SocialMediaInput = Field(
        default_factory=SocialMediaInput, description="Known social media URLs"
    )
    platforms_to_search: list[str] = Field(
        default_factory=lambda: ["X", "GitHub", "LinkedIn"],
        description="List of platforms to search if URL not provided",
    )


class SocialMediaAnalysisResponse(BaseModel):
    """Response for social media analysis."""

    success: bool = Field(..., description="Whether the analysis was successful")
    result: SocialMediaAnalysisResult | None = Field(None, description="Analysis result")
    error: str | None = Field(None, description="Error message if failed")
