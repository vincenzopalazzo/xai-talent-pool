# Candidate Scoring System

**Score Range**: 0-100 | **RL Normalized**: 0-1

Multi-factor evaluation algorithm for candidate assessment, optimized for verifiable rewards in reinforcement learning.

---

## Scoring Breakdown

### 1. Technical Competency (40 points)

#### 1a. Hard Skills Match (20 points)

| Criterion | Points | Verification |
|-----------|--------|--------------|
| Expert in required language | 5 per language (max 10) | Certifications, assessments, code samples |
| Proficient in secondary language | 2 per language (max 5) | Portfolio, projects |
| Critical framework expertise | 5 total | Production experience, contributions |

#### 1b. System Design & Architecture (10 points)

| Criterion | Points | Verification |
|-----------|--------|--------------|
| Scalable systems built | 2 per project (max 6) | Portfolio, case studies |
| Distributed systems experience | 4 (binary) | Resume verification, interview |

#### 1c. Code Quality (10 points)

```python
code_quality_score = (
    pr_acceptance_rate * 0.3 +
    code_review_rating * 0.3 +
    repository_impact * 0.2 +
    test_coverage * 0.2
) * 10
```

---

### 2. Experience & Track Record (25 points)

#### 2a. Professional Experience (12 points)

```python
base_score = min(years_experience / 2, 5)

company_tier_bonus = {
    "FAANG/Tier1": 4,
    "Unicorn/High-growth": 3,
    "Established tech": 2,
    "Other": 1
}

impact_multiplier = {
    "Led team of 10+": 3,
    "Led team of 5-9": 2,
    "Senior IC": 1,
    "IC": 0
}

total = base_score + company_tier_bonus + impact_multiplier  # max 12
```

#### 2b. Achievements (8 points max)

| Achievement | Points per Item | Max |
|------------|-----------------|-----|
| Patents filed | 2 | 4 |
| Research papers | 2 | 4 |
| Conference talks | 1 | 2 |
| Competition wins (Kaggle, hackathons) | 2 | 4 |

#### 2c. Specialized Expertise (5 points)

**AI/ML Engineers**:
- Kaggle: Grandmaster (5), Master (4), Expert (3), Contributor (2)
- Production model deployments: 1 pt each (max 3)
- ML publications: 2 pts per paper (max 4)

**Backend Engineers**:
- Performance improvements (50%+): 2 pts each (max 4)
- High-uptime systems (99.9%+): 2 pts
- Database optimization: 1 pt

**Frontend Engineers**:
- Lighthouse score avg >90: 3 pts
- WCAG AA compliance: 2 pts
- Performance optimization projects: 2 pts each (max 4)

**Security Engineers**:
- CVEs discovered: 2 pts each (max 4)
- Security certs (CISSP, CEH): 3 pts
- Security tool contributions: 2 pts

---

### 3. Problem-Solving & Learning (20 points)

#### 3a. Coding Assessment (12 points)

```python
score = (
    implementation_correctness * 0.4 +
    time_complexity * 0.3 +
    code_quality * 0.2 +
    edge_cases * 0.1
) * 12
```

| Component | 1.0 | 0.7 | 0.3 |
|-----------|-----|-----|-----|
| Implementation | All tests pass | 70%+ pass | <70% pass |
| Time Complexity | Optimal | Acceptable | Suboptimal |
| Code Quality | Clean, DRY | Minor issues | Poor |
| Edge Cases | All handled | Most handled | Few handled |

#### 3b. Adaptability (8 points)

| Criterion | Points |
|-----------|--------|
| Technology breadth (domains mastered) | 1 per domain (max 4) |
| Skills learned in last 2 years | 2 per skill (max 4) |

**Domains**: Frontend, Backend, DevOps, Mobile, Data/ML, Cloud, Security, Embedded

---

### 4. Communication & Collaboration (10 points)

#### 4a. Technical Communication (6 points)

| Criterion | Points | Scoring |
|-----------|--------|---------|
| Documentation quality | 0-3 | 3=Comprehensive, 2=Good, 1=Basic, 0=Minimal |
| Code review participation | 0-3 | Activity + quality + timeliness |

```python
review_score = (
    min(reviews_given / 50, 1) * 0.4 +
    avg_comment_quality * 0.4 +
    response_time_score * 0.2
) * 3
```

#### 4b. Team Dynamics (4 points)

| Criterion | Points |
|-----------|--------|
| Cross-functional projects | 0-2 |
| Mentorship/teaching | 0-2 |

---

### 5. Cultural Fit & Soft Skills (5 points)

| Criterion | Points |
|-----------|--------|
| Leadership examples | 0-2 |
| Product thinking (asks about users, metrics) | 0-2 |
| Values alignment | 0-1 |

---

## Score Calculation

### Standard Score

```python
def calculate_candidate_score(candidate):
    technical = (
        hard_skills_match(candidate) +      # 20
        system_design_score(candidate) +     # 10
        code_quality_score(candidate)        # 10
    )  # 40 points

    experience = (
        professional_experience(candidate) +  # 12
        achievements_score(candidate) +       # 8
        specialized_expertise(candidate)      # 5
    )  # 25 points

    problem_solving = (
        coding_assessment(candidate) +        # 12
        adaptability_score(candidate)         # 8
    )  # 20 points

    communication = (
        technical_communication(candidate) +  # 6
        team_dynamics(candidate)              # 4
    )  # 10 points

    soft_skills = cultural_fit(candidate)  # 5 points

    return technical + experience + problem_solving + communication + soft_skills
```

