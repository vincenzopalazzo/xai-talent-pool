# RLHF + GRPO Candidate Ranking System

This document explains the implementation of the **RLHF (Reinforcement Learning from Human Feedback)** system integrated with **GRPO (Group Relative Policy Optimization)** for intelligent candidate-job matching in the xAI Talent Pool.

## 🎯 Overview

The system provides AI-powered candidate ranking that continuously improves based on recruiter feedback. Recruiters can upvote candidates who perform better than expected and downvote those who don't meet expectations, allowing the ranking algorithm to learn and adapt over time.

### Key Features

- **GRPO-based Ranking**: Uses group-relative comparisons to rank candidates for specific jobs
- **RLHF Integration**: Incorporates human feedback (upvotes/downvotes) to improve rankings
- **Real-time Learning**: Model weights update based on feedback batches
- **Multi-factor Matching**: Considers skills, experience, location, and title relevance
- **Confidence Scoring**: Provides confidence metrics for each ranking
- **Feedback Analytics**: Track upvotes, downvotes, and net scores per candidate

## 🏗️ Architecture

```
┌─────────────────┐
│   Frontend UI   │  (Svelte)
│  - Talent Cards │
│  - Upvote/      │
│    Downvote     │
│  - Rankings     │
└────────┬────────┘
         │
         │ API Calls
         │
┌────────▼────────┐
│  Backend API    │  (Rust/Actix-web)
│  - Feedback     │  ← Stores feedback data
│    Storage      │  ← (External API assumed)
└────────┬────────┘
         │
         │ Forward to
         │
┌────────▼────────┐
│  Grok Service   │  (Python/FastAPI)
│  - GRPO         │
│    Algorithm    │
│  - Ranking      │
│    Engine       │
│  - Weight       │
│    Updates      │
└─────────────────┘
```

## 📊 How GRPO Works

**Group Relative Policy Optimization (GRPO)** ranks candidates by comparing them relative to each other within a group, rather than using absolute scores alone.

### Algorithm Steps

1. **Compute Base Scores**
   - Skills Match (35% weight): Overlap between job requirements and candidate skills
   - Experience Match (25% weight): Alignment of experience level
   - Location Match (15% weight): Geographic compatibility
   - Title Match (15% weight): Job title relevance
   - Feedback Score (10% weight): RLHF feedback adjustment

2. **Apply RLHF Feedback**
   - Upvotes increase candidate score
   - Downvotes decrease candidate score
   - Adjustment formula: `feedback_score = tanh((upvotes - downvotes) / total_feedback)`
   - Blended with base score using configurable weight

3. **GRPO Ranking**
   - Apply softmax to get relative probabilities
   - Rank candidates by probability distribution
   - This ensures relative comparison rather than absolute scoring

4. **Confidence Calculation**
   - Based on base score strength and ranking consistency
   - Reduced if no feedback data available yet

### Example

```python
# Candidate A: 80% skills match, 2 upvotes, 0 downvotes
# Candidate B: 70% skills match, 0 upvotes, 1 downvote
# Candidate C: 75% skills match, no feedback

# GRPO will rank them relatively:
# 1. Candidate A (high match + positive feedback)
# 2. Candidate C (good match, neutral)
# 3. Candidate B (lower match + negative feedback)
```

## 🔧 Implementation Details

### Backend Models (Rust)

Located in: `server/src/models.rs`

```rust
pub struct Feedback {
    pub id: String,
    pub talent_id: String,
    pub job_id: String,
    pub recruiter_id: Option<String>,
    pub feedback_type: String,  // "upvote" or "downvote"
    pub expected_rank: Option<f64>,
    pub actual_performance: String,
    pub notes: Option<String>,
    pub created_at: String,
}

pub struct CandidateRanking {
    pub id: String,
    pub talent_id: String,
    pub job_id: String,
    pub rank_score: f64,  // 0.0 to 1.0
    pub rank_position: i32,
    pub confidence: Option<f64>,
    pub match_factors: String,  // JSON
    pub model_version: String,
    pub created_at: String,
}
```

### GRPO Service (Python)

Located in: `grok-service/src/grok_service/services/ranking.py`

Key components:
- `GRPORankingService`: Main ranking engine
- `rank_candidates()`: Performs GRPO ranking
- `update_weights_from_feedback()`: RLHF learning

