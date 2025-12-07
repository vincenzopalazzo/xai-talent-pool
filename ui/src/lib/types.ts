export interface ExperienceSummary {
	company: string;
	role: string;
	duration?: string;
	summary: string;
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
	// Social media research
	github_report_id?: string;
	linkedin_report_id?: string;
	twitter_report_id?: string;
	stackoverflow_report_id?: string;
	social_research_status?: 'pending' | 'in_progress' | 'completed' | 'failed' | null;
	// TLDR summaries for each platform
	github_tldr?: string;
	linkedin_tldr?: string;
	twitter_tldr?: string;
	stackoverflow_tldr?: string;
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