### RL Reward Function

```python
def rl_reward(candidate, role_requirements):
    # Binary gate: must meet minimum requirements
    if not meets_minimum_requirements(candidate, role_requirements):
        return 0.0

    base_score = calculate_candidate_score(candidate)
    weighted_score = apply_role_weights(base_score, role_requirements)

    # Success amplification for strong candidates
    if weighted_score >= 75:
        weighted_score = min(weighted_score * 1.1, 100)

    return weighted_score / 100.0
```

---

## Role-Specific Weights

### Senior Engineer
```python
weights = {
    'technical': 0.45,
    'experience': 0.30,
    'problem_solving': 0.15,
    'communication': 0.07,
    'soft_skills': 0.03
}
```

### Tech Lead / Manager
```python
weights = {
    'technical': 0.35,
    'experience': 0.25,
    'problem_solving': 0.15,
    'communication': 0.15,
    'soft_skills': 0.10
}
```

### Junior Engineer
```python
weights = {
    'technical': 0.40,
    'experience': 0.10,
    'problem_solving': 0.35,
    'communication': 0.10,
    'soft_skills': 0.05
}
```

### Research Scientist / ML
```python
weights = {
    'technical': 0.35,
    'experience': 0.30,
    'problem_solving': 0.25,
    'communication': 0.08,
    'soft_skills': 0.02
}
```

---

## Hiring Thresholds

| Score | Decision | Success Rate |
|-------|----------|--------------|
| 75-100 | **Strong Hire** | 92%+ |
| 60-74 | **Hire** | 75-91% |
| 50-59 | **Maybe** | 50-74% |
| 40-49 | **Weak No** | <50% |
| 0-39 | **Strong No** | <25% |

---

## Detailed Examples

### Example 1: Senior Backend Engineer

**Profile**:
- 7 years at Stripe, led team of 6
- Expert: Go, Python | Proficient: Rust
- 3 scalable systems (100K+ users), distributed experience
- 2 patents, code metrics: 85% PR acceptance, 90% test coverage
- Coding assessment: 10.5/12
- Strong communicator, mentorship experience

**Credit Assignment**:

```
Technical (40):
├── Hard skills: 17/20
│   ├── Go (expert): 5
│   ├── Python (expert): 5
│   ├── Rust (proficient): 2
│   └── Framework expertise: 5
├── System design: 10/10
│   ├── 3 scalable systems: 6
│   └── Distributed systems: 4
└── Code quality: 8.5/10
    └── (0.85*0.3 + 0.9*0.3 + 0.7*0.2 + 0.9*0.2) * 10 = 8.5

Experience (25):
├── Professional: 8.5/12
│   ├── Years (7/2): 3.5
│   ├── Stripe (unicorn): 3
│   └── Led team of 6: 2
├── Achievements: 4/8
│   └── 2 patents: 4
└── Specialized (backend): 3/5

Problem-Solving (20):
├── Coding assessment: 10.5/12
└── Adaptability: 5/8
    ├── Breadth (3 domains): 3
    └── Recent learning: 2

Communication (10):
├── Technical comm: 4.5/6
│   ├── Documentation: 2
│   └── Code reviews: 2.5
└── Team dynamics: 3/4
    ├── Cross-functional: 1
    └── Mentorship: 2

Soft Skills (5):
└── Cultural fit: 4/5

TOTAL: 78/100 → Strong Hire
RL Reward: 0.78 * 1.1 = 0.858
```

### Example 2: Junior Frontend Engineer

**Profile**:
- 1.5 years at startup
- Expert: React, TypeScript
- 1 high-performance app (Lighthouse 95)
- 50+ OSS PRs, coding assessment: 9/12

**Credit Assignment**:

```
Technical (40):
├── Hard skills: 15/20
│   ├── React (expert): 5
│   ├── TypeScript (expert): 5
│   └── Framework: 5
├── System design: 2/10
│   └── 1 project: 2
└── Code quality: 7/10

Experience (25):
├── Professional: 1.75/12
│   ├── Years (1.5/2): 0.75
│   └── Startup: 1
├── Achievements: 0/8
└── Specialized (frontend): 5/5
    └── Lighthouse 95+: 3 (+ other frontend metrics)

Problem-Solving (20):
├── Coding assessment: 9/12
└── Adaptability: 6/8
    ├── Breadth: 2
    └── Recent learning: 4

Communication (10):
├── Technical comm: 4/6
│   ├── Documentation: 2
│   └── Code reviews: 2
└── Team dynamics: 0/4

Soft Skills (5):
└── Cultural fit: 3/5

Standard Total: 52.75/100 → Maybe

With Junior Weights:
24*0.40 + 6.75*0.10 + 15*0.35 + 4*0.10 + 3*0.05 = 80.4/100 → Strong Hire

RL Reward: 0.804 * 1.1 = 0.884
```

---

## Verifiability Levels

| Component | Verifiability | Source |
|-----------|---------------|--------|
| Code test results | **High** | Automated tests |
| Certifications | **High** | Issuing authority |
| GitHub metrics | **High** | GitHub API |
| Years of experience | **Medium** | Resume + background check |
| System design | **Medium** | Interview + verification |
| Code review quality | **Medium** | Peer ratings + metrics |
| Communication | **Low** | Subjective assessment |
| Cultural fit | **Low** | Subjective assessment |

**For RL**: Weight high-verifiability components more heavily to reduce reward noise.