Configuration:
```python
self.weights = {
    "skills_match": 0.35,
    "experience_match": 0.25,
    "location_match": 0.15,
    "title_match": 0.15,
    "feedback_score": 0.10,
}
```

### API Endpoints

#### Ranking Endpoints (Grok Service - Port 8001)

**POST** `/api/v1/ranking/rank`
- Rank candidates for a job
- Request body:
  ```json
  {
    "job": { /* job object */ },
    "candidates": [ /* array of candidates */ ],
    "feedback_data": [ /* optional feedback array */ ],
    "use_feedback": true
  }
  ```
- Returns: Array of `RankedCandidate` objects

**POST** `/api/v1/ranking/update-weights`
- Update model weights from feedback
- Request body:
  ```json
  {
    "feedback_batch": [ /* array of feedback */ ]
  }
  ```
- Returns: Updated weights and model version

**GET** `/api/v1/ranking/stats`
- Get current model statistics
- Returns: Model version and current weights

#### Feedback Endpoints (Backend API - Port 8080)

**POST** `/api/v1/feedback`
- Submit recruiter feedback
- Request body:
  ```json
  {
    "talent_id": "uuid",
    "job_id": "uuid",
    "feedback_type": "upvote" | "downvote",
    "expected_rank": 0.85,
    "recruiter_id": "uuid",
    "notes": "Optional notes"
  }
  ```

**GET** `/api/v1/feedback/talent/{talent_id}/stats?job_id={job_id}`
- Get feedback statistics for a candidate
- Returns: Upvotes, downvotes, net score

**GET** `/api/v1/feedback/job/{job_id}`
- Get all feedback for a job (for training)
- Returns: Array of feedback objects

### Frontend Components

#### CandidateFeedback Component
Location: `ui/src/lib/components/candidate-feedback.svelte`

Features:
- Upvote/downvote buttons with tooltips
- Visual feedback (green/red highlighting)
- Real-time stats display
- Loading states

Usage:
```svelte
<CandidateFeedback
  talentId={talent.id}
  jobId={jobId}
  expectedRank={rankPosition}
  showStats={true}
/>
```

#### TalentCard Component (Updated)
Location: `ui/src/lib/components/talent-card.svelte`

New props:
- `jobId`: Optional job ID for feedback context
- `rankPosition`: Candidate's rank position
- `showFeedback`: Toggle feedback controls

#### Ranking Service
Location: `ui/src/lib/services/ranking.ts`

Functions:
- `rankCandidatesForJob()`: Get ranked candidates
- `submitFeedback()`: Submit upvote/downvote
- `getFeedbackStats()`: Get feedback statistics
- `updateRankingWeights()`: Trigger model retraining

## 📱 User Interface

### Candidate List View with Feedback

```
┌─────────────────────────────────────────┐
│  [#1] John Doe                          │
│  @johndoe                         ⭐ 💬 │
│  Senior Software Engineer               │
│  San Francisco, CA                      │
│  Python, React, Node.js                 │
│  ──────────────────────────────────     │
│  Rank #1           Feedback: 👍 5 👎 0  │
│  [Show More]  [Contact]                 │
└─────────────────────────────────────────┘
```

### Feedback Buttons

- **Thumbs Up (👍)**: Candidate performed better than expected
- **Thumbs Down (👎)**: Candidate ranked higher but performed worse

When clicked, buttons:
- Change color (green for upvote, red for downvote)
- Submit feedback to API
- Update statistics in real-time
- Can be toggled to remove feedback

## 🔄 Workflow Example

### 1. Initial Ranking

```
Recruiter opens job #123 candidates page
→ Frontend fetches job and all candidates
→ Calls GRPO ranking API with job requirements
→ Displays ranked list (no feedback yet)
```

### 2. Recruiter Provides Feedback

```
Recruiter reviews candidate #5 (ranked #5)
→ Candidate exceeds expectations in interview
→ Recruiter clicks thumbs up 👍
→ Feedback submitted to API
→ Stats update: +1 upvote
```

### 3. Re-ranking with Feedback

```
After collecting feedback on 10+ candidates
→ Recruiter clicks "Re-rank" button
→ GRPO runs with feedback data
→ Candidate #5 now ranked #2 (moved up!)
→ Updated rankings displayed
```

### 4. Model Retraining

```
After 50+ feedback samples collected
→ Recruiter clicks "Retrain Model"
→ GRPO updates feature weights
→ Future rankings incorporate learned patterns
→ Model version increments
```

