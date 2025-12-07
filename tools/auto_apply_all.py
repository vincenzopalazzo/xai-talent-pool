#!/usr/bin/env python3
"""
Automatically apply all talent to all job postings.
"""

import requests
import logging
from typing import List, Dict

logging.basicConfig(
    level=logging.INFO,
    format='%(message)s'
)
logger = logging.getLogger(__name__)

API_BASE_URL = "http://localhost:8080/api/v1"

def get_all_talent() -> List[Dict]:
    """Fetch all talent from the API."""
    try:
        response = requests.get(f"{API_BASE_URL}/talents", timeout=10)
        response.raise_for_status()
        return response.json()
    except requests.exceptions.RequestException as e:
        logger.error(f"Failed to fetch talent: {e}")
        return []

def get_all_jobs() -> List[Dict]:
    """Fetch all jobs from the API."""
    try:
        response = requests.get(f"{API_BASE_URL}/jobs", timeout=10)
        response.raise_for_status()
        return response.json()
    except requests.exceptions.RequestException as e:
        logger.error(f"Failed to fetch jobs: {e}")
        return []

def get_existing_applications_for_talent(talent_id: str) -> List[Dict]:
    """Fetch existing applications for a talent."""
    try:
        response = requests.get(f"{API_BASE_URL}/applications/talent/{talent_id}", timeout=10)
        response.raise_for_status()
        return response.json()
    except requests.exceptions.RequestException as e:
        logger.error(f"Failed to fetch applications for talent {talent_id}: {e}")
        return []

def create_application(talent_id: str, job_id: str, talent_name: str, job_title: str, company_name: str) -> bool:
    """Create an application for a talent to a job."""
    application_data = {
        "talent_id": talent_id,
        "job_id": job_id,
        "cover_letter": f"I am very interested in the {job_title} position at {company_name}. I believe my skills and experience make me a great fit for this role."
    }

    try:
        response = requests.post(f"{API_BASE_URL}/applications", json=application_data, timeout=10)
        response.raise_for_status()
        logger.info(f"  ✓ Applied {talent_name} to {job_title} at {company_name}")
        return True
    except requests.exceptions.RequestException as e:
        logger.error(f"  ✗ Failed to apply {talent_name} to {job_title}: {e}")
        return False

def auto_apply_all():
    """Apply all talent to all jobs."""
    logger.info("Fetching talent and jobs...\n")

    talent_list = get_all_talent()
    jobs_list = get_all_jobs()

    if not talent_list:
        logger.error("No talent found in the system!")
        return

    if not jobs_list:
        logger.error("No jobs found in the system!")
        return

    logger.info(f"Found {len(talent_list)} talent and {len(jobs_list)} jobs\n")
    logger.info("Starting auto-application process...\n")

    total_applications = 0
    successful_applications = 0
    skipped_applications = 0

    for talent in talent_list:
        talent_id = talent['id']
        talent_name = talent.get('name', 'Unknown')

        logger.info(f"Processing talent: {talent_name}")

        # Get existing applications for this talent
        existing_apps = get_existing_applications_for_talent(talent_id)
        existing_job_ids = {app['job_id'] for app in existing_apps}

        for job in jobs_list:
            job_id = job['id']
            job_title = job.get('title', 'Unknown Position')
            company_name = job.get('company_name', 'Unknown Company')

            # Skip if already applied
            if job_id in existing_job_ids:
                logger.info(f"  ⊘ Already applied to {job_title} at {company_name}")
                skipped_applications += 1
                continue

            total_applications += 1
            if create_application(talent_id, job_id, talent_name, job_title, company_name):
                successful_applications += 1

        logger.info("")  # Empty line between talents

    logger.info("=" * 70)
    logger.info(f"Auto-application completed!")
    logger.info(f"Total new applications created: {successful_applications}")
    logger.info(f"Applications skipped (already applied): {skipped_applications}")
    logger.info(f"Failed applications: {total_applications - successful_applications}")
    logger.info("=" * 70)

if __name__ == "__main__":
    auto_apply_all()
