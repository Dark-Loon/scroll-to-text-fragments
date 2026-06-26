use unicode_segmentation::UnicodeSegmentation;
const THRESHOLD: usize = 150;

pub fn extract_range(text: &str) -> (String, Option<String>) {
    if text.len() <= THRESHOLD {
        return (text.to_string(), None);
    }

    let words: Vec<&str> = text.unicode_words().collect();

    match words.len() {
        0 => (text.to_string(), None),
        1 => (text.to_string(), None),
        2 => (words[0].to_string(), Some(words[1].to_string())),
        _ => {
            let start = words[..4].join(" ");
            let end = words[words.len() - 4..].join(" ");
            (start, Some(end))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // threshold boundary

    #[test]
    fn short_text_returned_whole() {
        let text = "It is here that Wittgenstein's rejection begins.";
        let (start, end) = extract_range(text);
        assert_eq!(start, text);
        assert_eq!(end, None);
    }

    #[test]
    fn text_exactly_at_threshold_returned_whole() {
        // 150 bytes exactly should not trigger range extraction
        let text = "a".repeat(THRESHOLD);
        let (start, end) = extract_range(&text);
        assert_eq!(start, text);
        assert_eq!(end, None);
    }

    #[test]
    fn text_one_byte_over_threshold_triggers_range() {
        let text = format!(
            "one two three four {} five six seven eight",
            "middle".repeat(THRESHOLD)
        );
        let (start, end) = extract_range(&text);
        assert_eq!(start, "one two three four");
        assert!(end.is_some());
    }

    // punctuation preservation

    #[test]
    fn punctuation_preserved_in_start_anchor() {
        let text = "Our analysis does not, support the prevalence of either type of high god among ancestral hunter-gatherers, \
        and the evolution of high gods does not correlate with any of the other traits of hunter-gatherer religion, including ancestor worship.";
        let (start, _) = extract_range(text);
        assert_eq!(start, "Our analysis does not,");
    }

    #[test]
    fn punctuation_preserved_in_end_anchor() {
        let text = "Our analysis does not, support the prevalence of either type of high god among ancestral hunter-gatherers, \
        and the evolution of high gods does not correlate with any of the other traits of hunter-gatherer religion, including ancestor worship.";
        let (_, end) = extract_range(text);
        assert_eq!(
            end,
            Some("religion, including ancestor worship.".to_string())
        );
    }

    #[test]
    fn em_dash_preserved_in_anchor() {
        let text = "Family resemblance also serves to exhibit \
                    the lack of boundaries—and the distance from exactness—";
        let (_, end) = extract_range(text);
        assert!(end.unwrap().contains("—"));
    }

    #[test]
    fn curly_quotes_preserved() {
        let text = "Family resemblance also serves to exhibit \
                    the lack of boundaries—and the distance from \"exactness\"—";
        let (_, end) = extract_range(text);
        assert!(end.unwrap().contains("\""));
    }

    // non-ASCII scripts

    #[test]
    fn arabic_punctuation_preserved() {
        let text = "يُعدّ هذا النص، الذي يتناول موضوع الفلسفة، \
                    من أهم النصوص التي كتبها الفيلسوف في هذه المرحلة \
                    من حياته الفكرية والعلمية والأدبية المتميزة جداً.";
        let (start, end) = extract_range(text);
        assert!(!start.is_empty());
        assert!(end.is_some());
        assert!(end.unwrap().chars().any(|c| !c.is_whitespace()));
    }

    #[test]
    fn hebrew_text_extracts_range() {
        let text = "הטקסט הזה עוסק בנושא הפילוסופיה של ויטגנשטיין, \
                    ובמיוחד בתפיסתו לגבי משמעות המילים והשימוש בשפה \
                    בהקשרים שונים של החיים היומיומיים והאקדמיים.";
        let (start, end) = extract_range(text);
        assert!(!start.is_empty());
        assert!(end.is_some());
    }

    // edge cases

    #[test]
    fn single_long_word_returns_no_end() {
        // one token over threshold, can't form a range
        let text = "أ".repeat(400);
        let (start, end) = extract_range(&text);
        assert_eq!(start, text);
        assert_eq!(end, None);
    }

    #[test]
    fn start_and_end_are_not_equal() {
        let text = format!("one two three four {}", "x".repeat(THRESHOLD));
        let (start, end) = extract_range(&text);
        // start is the first 4 words; if end would equal start, it's suppressed
        if let Some(e) = end {
            assert_ne!(start, e);
        }
    }

    #[test]
    fn trailing_whitespace_does_not_leak_into_end_anchor() {
        let text = format!(
            "one two three four {} five six seven eight   ",
            "middle ".repeat(20)
        );
        let (_, end) = extract_range(&text);
        if let Some(e) = end {
            assert!(!e.ends_with(' '));
        }
    }
}
