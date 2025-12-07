# Recruiter Search Tool

A powerful tool for recruiters to search and compile information about candidates across multiple social media platforms using AI.


## Trained model
Below consists of a training run done on the RLHF data collected from using the reranking option in multiple job postings.
https://colab.research.google.com/drive/1LZQ5eefUg6OnHTPD-tsMarkrnuKYkwaB?usp=sharing

## Features

- 🔍 Searches GitHub, LinkedIn, Twitter/X, and Stack Overflow
- 🤖 Uses XAI's Grok model with high reasoning effort
- 📄 Generates separate markdown documents for each platform
- 📊 Creates a summary document with links to all reports
- 💼 Focused on information relevant for job recruiting

## Prerequisites

- Python 3.7 or higher
- XAI API key (get one from [xAI](https://x.ai))

## Installation

1. Clone or download this repository

2. Install dependencies:
```bash
pip3 install --user -r requirements.txt
```

3. Set your XAI API key as an environment variable:
```bash
export XAI_API_KEY='your-api-key-here'
```

## Usage

Basic usage:
```bash
python3 recruiter_search.py "John Doe"
```

Specify custom output directory:
```bash
python3 recruiter_search.py "Jane Smith" --output-dir my_searches
```

## Output

The tool creates a folder with the candidate's name containing:

- `github.md` - GitHub profile and projects analysis
- `linkedin.md` - LinkedIn professional background
- `twitter_x.md` - Twitter/X presence and engagement
- `stack_overflow.md` - Technical expertise and contributions
- `summary.md` - Quick overview with links to all reports

### Example Output Structure

```
search_results/
└── John_Doe/
    ├── github.md
    ├── linkedin.md
    ├── twitter_x.md
    ├── stack_overflow.md
    └── summary.md
```

## How It Works

1. Takes a person's name as input
2. Makes an AI-powered search query for each social media platform
3. Focuses on recruiting-relevant information:
   - Professional experience
   - Technical skills
   - Projects and contributions
   - Community engagement
   - Expertise areas
4. Generates detailed markdown reports for each platform
5. Creates a summary document linking all reports

## Notes

- The tool uses AI analysis, so results depend on publicly available information
- Each platform search is a separate API call to ensure detailed, focused results
- All documents are saved in markdown format for easy reading and sharing
- Token usage statistics are included in each report

## Example

```bash
# Search for a candidate
python3 recruiter_search.py "Linus Torvalds"

=