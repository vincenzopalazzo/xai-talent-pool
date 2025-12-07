# Grok Service

Python microservice for Grok AI integration in X Talent Pool.

## Features

- Resume analysis using Grok AI
- Extract work experiences from PDF resumes
- Extract profile URLs (LinkedIn, X, GitHub, GitLab)

## Setup

```bash
# Install dependencies
pip install -e ".[dev]"

# Copy environment file
cp .env.example .env
# Edit .env and add your XAI_API_KEY

# Run development server
uvicorn grok_service.main:app --reload --port 8001
```

## API Endpoints

- `GET /health` - Health check
- `POST /api/v1/screening/initial` - Initial screening with resume analysis

## Environment Variables

- `XAI_API_KEY` - xAI API key for Grok
- `GROK_HOST` - Server host (default: 0.0.0.0)
- `GROK_PORT` - Server port (default: 8001)
