export interface ExperienceSummary {
	company: string;
	role: string;
	duration?: string;
	summary: string;
}

export interface PlatformProfile {
	platform: string;
	handle?: string;
	url?: string;
	verified: boolean;
	bio?: string;
	tldr?: string;
	highlights: string[];
	skills: string[];
	experience_signals: string[];
	red_flags: string[];
	recruiter_notes: string[];
}

export interface SocialMediaAnalysis {
	talent_id: string;
	x_handle?: string;
	tldr?: string;
	profiles: PlatformProfile[];
	combined_skills: string[];
	summary?: string;
}

export interface ScoringBreakdown {
	skills_match: number;
	experience_fit: number;
	culture_fit: number;
	overall_impression: number;
}

export interface CandidateScoreDetails {
	talent_id: string;
	job_id: string;
	overall_score: number;
	breakdown: ScoringBreakdown;
	strengths: string[];
	concerns: string[];
	recommendation: string; // strong_yes, yes, maybe, no
	summary: string;
	timestamp: string;
}

export interface Talent {
	id: string;
	name: string;
	email: string;
	handle: string;
	avatar?: string;
	title: string;
	location?: string;
	experience: string;
	skills: string[];
	bio?: string;
	verified: boolean;
	created_at: string;
	saved?: boolean; // local UI state
	// Grok-extracted resume fields
	resume_experiences?: string; // JSON array of ExperienceSummary
	linkedin_url?: string;
	x_url?: string;
	github_url?: string;
	gitlab_url?: string;
	// xAI Collections integration
	collection_id?: string;
	resume_document_id?: string;
	// Social Media Analysis
	social_analysis?: string; // JSON string of SocialMediaAnalysis
	x_handle_discovered?: string;
	// Candidate Scoring
	candidate_score?: number;
	candidate_score_details?: string; // JSON string of CandidateScoreDetails
}

export interface Job {
	id: string;
	title: string;
	description: string;
	company_name: string;
	company_logo?: string;
	location?: string;
	location_type: string; // remote, onsite, hybrid
	employment_type: string; // full-time, part-time, contract
	salary_min?: number;
	salary_max?: number;
	salary_currency?: string;
	skills_required: string; // comma-separated from API
	experience_level: string; // entry, mid, senior, lead
	status: string; // active, closed, draft
	created_at: string;
	expires_at?: string;
	saved?: boolean; // local UI state
}

export interface Application {
	id: string;
	talent_id: string;
	job_id: string;
	has_resume: boolean;
	resume_filename?: string;
	cover_letter?: string;
	status: string; // pending, reviewed, accepted, rejected
	created_at: string;
	// Populated from talent lookup
	talent?: Talent;
}