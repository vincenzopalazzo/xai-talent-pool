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
