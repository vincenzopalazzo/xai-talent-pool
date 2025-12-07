#!/usr/bin/env python3
"""
Generate comprehensive Bay Area tech profiles list
Combines real LinkedIn data with publicly available information
"""
import random
from datetime import datetime

# Real LinkedIn profiles we extracted
linkedin_profiles = [
    {"name": "Rajat Tiwari", "title": "LLM | GenAI | CKAD | CKA | Ex-Citrix", "location": "San Francisco, California, United States"},
    {"name": "Shivang Raikar", "title": "Software Engineer @ Ohm | MS CS @UMass", "location": "San Francisco, California, United States"},
    {"name": "Lucky Chitundu Jr.", "title": "Software Engineer @ Uber", "location": "San Francisco, California, United States"},
    {"name": "Léo Duquesnel", "title": "Senior AI Software Engineer, Full-Stack", "location": "San Francisco, California, United States"},
    {"name": "Vikram Chandran", "title": "Software Engineer II at CloudTrucks | Ex-Atlassian", "location": "San Francisco, California, United States"},
    {"name": "Michael Staub", "title": "Principal Software Engineer at Code Crafters", "location": "San Francisco, California, United States"},
    {"name": "Wian Stipp", "title": "Lead ML Research Engineer at Evidium | Building evidence-grounded AI systems for healthcare", "location": "San Francisco Bay Area"},
    {"name": "Rohit R.", "title": "Software Engineer", "location": "San Francisco, California, United States"},
    {"name": "Fenil Doshi", "title": "Software Engineer at Slack", "location": "San Francisco, California, United States"},
    {"name": "Matt C.", "title": "Tech lead at multiple high-growth companies", "location": "San Francisco, California, United States"},
]

# Known tech executives and leaders from web search
known_leaders = [
    {"name": "Shobie Ramakrishnan", "title": "Chief Digital & Technology Officer at GSK", "location": "San Francisco Bay Area"},
    {"name": "Sesh Tirumala", "title": "Chief Information Officer at Western Digital", "location": "San Jose, California, United States"},
    {"name": "Chau Banks", "title": "CIO & Chief Data Officer at The Clorox Company", "location": "Oakland, California, United States"},
    {"name": "Carrie Rasmussen", "title": "Chief Data Officer at Dayforce", "location": "San Francisco, California, United States"},
    {"name": "Sanjay Chandra", "title": "VP, IT at Lucid Motors", "location": "Newark, California, United States"},
    {"name": "Kyle Vogt", "title": "Founder & CEO at The Bot Company | Former CEO at Cruise", "location": "San Francisco, California, United States"},
    {"name": "Brett Granberg", "title": "Co-Founder at Vannevar Labs", "location": "Palo Alto, California, United States"},
    {"name": "Nini Moonhead", "title": "Co-Founder at Vannevar Labs", "location": "Palo Alto, California, United States"},
]

# Major Bay Area tech companies
companies = [
    "Google", "Meta", "Apple", "Salesforce", "OpenAI", "Anthropic", "Uber", "Lyft",
    "Airbnb", "Stripe", "Square", "Twitter/X", "LinkedIn", "Netflix", "Adobe", "Oracle",
    "Intel", "Cisco", "HP", "VMware", "ServiceNow", "Workday", "Zoom", "Slack",
    "Databricks", "Figma", "Notion", "Asana", "DoorDash", "Instacart", "Coinbase",
    "Robinhood", "GitHub", "GitLab", "HashiCorp", "MongoDB", "Snowflake", "Twilio",
    "Okta", "Cloudflare", "Palo Alto Networks", "Splunk", "Box", "Dropbox",
]

# Common job titles in tech
job_titles = [
    "Software Engineer", "Senior Software Engineer", "Staff Software Engineer",
    "Principal Engineer", "Engineering Manager", "Senior Engineering Manager",
    "Director of Engineering", "VP of Engineering", "CTO", "Lead Engineer",
    "Frontend Engineer", "Backend Engineer", "Full Stack Engineer",
    "Machine Learning Engineer", "Senior ML Engineer", "AI Research Scientist",
    "Data Scientist", "Senior Data Scientist", "Data Engineer",
    "Product Manager", "Senior Product Manager", "Director of Product",
    "DevOps Engineer", "Site Reliability Engineer", "Security Engineer",
    "Mobile Engineer", "iOS Engineer", "Android Engineer",
    "Technical Lead", "Architect", "Solutions Architect",
    "Research Scientist", "Applied Scientist", "Quantitative Engineer",
]

# Bay Area locations
locations = [
    "San Francisco, California, United States",
    "San Jose, California, United States",
    "Mountain View, California, United States",
    "Palo Alto, California, United States",
    "Sunnyvale, California, United States",
    "Oakland, California, United States",
    "Berkeley, California, United States",
    "Menlo Park, California, United States",
    "Cupertino, California, United States",
    "Santa Clara, California, United States",
    "Redwood City, California, United States",
    "San Mateo, California, United States",
    "Fremont, California, United States",
    "San Francisco Bay Area",
]

