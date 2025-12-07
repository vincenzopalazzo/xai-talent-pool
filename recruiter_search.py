#!/usr/bin/env python3
"""
Recruiter Profile Search Tool
Uses XAI SDK to search for a person's information across multiple social media platforms
and generates separate documents for each platform.
"""

import os
import argparse
from pathlib import Path
from datetime import datetime

from xai_sdk import Client
from xai_sdk.chat import system, user


def search_platform(client, person_name, platform_name, platform_focus):
    """Search for a person on a specific platform using XAI SDK"""
    print(f"\nSearching {platform_name}...")

    chat = client.chat.create(
        model="grok-3-mini",
        reasoning_effort="high",
        messages=[
            system("You are a professional recruiter's research assistant. Provide detailed, factual information about individuals on social media platforms that would be relevant for job recruiting purposes."),
        ],
    )

    prompt = f"""Search for information about {person_name} on {platform_name}.

Focus on {platform_focus}.

Provide a comprehensive report including:
1. Profile information (if publicly available)
2. Professional background and experience
3. Skills and expertise demonstrated
4. Notable projects, contributions, or content
5. Professional network and connections (if visible)
6. Any other relevant information for a job recruiter

If you cannot find specific information, provide guidance on what a recruiter should look for on {platform_name}."""

    chat.append(user(prompt))
    response = chat.sample()

    print(f"  ✓ Completed ({response.usage.completion_tokens} tokens, {response.usage.reasoning_tokens} reasoning tokens)")

    return {
        'content': response.content,
        'completion_tokens': response.usage.completion_tokens,
        'reasoning_tokens': response.usage.reasoning_tokens
    }


def save_document(output_dir, platform_name, person_name, data):
    """Save the search results to a markdown document"""
    filename = output_dir / f"{platform_name.lower().replace('/', '_')}.md"

    with open(filename, 'w') as f:
        f.write(f"# {platform_name} Profile Report\n\n")
        f.write(f"**Candidate:** {person_name}\n\n")
        f.write(f"**Generated:** {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}\n\n")
        f.write("---\n\n")
        f.write(data['content'])
        f.write("\n\n---\n\n")
        f.write(f"*Report generated using AI analysis*\n")
        f.write(f"*Completion tokens: {data['completion_tokens']}, Reasoning tokens: {data['reasoning_tokens']}*\n")

    print(f"  → Saved to {filename}")


def create_summary(output_dir, person_name, platforms_data):
    """Create a summary document with all platform links"""
    filename = output_dir / "summary.md"

    with open(filename, 'w') as f:
        f.write(f"# Recruitment Research Summary\n\n")
        f.write(f"**Candidate:** {person_name}\n\n")
        f.write(f"**Generated:** {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}\n\n")
        f.write("---\n\n")
        f.write("## Platform Reports\n\n")

        for platform in platforms_data.keys():
            platform_file = f"{platform.lower().replace('/', '_')}.md"
            f.write(f"- [{platform}](./{platform_file})\n")

        f.write("\n## Quick Overview\n\n")
        f.write("Review each platform-specific document for detailed information about the candidate's:\n\n")
        f.write("- **GitHub**: Technical skills, projects, and open-source contributions\n")
        f.write("- **LinkedIn**: Professional experience, education, and career history\n")
        f.write("- **Twitter/X**: Professional presence, thought leadership, and industry engagement\n")
        f.write("- **Stack Overflow**: Technical expertise and community contributions\n")
        f.write("\n---\n\n")
        f.write("*All reports generated using AI analysis for recruiting purposes*\n")

    print(f"  → Saved to {filename}")


def main():
    parser = argparse.ArgumentParser(
        description='Search for a person across social media platforms for recruiting purposes using XAI SDK'
    )
    parser.add_argument('name', help='Full name of the person to search for')
    parser.add_argument('--output-dir', default='search_results', help='Directory to save results')

    args = parser.parse_args()

    # Check for API key
    api_key = ""

    # Create XAI client
    client = Client(
        api_key=api_key,
        timeout=3600,
    )

    # Create output directory
    output_dir = Path(args.output_dir) / args.name.replace(' ', '_')
    output_dir.mkdir(parents=True, exist_ok=True)

    print("=" * 60)
    print(f"RECRUITER SEARCH TOOL")
    print("=" * 60)
    print(f"Candidate: {args.name}")
    print(f"Output Directory: {output_dir}")
    print("=" * 60)

    # Define platforms to search
    platforms = {
        'GitHub': 'technical projects, repositories, code contributions, programming languages, and open-source activity',
        'LinkedIn': 'professional experience, job history, education, skills, endorsements, and recommendations',
        'Twitter/X': 'professional tweets, thought leadership, industry engagement, and online presence',
        'Stack Overflow': 'technical questions answered, reputation score, areas of expertise, and community contributions'
    }

    results = {}

    # Search each platform
    for platform_name, focus in platforms.items():
        results[platform_name] = search_platform(client, args.name, platform_name, focus)
        save_document(output_dir, platform_name, args.name, results[platform_name])

    # Create summary document
    print("\nCreating summary document...")
    create_summary(output_dir, args.name, results)

    # Print completion message
    print("\n" + "=" * 60)
    print("✓ SEARCH COMPLETE!")
    print("=" * 60)
    print(f"\nDocuments saved to: {output_dir}/")
    print("\nGenerated files:")
    for platform in platforms.keys():
        print(f"  • {platform.lower().replace('/', '_')}.md")
    print(f"  • summary.md")
    print("\n")


if __name__ == '__main__':
    main()
