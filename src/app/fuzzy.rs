use ratatui::text::Line;

pub fn fuzzy_search(items: &[Line<'static>], query: &str) -> Vec<(usize, Line<'static>)> {
    let query_lower = query.to_lowercase();
    let mut results: Vec<(usize, Line<'static>, usize)> = Vec::new();

    for (index, item) in items.iter().enumerate() {
        let item_text = item
            .spans
            .iter()
            .map(|span| span.content.clone())
            .collect::<String>()
            .to_lowercase();

        if let Some(score) = fuzzy_match(&item_text, &query_lower) {
            results.push((index, item.clone(), score));
        }
    }

    // Sort by score (higher is better)
    results.sort_by(|a, b| b.2.cmp(&a.2));

    // Return only index and item
    results
        .into_iter()
        .map(|(index, item, _)| (index, item))
        .collect()
}

fn fuzzy_match(text: &str, pattern: &str) -> Option<usize> {
    if pattern.is_empty() {
        return Some(0);
    }

    let text_chars: Vec<char> = text.chars().collect();
    let pattern_chars: Vec<char> = pattern.chars().collect();

    let mut score = 0;
    let mut pattern_idx = 0;
    let mut last_match_idx = 0;

    for (text_idx, &text_char) in text_chars.iter().enumerate() {
        if pattern_idx < pattern_chars.len() && text_char == pattern_chars[pattern_idx] {
            // Consecutive matches get bonus points
            if text_idx == last_match_idx + 1 {
                score += 10;
            } else {
                score += 5;
            }

            // Beginning of word matches get bonus points
            if text_idx == 0
                || text_chars[text_idx - 1] == ' '
                || text_chars[text_idx - 1] == '-'
                || text_chars[text_idx - 1] == '_'
            {
                score += 15;
            }

            last_match_idx = text_idx;
            pattern_idx += 1;
        }
    }

    if pattern_idx == pattern_chars.len() {
        // Shorter strings with matches score higher
        score += 1000 / (text.len() + 1);
        Some(score)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::text::Line;

    #[test]
    fn test_fuzzy_match() {
        assert!(fuzzy_match("production-server-01", "prod").is_some());
        assert!(fuzzy_match("development-api", "dev").is_some());
        assert!(fuzzy_match("web-server", "web").is_some());
        assert!(fuzzy_match("database", "xyz").is_none());
    }

    #[test]
    fn test_fuzzy_search() {
        let items = vec![
            Line::from("production-server-01"),
            Line::from("development-api"),
            Line::from("web-server"),
            Line::from("database-prod"),
        ];

        let results = fuzzy_search(&items, "prod");
        assert!(!results.is_empty());

        // Should match both "production-server-01" and "database-prod"
        assert!(results.len() >= 2);
    }
}
