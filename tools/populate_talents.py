#!/usr/bin/env python3
"""Populate talent pool from bay_area_tech_profiles.md"""

import re
import json
import uuid
import requests
from typing import Dict, List


def parse_markdown(file_path: str) -> List[Dict]:
    """Parse markdown file and extract talent profiles."""
    with open(file_path, 'r') as f:
        content = f.read()

    profiles = []
    pattern = r'## \d+\. (.+?)\n\n- \*\*Title:\*\* (.+?)\n- \*\*Location:\*\* (.+?)\n'

    matches = re.findall(pattern, content)

    for name, title, location in matches:
        profiles.append({
            'name': name.strip(),
            'title': title.strip(),
            'location': location.strip()
        })

    return profiles


def extract_skills(title: str) -> str:
    """Extract relevant skills from job title."""
    skills_map = {
        'software engineer': 'Python, JavaScript, Go, System Design, APIs',
        'frontend': 'React, TypeScript, JavaScript, CSS, HTML, UI/UX',
        'backend': 'Python, Go, Java, PostgreSQL, Redis, Microservices',
        'full stack': 'React, Node.js, Python, PostgreSQL, AWS, Docker',
        'data scientist': 'Python, SQL, Machine Learning, TensorFlow, Statistics',
        'data engineer': 'Python, Spark, SQL, Airflow, ETL, Data Warehousing',
        'ml engineer': 'Python, TensorFlow, PyTorch, MLOps, Kubernetes',
        'ai research': 'Python, PyTorch, Research, NLP, Computer Vision',
        'devops': 'Kubernetes, Docker, Terraform, AWS, CI/CD, Monitoring',
        'site reliability': 'Kubernetes, Go, Python, Prometheus, Terraform',
        'mobile': 'Swift, Kotlin, React Native, iOS, Android',
        'ios': 'Swift, Objective-C, UIKit, SwiftUI, Xcode',
        'android': 'Kotlin, Java, Jetpack, Android SDK',
        'security': 'Security, Cryptography, Penetration Testing, OWASP',
        'architect': 'System Design, Microservices, AWS, Architecture',
        'engineering manager': 'Leadership, Team Management, Agile, System Design',
        'director': 'Leadership, Strategy, Team Building, Product Development',
        'vp': 'Executive Leadership, Strategy, P&L, Org Design',
        'cto': 'Executive Leadership, Technology Strategy, Innovation',
        'cio': 'IT Strategy, Digital Transformation, Enterprise Architecture',
        'product manager': 'Product Strategy, Roadmap, Analytics, User Research',
        'research scientist': 'Research, Python, Publications, ML, Statistics',
        'applied scientist': 'Machine Learning, Python, Research, Production ML',
        'quantitative': 'Python, Statistics, Math, Finance, Algorithms',
        'solutions architect': 'AWS, System Design, Cloud Architecture, APIs',
        'technical lead': 'Leadership, System Design, Mentoring, Architecture',
        'principal': 'Architecture, Technical Leadership, System Design, Mentoring',
    }

    title_lower = title.lower()
    for key, skills in skills_map.items():
        if key in title_lower:
            return skills

    return 'Python, JavaScript, SQL, Git, Agile'


def get_experience_level(title: str) -> str:
    """Determine experience level from title."""
    title_lower = title.lower()

    if any(x in title_lower for x in ['cto', 'cio', 'vp', 'chief']):
        return '15+ years'
    elif any(x in title_lower for x in ['director', 'principal', 'senior director']):
        return '10+ years'
    elif any(x in title_lower for x in ['senior', 'lead', 'staff']):
        return '5-10 years'
    elif 'engineer' in title_lower or 'scientist' in title_lower:
        return '3-5 years'
    else:
        return '2-5 years'


def create_handle(name: str) -> str:
    """Create X handle from name."""
    # Remove suffixes and clean
    name = re.sub(r'\s+(Jr\.|Sr\.|III|II|I|V|IV|[A-Z]\.)$', '', name)
    parts = name.split()

    if len(parts) >= 2:
        # Use first initial + last name
        handle = parts[0][0].lower() + parts[-1].lower()
    else:
        handle = name.lower().replace(' ', '')

    # Remove special characters
    handle = re.sub(r'[^a-z0-9_]', '', handle)
    return handle[:15]  # X handle max length


def generate_email(name: str) -> str:
    """Generate email from name."""
    clean_name = name.lower().replace(' ', '').replace('.', '')
    clean_name = re.sub(r'[^a-z]', '', clean_name)
    return f"{clean_name}@example.com"


def create_talent(profile: Dict, api_url: str) -> bool:
    """Create talent via API."""
    talent_data = {
        'name': profile['name'],
        'email': generate_email(profile['name']),
        'handle': create_handle(profile['name']),
        'title': profile['title'],
        'location': profile['location'],
        'skills': extract_skills(profile['title']),
        'experience': get_experience_level(profile['title']),
        'bio': f"Experienced tech professional based in {profile['location']}",
        'verified': False,
        'avatar': f"https://api.dicebear.com/7.x/avataaars/svg?seed={profile['name'].replace(' ', '')}"
    }

    try:
        response = requests.post(api_url, json=talent_data)
        if response.status_code in [200, 201]:
            print(f"✓ Created: {profile['name']}")
            return True
        else:
            print(f"✗ Failed: {profile['name']} - {response.status_code} {response.text}")
            return False
    except Exception as e:
        print(f"✗ Error creating {profile['name']}: {e}")
        return False


def main():
    md_file = '/Users/pran-ker/Developer/xai-talent-pool/bay_area_tech_profiles.md'
    api_url = 'http://127.0.0.1:8080/api/v1/talents'

    print("Parsing markdown file...")
    profiles = parse_markdown(md_file)
    print(f"Found {len(profiles)} profiles\n")

    print("Creating talents...")
    success_count = 0
    for profile in profiles:
        if create_talent(profile, api_url):
            success_count += 1

    print(f"\nCompleted: {success_count}/{len(profiles)} talents created")


if __name__ == '__main__':
    main()