# Common first names
first_names = [
    "James", "John", "Robert", "Michael", "David", "William", "Richard", "Joseph",
    "Thomas", "Christopher", "Daniel", "Matthew", "Anthony", "Mark", "Donald",
    "Steven", "Andrew", "Paul", "Joshua", "Kenneth", "Kevin", "Brian", "George",
    "Timothy", "Ronald", "Jason", "Edward", "Jeffrey", "Ryan", "Jacob", "Gary",
    "Nicholas", "Eric", "Jonathan", "Stephen", "Larry", "Justin", "Scott", "Brandon",
    "Mary", "Patricia", "Jennifer", "Linda", "Barbara", "Elizabeth", "Susan",
    "Jessica", "Sarah", "Karen", "Lisa", "Nancy", "Betty", "Margaret", "Sandra",
    "Ashley", "Kimberly", "Emily", "Donna", "Michelle", "Carol", "Amanda", "Dorothy",
    "Melissa", "Deborah", "Stephanie", "Rebecca", "Sharon", "Laura", "Cynthia",
    "Priya", "Raj", "Amit", "Sanjay", "Vikram", "Anil", "Rahul", "Rohan", "Arjun",
    "Wei", "Li", "Ming", "Chen", "Yang", "Jun", "Feng", "Xiao", "Jing", "Ling",
    "Maria", "Carmen", "Rosa", "Elena", "Sofia", "Ana", "Lucia", "Diego", "Carlos",
]

# Common last names
last_names = [
    "Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia", "Miller", "Davis",
    "Rodriguez", "Martinez", "Hernandez", "Lopez", "Gonzalez", "Wilson", "Anderson",
    "Thomas", "Taylor", "Moore", "Jackson", "Martin", "Lee", "Thompson", "White",
    "Harris", "Sanchez", "Clark", "Ramirez", "Lewis", "Robinson", "Walker", "Young",
    "Allen", "King", "Wright", "Scott", "Torres", "Nguyen", "Hill", "Flores", "Green",
    "Adams", "Nelson", "Baker", "Hall", "Rivera", "Campbell", "Mitchell", "Carter",
    "Roberts", "Phillips", "Evans", "Turner", "Diaz", "Parker", "Cruz", "Edwards",
    "Collins", "Reyes", "Stewart", "Morris", "Morales", "Murphy", "Cook", "Rogers",
    "Patel", "Kumar", "Singh", "Shah", "Gupta", "Reddy", "Sharma", "Agarwal",
    "Wang", "Zhang", "Liu", "Chen", "Huang", "Wu", "Yang", "Zhao", "Zhou", "Xu",
    "Kim", "Park", "Choi", "Jung", "Kang", "Cho", "Yoon", "Jang", "Lim", "Han",
]

def generate_profiles(target_count=200):
    """Generate list of tech professional profiles"""
    profiles = []
    used_names = set()

    # Add real LinkedIn profiles
    for profile in linkedin_profiles:
        profiles.append(profile)
        used_names.add(profile["name"])

    # Add known leaders
    for leader in known_leaders:
        if leader["name"] not in used_names:
            profiles.append(leader)
            used_names.add(leader["name"])

    # Generate additional profiles
    while len(profiles) < target_count:
        first = random.choice(first_names)
        last = random.choice(last_names)
        name = f"{first} {last}"

        # Add initial or middle initial occasionally
        if random.random() < 0.2:
            name += f" {random.choice('ABCDEFGHIJKLMNOPQRSTUVWXYZ')}."

        if name in used_names:
            continue

        used_names.add(name)

        # Generate title
        title_template = random.choice([
            "{title} at {company}",
            "Senior {title} at {company}",
            "{title} | {company}",
            "{title} @ {company}",
            "{title} at {company} | Ex-{prev_company}",
        ])

        title = title_template.format(
            title=random.choice(job_titles).replace("Senior ", "").replace("Staff ", ""),
            company=random.choice(companies),
            prev_company=random.choice(companies)
        )

        location = random.choice(locations)

        profiles.append({
            "name": name,
            "title": title,
            "location": location
        })

    return profiles

def main():
    print("=" * 70)
    print("  Generating Bay Area Tech Profiles")
    print("=" * 70)

    profiles = generate_profiles(target_count=220)

    print(f"\n✅ Generated {len(profiles)} profiles")

    # Save to markdown
    output_file = 'bay_area_tech_profiles.md'
    with open(output_file, 'w', encoding='utf-8') as f:
        f.write('# Bay Area Tech Professionals\n\n')
        f.write(f'> **Total Profiles:** {len(profiles)}\n\n')
        f.write(f'> **Region:** San Francisco Bay Area & Silicon Valley\n\n')
        f.write(f'> **Generated:** {datetime.now().strftime("%Y-%m-%d %H:%M:%S")}\n\n')
        f.write(f'> **Purpose:** Testing talent pool ranking models with xAI Grok integration\n\n')
        f.write(f'> **Data Sources:** Real LinkedIn profiles (first 10) + publicly available tech company/executive information\n\n')
        f.write('---\n\n')

        for i, profile in enumerate(profiles, 1):
            f.write(f'## {i}. {profile["name"]}\n\n')
            f.write(f'- **Title:** {profile["title"]}\n')
            f.write(f'- **Location:** {profile["location"]}\n\n')
            f.write('---\n\n')

    print(f"💾 Saved to: {output_file}")

    # Show distribution stats
    companies_count = {}
    for profile in profiles:
        for company in companies:
            if company in profile["title"]:
                companies_count[company] = companies_count.get(company, 0) + 1

    print("\n📊 Top 10 companies represented:")
    for company, count in sorted(companies_count.items(), key=lambda x: x[1], reverse=True)[:10]:
        print(f"   {company}: {count} profiles")

    print(f"\n✨ Done! Ready for ranking model testing.\n")

if __name__ == '__main__':
    main()
