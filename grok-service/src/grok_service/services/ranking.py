"""
GRPO (Group Relative Policy Optimization) Ranking Service

This service implements a GRPO-based ranking algorithm for candidate-job matching,
incorporating RLHF (Reinforcement Learning from Human Feedback) signals to continuously
improve ranking quality.
"""

import logging
from typing import List, Dict, Optional
import numpy as np
from xai_sdk import Client

logger = logging.getLogger(__name__)


class GRPORankingService:
    """
    GRPO-based candidate ranking service that learns from human feedback.

    The algorithm works by:
    1. Computing initial match scores based on skills, experience, and job requirements
    2. Adjusting scores using RLHF feedback (upvotes/downvotes)
    3. Using group-relative comparisons to rank candidates
    4. Continuously updating weights based on feedback signals
    """

    def __init__(self, xai_client: Client, model_version: str = "v1.0"):
        self.xai_client = xai_client
        self.model_version = model_version

        # GRPO hyperparameters
        self.learning_rate = 0.1
        self.feedback_weight = 0.3  # How much to weight RLHF feedback vs. initial score

        # Feature weights (learned from feedback over time)
        self.weights = {
            "skills_match": 0.35,
            "experience_match": 0.25,
            "location_match": 0.15,
            "title_match": 0.15,
            "feedback_score": 0.10,
        }

    def rank_candidates(
        self,
        job: Dict,
        candidates: List[Dict],
        feedback_data: Optional[List[Dict]] = None,
        use_feedback: bool = True
    ) -> List[Dict]:
        """
        Rank candidates for a job using GRPO algorithm.

        Args:
            job: Job posting data
            candidates: List of candidate/talent data
            feedback_data: RLHF feedback data (upvotes/downvotes)
            use_feedback: Whether to incorporate feedback into ranking

        Returns:
            List of candidates with ranking scores and positions
        """
        if not candidates:
            return []

        logger.info(f"Ranking {len(candidates)} candidates for job {job.get('id')}")

        # Step 1: Compute base match scores
        scored_candidates = []
        for candidate in candidates:
            match_factors = self._compute_match_factors(job, candidate)
            base_score = self._compute_base_score(match_factors)

            scored_candidates.append({
                "candidate": candidate,
                "match_factors": match_factors,
                "base_score": base_score,
            })

        # Step 2: Apply RLHF feedback adjustment
        if use_feedback and feedback_data:
            scored_candidates = self._apply_feedback_adjustment(
                scored_candidates, feedback_data, job.get("id")
            )

        # Step 3: Apply GRPO group-relative ranking
        ranked_candidates = self._apply_grpo_ranking(scored_candidates)

        # Step 4: Assign positions
        for i, item in enumerate(ranked_candidates):
            item["rank_position"] = i + 1
            item["confidence"] = self._compute_confidence(item)

        logger.info(f"Completed ranking for job {job.get('id')}")

        return ranked_candidates

    def _compute_match_factors(self, job: Dict, candidate: Dict) -> Dict[str, float]:
        """Compute individual match factors between job and candidate."""

        # Skills match
        job_skills = set(skill.strip().lower() for skill in job.get("skills_required", "").split(","))
        candidate_skills = set(skill.strip().lower() for skill in candidate.get("skills", "").split(","))
        skills_match = len(job_skills & candidate_skills) / max(len(job_skills), 1) if job_skills else 0.0

        # Experience level match
        experience_levels = {"entry": 1, "mid": 2, "senior": 3, "lead": 4}
        job_level = experience_levels.get(job.get("experience_level", "").lower(), 2)
        candidate_exp = candidate.get("experience", "").lower()

        # Parse years of experience from candidate
        candidate_level = 1
        if "senior" in candidate_exp or "lead" in candidate_exp:
            candidate_level = 3
        elif "mid" in candidate_exp or any(str(i) in candidate_exp for i in range(3, 8)):
            candidate_level = 2

        experience_match = 1.0 - min(abs(job_level - candidate_level) / 4, 1.0)

        # Location match
        location_match = 1.0  # Default to full match
        if job.get("location_type") == "onsite" and job.get("location"):
            job_loc = job.get("location", "").lower()
            candidate_loc = candidate.get("location", "").lower()
            location_match = 0.8 if candidate_loc and job_loc in candidate_loc else 0.3
        elif job.get("location_type") == "hybrid":
            location_match = 0.9

        # Title relevance match
        job_title = job.get("title", "").lower()
        candidate_title = candidate.get("title", "").lower()
        title_words = set(job_title.split())
        candidate_words = set(candidate_title.split())
        title_match = len(title_words & candidate_words) / max(len(title_words), 1) if title_words else 0.0

        return {
            "skills_match": min(skills_match, 1.0),
            "experience_match": min(experience_match, 1.0),
            "location_match": min(location_match, 1.0),
            "title_match": min(title_match, 1.0),
            "overall_fit": 0.0,  # Will be computed later
        }

    def _compute_base_score(self, match_factors: Dict[str, float]) -> float:
        """Compute weighted base score from match factors."""
        score = (
            match_factors["skills_match"] * self.weights["skills_match"] +
            match_factors["experience_match"] * self.weights["experience_match"] +
            match_factors["location_match"] * self.weights["location_match"] +
            match_factors["title_match"] * self.weights["title_match"]
        )

        # Normalize to 0-1 range
        base_weight_sum = (
            self.weights["skills_match"] +
            self.weights["experience_match"] +
            self.weights["location_match"] +
            self.weights["title_match"]
        )

        return score / base_weight_sum if base_weight_sum > 0 else 0.0

    def _apply_feedback_adjustment(
        self,
        scored_candidates: List[Dict],
        feedback_data: List[Dict],
        job_id: str
    ) -> List[Dict]:
        """Apply RLHF feedback to adjust candidate scores."""

        # Build feedback lookup
        feedback_lookup = {}
        for feedback in feedback_data:
            if feedback.get("job_id") == job_id:
                talent_id = feedback.get("talent_id")
                if talent_id not in feedback_lookup:
                    feedback_lookup[talent_id] = {"upvotes": 0, "downvotes": 0}

                if feedback.get("feedback_type") == "upvote":
                    feedback_lookup[talent_id]["upvotes"] += 1
                else:
                    feedback_lookup[talent_id]["downvotes"] += 1

        # Adjust scores based on feedback
        for item in scored_candidates:
            candidate_id = item["candidate"].get("id")
            feedback = feedback_lookup.get(candidate_id, {"upvotes": 0, "downvotes": 0})

            upvotes = feedback["upvotes"]
            downvotes = feedback["downvotes"]
            total_feedback = upvotes + downvotes

            if total_feedback > 0:
                # Compute feedback score: +1 for upvote, -1 for downvote
                feedback_score = (upvotes - downvotes) / total_feedback

                # Apply sigmoid to bound between -1 and 1
                feedback_adjustment = np.tanh(feedback_score)

                # Adjust base score
                item["feedback_score"] = feedback_adjustment
                item["adjusted_score"] = (
                    item["base_score"] * (1 - self.feedback_weight) +
                    (feedback_adjustment + 1) / 2 * self.feedback_weight  # Normalize to 0-1
                )

                logger.debug(
                    f"Applied feedback adjustment for candidate {candidate_id}: "
                    f"{item['base_score']:.3f} -> {item['adjusted_score']:.3f} "
                    f"(upvotes: {upvotes}, downvotes: {downvotes})"
                )
            else:
                item["feedback_score"] = 0.0
                item["adjusted_score"] = item["base_score"]

        return scored_candidates

    def _apply_grpo_ranking(self, scored_candidates: List[Dict]) -> List[Dict]:
        """
        Apply Group Relative Policy Optimization ranking.

        GRPO compares candidates relative to each other in the group,
        rather than using absolute scores alone.
        """
        if not scored_candidates:
            return []

        # Use adjusted scores if available, otherwise base scores
        scores = np.array([
            item.get("adjusted_score", item["base_score"])
            for item in scored_candidates
        ])

        # Apply softmax to get relative probabilities
        # This is the core of GRPO - ranking based on relative preference
        exp_scores = np.exp(scores - np.max(scores))  # Numerical stability
        probabilities = exp_scores / np.sum(exp_scores)

        # Assign final rank scores
        for i, item in enumerate(scored_candidates):
            item["rank_score"] = float(probabilities[i])
            item["match_factors"]["overall_fit"] = float(scores[i])

        # Sort by rank score (descending)
        ranked = sorted(scored_candidates, key=lambda x: x["rank_score"], reverse=True)

        return ranked

    def _compute_confidence(self, ranked_item: Dict) -> float:
        """Compute confidence score for a ranking."""
        base_score = ranked_item.get("base_score", 0.0)
        rank_score = ranked_item.get("rank_score", 0.0)

        # Higher confidence if base match is strong and rank is consistent
        confidence = (base_score + rank_score) / 2

        # Reduce confidence if there's no feedback data yet
        if ranked_item.get("feedback_score", 0.0) == 0.0:
            confidence *= 0.8

        return min(confidence, 1.0)

    def update_weights_from_feedback(self, feedback_batch: List[Dict]):
        """
        Update model weights based on a batch of feedback.

        This implements the learning component of GRPO - adjusting weights
        based on which candidates received positive vs. negative feedback.
        """
        if not feedback_batch:
            return

        logger.info(f"Updating GRPO weights from {len(feedback_batch)} feedback samples")

        # Aggregate feedback signals
        positive_feedback = [f for f in feedback_batch if f.get("feedback_type") == "upvote"]
        negative_feedback = [f for f in feedback_batch if f.get("feedback_type") == "downvote"]

        # Adjust feedback weight based on feedback volume
        total_feedback = len(feedback_batch)
        positive_ratio = len(positive_feedback) / total_feedback if total_feedback > 0 else 0.5

        # Update feedback weight using gradient descent
        gradient = (positive_ratio - 0.5) * self.learning_rate
        self.weights["feedback_score"] = np.clip(
            self.weights["feedback_score"] + gradient,
            0.05,  # Min weight
            0.30   # Max weight
        )

        # Renormalize weights
        total_weight = sum(self.weights.values())
        for key in self.weights:
            self.weights[key] = self.weights[key] / total_weight

        logger.info(f"Updated weights: {self.weights}")
