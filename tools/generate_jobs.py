#!/usr/bin/env python3
"""
Generate and post multiple job listings to the talent pool application.
"""

import requests
import random
from typing import List, Dict
from datetime import datetime, timedelta

API_BASE_URL = "http://localhost:8080/api/v1"

# Job data templates
COMPANIES = [
    {"name": "TechCorp", "logo": "https://via.placeholder.com/100x100?text=TC"},
    {"name": "DataSystems Inc", "logo": "https://via.placeholder.com/100x100?text=DS"},
    {"name": "CloudNine Solutions", "logo": "https://via.placeholder.com/100x100?text=C9"},
    {"name": "AI Innovations", "logo": "https://via.placeholder.com/100x100?text=AI"},
    {"name": "SecureNet Technologies", "logo": "https://via.placeholder.com/100x100?text=SN"},
    {"name": "FinTech Global", "logo": "https://via.placeholder.com/100x100?text=FG"},
    {"name": "HealthTech Partners", "logo": "https://via.placeholder.com/100x100?text=HT"},
    {"name": "EduLearn Systems", "logo": "https://via.placeholder.com/100x100?text=EL"},
    {"name": "GreenEnergy Co", "logo": "https://via.placeholder.com/100x100?text=GE"},
    {"name": "RetailNext", "logo": "https://via.placeholder.com/100x100?text=RN"},
    {"name": "MediaStream Labs", "logo": "https://via.placeholder.com/100x100?text=MS"},
    {"name": "GameDev Studios", "logo": "https://via.placeholder.com/100x100?text=GD"},
    {"name": "CyberDefense Corp", "logo": "https://via.placeholder.com/100x100?text=CD"},
    {"name": "Quantum Computing Inc", "logo": "https://via.placeholder.com/100x100?text=QC"},
    {"name": "RoboTech Industries", "logo": "https://via.placeholder.com/100x100?text=RT"},
]

JOB_TITLES = {
    "engineering": [
        "Senior Software Engineer",
        "Frontend Developer",
        "Backend Engineer",
        "Full Stack Developer",
        "DevOps Engineer",
        "Site Reliability Engineer",
        "Machine Learning Engineer",
        "Data Engineer",
        "Security Engineer",
        "Mobile Developer",
        "Cloud Architect",
        "Platform Engineer",
    ],
    "data": [
        "Data Scientist",
        "Data Analyst",
        "Business Intelligence Analyst",
        "Analytics Engineer",
        "ML Research Scientist",
    ],
    "product": [
        "Product Manager",
        "Senior Product Manager",
        "Technical Product Manager",
        "Product Designer",
        "UX Researcher",
    ],
    "management": [
        "Engineering Manager",
        "Technical Lead",
        "Director of Engineering",
        "VP of Engineering",
    ],
}

LOCATIONS = [
    "San Francisco, CA",
    "New York, NY",
    "Seattle, WA",
    "Austin, TX",
    "Boston, MA",
    "Denver, CO",
    "Portland, OR",
    "Chicago, IL",
    "Los Angeles, CA",
    "Miami, FL",
    "Atlanta, GA",
    "Raleigh, NC",
]

LOCATION_TYPES = ["remote", "onsite", "hybrid"]
EMPLOYMENT_TYPES = ["full-time", "part-time", "contract"]
EXPERIENCE_LEVELS = ["entry", "mid", "senior", "lead"]

SKILLS_BY_CATEGORY = {
    "frontend": ["React", "TypeScript", "Vue.js", "Angular", "Svelte", "CSS", "HTML", "JavaScript", "Tailwind CSS"],
    "backend": ["Python", "Rust", "Go", "Java", "Node.js", "PostgreSQL", "MongoDB", "Redis", "Kafka"],
    "devops": ["Docker", "Kubernetes", "AWS", "GCP", "Azure", "Terraform", "Jenkins", "GitLab CI"],
    "data": ["Python", "SQL", "Spark", "Airflow", "TensorFlow", "PyTorch", "Pandas", "NumPy"],
    "mobile": ["Swift", "Kotlin", "React Native", "Flutter", "iOS", "Android"],
}

