use crate::models::PairwisePreference;
use chrono::Utc;
use uuid::Uuid;
use std::collections::HashSet;

/// Derive pairwise preferences from a reorder event
///
/// Algorithm:
/// - If candidate C moved from position old_pos to new_pos:
///   - If new_pos < old_pos (moved up):
///     - For each candidate D at positions [new_pos..old_pos), record C ≻ D
///   - If new_pos > old_pos (moved down):
///     - For each candidate D at positions (old_pos..new_pos], record D ≻ C
///
/// Returns a vector of PairwisePreference objects ready to be inserted.
pub fn derive_pairwise_preferences(
    before_order: &[String],
    after_order: &[String],
    job_id: &str,
    job_text: &str,
    talent_names: &std::collections::HashMap<String, String>,  // talent_id -> name
    reorder_event_id: &str,
    moved_talent_id: Option<&str>,  // If provided, only derive for this candidate
) -> Vec<PairwisePreference> {
    let mut preferences = Vec::new();

    // Handle edge cases
    if before_order.is_empty() || after_order.is_empty() {
        return preferences;
    }

    if before_order.len() != after_order.len() {
        log::warn!("Before and after order lengths don't match");
        return preferences;
    }

    // Check for no-op reorder
    if before_order == after_order {
        return preferences;
    }

    // Find which candidates changed positions
    for (new_pos, talent_id) in after_order.iter().enumerate() {
        // If moved_talent_id is specified, only process that candidate
        if let Some(moved_id) = moved_talent_id {
            if talent_id != moved_id {
                continue;
            }
        }

        // Find old position of this talent
        let old_pos = match before_order.iter().position(|id| id == talent_id) {
            Some(pos) => pos,
            None => {
                log::warn!("Talent {} not found in before_order", talent_id);
                continue;
            }
        };

        // Skip if position didn't change
        if old_pos == new_pos {
            continue;
        }

        let winner_text = talent_names.get(talent_id)
            .map(|s| s.as_str())
            .unwrap_or("Unknown");

        if new_pos < old_pos {
            // Moved up: talent is now preferred over candidates they passed
            // Look at candidates that were between new_pos and old_pos in BEFORE order
            for crossed_pos in new_pos..old_pos {
                let crossed_talent_id = &before_order[crossed_pos];
                if crossed_talent_id == talent_id {
                    continue;
                }

                let loser_text = talent_names.get(crossed_talent_id)
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown");

                preferences.push(PairwisePreference {
                    id: Uuid::new_v4().to_string(),
                    winner_id: talent_id.clone(),
                    loser_id: crossed_talent_id.clone(),
                    job_id: job_id.to_string(),
                    job_text: job_text.to_string(),
                    winner_text: winner_text.to_string(),
                    loser_text: loser_text.to_string(),
                    source: "manual_reorder".to_string(),
                    confidence: 1.0,
                    reorder_event_id: Some(reorder_event_id.to_string()),
                    created_at: Utc::now().to_rfc3339(),
                });
            }
        } else {
            // Moved down: candidates that passed this talent are now preferred
            // Look at candidates that are between old_pos and new_pos in BEFORE order
            for crossed_pos in (old_pos + 1)..=new_pos {
                let crossed_talent_id = &before_order[crossed_pos];
                if crossed_talent_id == talent_id {
                    continue;
                }

                let winner_text_crossed = talent_names.get(crossed_talent_id)
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown");

                preferences.push(PairwisePreference {
                    id: Uuid::new_v4().to_string(),
                    winner_id: crossed_talent_id.clone(),
                    loser_id: talent_id.clone(),
                    job_id: job_id.to_string(),
                    job_text: job_text.to_string(),
                    winner_text: winner_text_crossed.to_string(),
                    loser_text: winner_text.to_string(),
                    source: "manual_reorder".to_string(),
                    confidence: 1.0,
                    reorder_event_id: Some(reorder_event_id.to_string()),
                    created_at: Utc::now().to_rfc3339(),
                });
            }
        }
    }

    // Deduplicate preferences by (winner_id, loser_id) pair
    // This handles cases where swapping candidates generates the same preference twice
    let mut seen = HashSet::new();
    let mut deduped_preferences = Vec::new();

    for pref in preferences {
        let key = (pref.winner_id.clone(), pref.loser_id.clone());
        if seen.insert(key) {
            deduped_preferences.push(pref);
        }
    }

    deduped_preferences
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_talent_names() -> HashMap<String, String> {
        let mut names = HashMap::new();
        names.insert("t1".to_string(), "Alice".to_string());
        names.insert("t2".to_string(), "Bob".to_string());
        names.insert("t3".to_string(), "Charlie".to_string());
        names.insert("t4".to_string(), "Diana".to_string());
        names.insert("t5".to_string(), "Eve".to_string());
        names
    }

    #[test]
    fn test_no_op_reorder() {
        let before = vec!["t1".to_string(), "t2".to_string(), "t3".to_string()];
        let after = before.clone();
        let names = create_talent_names();

        let prefs = derive_pairwise_preferences(
            &before,
            &after,
            "job1",
            "Software Engineer",
            &names,
            "event1",
            None,
        );

        assert_eq!(prefs.len(), 0, "No-op reorder should produce no preferences");
    }

    #[test]
    fn test_move_up_one_position() {
        // Bob (t2) moves from position 1 to position 0
        let before = vec!["t1".to_string(), "t2".to_string(), "t3".to_string()];
        let after = vec!["t2".to_string(), "t1".to_string(), "t3".to_string()];
        let names = create_talent_names();

        let prefs = derive_pairwise_preferences(
            &before,
            &after,
            "job1",
            "Software Engineer",
            &names,
            "event1",
            Some("t2"),
        );

        // Bob (t2) moved up, so t2 ≻ t1
        assert_eq!(prefs.len(), 1);
        assert_eq!(prefs[0].winner_id, "t2");
        assert_eq!(prefs[0].loser_id, "t1");
        assert_eq!(prefs[0].winner_text, "Bob");
        assert_eq!(prefs[0].loser_text, "Alice");
    }

    #[test]
    fn test_move_down_one_position() {
        // Alice (t1) moves from position 0 to position 1
        let before = vec!["t1".to_string(), "t2".to_string(), "t3".to_string()];
        let after = vec!["t2".to_string(), "t1".to_string(), "t3".to_string()];
        let names = create_talent_names();

        let prefs = derive_pairwise_preferences(
            &before,
            &after,
            "job1",
            "Software Engineer",
            &names,
            "event1",
            Some("t1"),
        );

        // Alice (t1) moved down, so t2 ≻ t1
        assert_eq!(prefs.len(), 1);
        assert_eq!(prefs[0].winner_id, "t2");
        assert_eq!(prefs[0].loser_id, "t1");
    }

    #[test]
    fn test_move_up_multiple_positions() {
        // Eve (t5) moves from position 4 to position 1
        let before = vec!["t1".to_string(), "t2".to_string(), "t3".to_string(), "t4".to_string(), "t5".to_string()];
        let after = vec!["t1".to_string(), "t5".to_string(), "t2".to_string(), "t3".to_string(), "t4".to_string()];
        let names = create_talent_names();

        let prefs = derive_pairwise_preferences(
            &before,
            &after,
            "job1",
            "Software Engineer",
            &names,
            "event1",
            Some("t5"),
        );

        // Eve (t5) moved from 4 to 1, crossing t2, t3, t4
        // So: t5 ≻ t2, t5 ≻ t3, t5 ≻ t4
        assert_eq!(prefs.len(), 3);

        // Check all preferences are for t5 as winner
        for pref in &prefs {
            assert_eq!(pref.winner_id, "t5");
            assert_eq!(pref.winner_text, "Eve");
        }

        // Check losers (order might vary)
        let loser_ids: Vec<&str> = prefs.iter().map(|p| p.loser_id.as_str()).collect();
        assert!(loser_ids.contains(&"t2"));
        assert!(loser_ids.contains(&"t3"));
        assert!(loser_ids.contains(&"t4"));
    }

    #[test]
    fn test_move_down_multiple_positions() {
        // Alice (t1) moves from position 0 to position 3
        let before = vec!["t1".to_string(), "t2".to_string(), "t3".to_string(), "t4".to_string(), "t5".to_string()];
        let after = vec!["t2".to_string(), "t3".to_string(), "t4".to_string(), "t1".to_string(), "t5".to_string()];
        let names = create_talent_names();

        let prefs = derive_pairwise_preferences(
            &before,
            &after,
            "job1",
            "Software Engineer",
            &names,
            "event1",
            Some("t1"),
        );

        // Alice (t1) moved from 0 to 3, crossed by t2, t3, t4
        // So: t2 ≻ t1, t3 ≻ t1, t4 ≻ t1
        assert_eq!(prefs.len(), 3);

        // Check all preferences are for t1 as loser
        for pref in &prefs {
            assert_eq!(pref.loser_id, "t1");
            assert_eq!(pref.loser_text, "Alice");
        }

        // Check winners
        let winner_ids: Vec<&str> = prefs.iter().map(|p| p.winner_id.as_str()).collect();
        assert!(winner_ids.contains(&"t2"));
        assert!(winner_ids.contains(&"t3"));
        assert!(winner_ids.contains(&"t4"));
    }

    #[test]
    fn test_empty_orders() {
        let before: Vec<String> = vec![];
        let after: Vec<String> = vec![];
        let names = create_talent_names();

        let prefs = derive_pairwise_preferences(
            &before,
            &after,
            "job1",
            "Software Engineer",
            &names,
            "event1",
            None,
        );

        assert_eq!(prefs.len(), 0);
    }

    #[test]
    fn test_mismatched_lengths() {
        let before = vec!["t1".to_string(), "t2".to_string()];
        let after = vec!["t1".to_string()];
        let names = create_talent_names();

        let prefs = derive_pairwise_preferences(
            &before,
            &after,
            "job1",
            "Software Engineer",
            &names,
            "event1",
            None,
        );

        assert_eq!(prefs.len(), 0);
    }
}