## 🎓 Learning Process

The system learns in two ways:

### 1. Real-time Feedback Adjustment
- Each upvote/downvote immediately affects candidate scores
- Used in ranking calculations
- No model changes

### 2. Batch Weight Updates
- Collected feedback analyzed in batches
- Feature weights adjusted via gradient descent
- Model version updated
- Affects all future rankings

Example:
```
Initial weights:
- skills_match: 0.35
- experience_match: 0.25
- feedback_score: 0.10

After 100 feedback samples showing location matters more:
- skills_match: 0.30
- experience_match: 0.20
- location_match: 0.20  (↑ increased)
- feedback_score: 0.15  (↑ increased)
```

## 🚀 Getting Started

### 1. Start Services

```bash
# Terminal 1: Backend (assumes external feedback API)
make server-dev

# Terminal 2: Grok Service (GRPO ranking)
make grok-dev

# Terminal 3: Frontend
make dev
```

### 2. Configure Environment

Create `.env` in `grok-service/`:
```env
GROK_XAI_API_KEY=your_xai_api_key
GROK_HOST=0.0.0.0
GROK_PORT=8001
```

Create `.env` in `ui/`:
```env
VITE_API_URL=http://localhost:8080
VITE_GROK_SERVICE_URL=http://localhost:8001
```

### 3. Usage Example

See: `ui/src/routes/(app)/jobs/[id]/candidates/+page.svelte`

This example page demonstrates:
- Loading and ranking candidates
- Displaying feedback controls
- Re-ranking on demand
- Model retraining

## 📈 Metrics & Analytics

### Per-Candidate Metrics
- Total upvotes
- Total downvotes
- Net score (upvotes - downvotes)
- Feedback ratio

### Model Metrics
- Current feature weights
- Model version
- Average confidence score
- Total rankings performed

### Job Metrics
- Candidates ranked
- Feedback collected
- Top-ranked candidates
- Match quality distribution

## 🔒 Data Assumptions

This implementation assumes:

1. **External Feedback API**: Feedback data is stored and retrieved via external APIs (not in local database)

2. **External Talent API**: Candidate/talent data comes from external source

3. **External Job API**: Job postings fetched from external API

To integrate with actual data sources:
- Replace API calls in `ui/src/lib/services/ranking.ts`
- Update endpoint URLs to point to your APIs
- Ensure response formats match TypeScript interfaces

## 🛠️ Customization

### Adjust Feature Weights

Edit `grok-service/src/grok_service/services/ranking.py`:

```python
self.weights = {
    "skills_match": 0.40,  # Increase skills importance
    "experience_match": 0.30,
    "location_match": 0.10,
    "title_match": 0.10,
    "feedback_score": 0.10,
}
```

### Adjust Feedback Weight

```python
self.feedback_weight = 0.3  # 30% feedback, 70% base score
```

### Add New Match Factors

1. Add factor to `_compute_match_factors()`
2. Add weight to `self.weights`
3. Update `_compute_base_score()`
4. Update TypeScript interface in `ui/src/lib/services/ranking.ts`

## 🐛 Troubleshooting

### Rankings don't change after feedback
- Check that `use_feedback=true` in ranking request
- Verify feedback is being stored correctly
- Check console for API errors

### Model retraining fails
- Ensure sufficient feedback samples (>10 recommended)
- Check Grok service logs
- Verify xAI API key is valid

### Frontend not loading
- Check API URLs in `.env`
- Verify services are running
- Check browser console for errors

## 📚 Additional Resources

- [GRPO Paper](https://arxiv.org/abs/2402.03300) (Group Relative Policy Optimization)
- [RLHF Overview](https://huggingface.co/blog/rlhf) (Reinforcement Learning from Human Feedback)
- [shadcn-svelte](https://www.shadcn-svelte.com) (UI component library)
- [FastAPI](https://fastapi.tiangolo.com) (Python API framework)

## 🤝 Contributing

To extend this system:

1. **Add new ranking factors**: Modify `GRPORankingService._compute_match_factors()`
2. **Improve UI**: Enhance `candidate-feedback.svelte` with animations, charts
3. **Add analytics**: Create dashboard for feedback insights
4. **Optimize algorithm**: Experiment with different weight update strategies

## 📝 License

Part of the xAI Talent Pool project.

---

**Questions?** Check the code comments or open an issue in the repository.