JOB_DESCRIPTIONS = {
    "engineering": "We're looking for a talented engineer to join our growing team. You'll work on cutting-edge technology, collaborate with cross-functional teams, and help build products that impact millions of users. The ideal candidate is passionate about clean code, scalable architecture, and continuous improvement.",
    "data": "Join our data team to help drive insights and decision-making across the organization. You'll work with large datasets, build predictive models, and communicate findings to stakeholders. Experience with statistical analysis and data visualization is essential.",
    "product": "We're seeking a product professional to help shape the future of our platform. You'll work closely with engineering, design, and business teams to define product strategy, prioritize features, and deliver exceptional user experiences.",
    "management": "Lead and grow a team of talented engineers while contributing to technical strategy and architecture. You'll mentor team members, remove blockers, and foster a culture of innovation and excellence.",
}

def generate_job(category: str = None) -> Dict:
    """Generate a single job posting."""
    if category is None:
        category = random.choice(list(JOB_TITLES.keys()))

    company = random.choice(COMPANIES)
    title = random.choice(JOB_TITLES[category])
    location_type = random.choice(LOCATION_TYPES)
    employment_type = random.choice(EMPLOYMENT_TYPES)
    experience_level = random.choice(EXPERIENCE_LEVELS)

    # Select appropriate skills based on job title
    if "Frontend" in title or "Full Stack" in title:
        skill_pool = SKILLS_BY_CATEGORY["frontend"] + SKILLS_BY_CATEGORY["backend"][:3]
    elif "Backend" in title or "Full Stack" in title:
        skill_pool = SKILLS_BY_CATEGORY["backend"] + SKILLS_BY_CATEGORY["frontend"][:2]
    elif "DevOps" in title or "SRE" in title or "Cloud" in title or "Platform" in title:
        skill_pool = SKILLS_BY_CATEGORY["devops"]
    elif "Data" in title or "ML" in title or "Machine Learning" in title:
        skill_pool = SKILLS_BY_CATEGORY["data"]
    elif "Mobile" in title:
        skill_pool = SKILLS_BY_CATEGORY["mobile"]
    else:
        skill_pool = SKILLS_BY_CATEGORY["backend"]

    skills = random.sample(skill_pool, min(random.randint(4, 7), len(skill_pool)))

    # Generate salary based on experience level
    salary_ranges = {
        "entry": (60000, 100000),
        "mid": (90000, 150000),
        "senior": (130000, 200000),
        "lead": (160000, 250000),
    }
    min_sal, max_sal = salary_ranges[experience_level]
    salary_min = random.randrange(min_sal, max_sal - 20000, 5000)
    salary_max = random.randrange(salary_min + 20000, max_sal, 5000)

    # Generate expiration date (30-90 days from now)
    expires_days = random.randint(30, 90)
    expires_at = (datetime.now() + timedelta(days=expires_days)).isoformat()

    job = {
        "title": title,
        "description": JOB_DESCRIPTIONS[category],
        "company_name": company["name"],
        "company_logo": company["logo"],
        "location": random.choice(LOCATIONS) if location_type != "remote" else None,
        "location_type": location_type,
        "employment_type": employment_type,
        "salary_min": salary_min,
        "salary_max": salary_max,
        "salary_currency": "USD",
        "skills_required": ",".join(skills),
        "experience_level": experience_level,
        "expires_at": expires_at,
    }

    return job

def post_job(job: Dict) -> bool:
    """Post a job to the API."""
    try:
        response = requests.post(f"{API_BASE_URL}/jobs", json=job, timeout=10)
        response.raise_for_status()
        print(f"✓ Created: {job['title']} at {job['company_name']}")
        return True
    except requests.exceptions.RequestException as e:
        print(f"✗ Failed to create {job['title']}: {e}")
        return False

def generate_and_post_jobs(count: int = 50):
    """Generate and post multiple jobs."""
    print(f"Generating {count} job postings...\n")

    successful = 0
    failed = 0

    for i in range(count):
        job = generate_job()
        if post_job(job):
            successful += 1
        else:
            failed += 1

    print(f"\n{'='*60}")
    print(f"Results: {successful} successful, {failed} failed")
    print(f"{'='*60}")

if __name__ == "__main__":
    import sys

    count = 50
    if len(sys.argv) > 1:
        try:
            count = int(sys.argv[1])
        except ValueError:
            print(f"Invalid count: {sys.argv[1]}, using default of 50")

    generate_and_post_jobs(count)
