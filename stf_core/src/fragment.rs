use crate::encode::encode_special_characters;

/// Represent the text fragment directive
/// See: https://wicg.github.io/scroll-to-text-fragment/
#[derive(Debug)]
pub struct TextFragment {
    pub start: String,
    pub end: Option<String>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}

// TextFragment::from_text(text: &str, prefix: Option<String>, suffix: Option<String>) -> TextFragment
//   - char count or grapheme count for the 300 threshold?
//   - 300 words for range mode?
//   - unicode-segmentation for word boundaries (handles RTL correctly?)
impl TextFragment {
    pub fn new(
        start: String,
        end: Option<String>,
        prefix: Option<String>,
        suffix: Option<String>,
    ) -> TextFragment {
        TextFragment {
            start: normalize_whitespace(&start),
            end: end.as_deref().map(normalize_whitespace),
            prefix: prefix.as_deref().map(normalize_whitespace),
            suffix: suffix.as_deref().map(normalize_whitespace),
        }
    }

    pub fn to_directive(&self) -> String {
        let mut string = String::from("#:~:text=");
        let comma = ",";
        let dash = "-";

        if let Some(value) = &self.prefix {
            let encoded_value = encode_special_characters(value);

            string.push_str(&encoded_value);
            string.push_str(dash);
            string.push_str(comma);
        }

        let encoded_value = encode_special_characters(&self.start);
        string.push_str(&encoded_value);

        if let Some(value) = &self.end {
            let encoded_value = encode_special_characters(value);

            string.push_str(comma);
            string.push_str(&encoded_value);
        }

        if let Some(value) = &self.suffix {
            let encoded_value = encode_special_characters(value);

            string.push_str(comma);
            string.push_str(dash);
            string.push_str(&encoded_value);
        }

        string
    }
}

fn normalize_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_newlines_and_tabs_to_spaces() {
        let f = TextFragment::new(
            "first line\nsecond line".into(), None, None, None);

        assert_eq!(f.start, "first line second line");
    }

    #[test]
    fn normalizes_windows_line_endings() {
        let f = TextFragment::new(
            "first line\r\nsecond line".into(), None, None, None);

        assert_eq!(f.start, "first line second line");
    }

    #[test]
    fn should_return_directive() {
        let mut fragment =
            TextFragment::new(String::from("human"), Some(String::from("URL")), None, None);

        assert_eq!(fragment.to_directive(), "#:~:text=human,URL");

        fragment = TextFragment::new(
            String::from(
                "The first recorded idea of using digital electronics for computing was the 1931 paper \"The Use of Thyratrons for High Speed Automatic Counting of Physical Phenomena\" by C. E. Wynn-Williams.",
            ),
            None,
            None,
            None,
        );

        assert_eq!(
            fragment.to_directive(),
            "#:~:text=The%20first%20recorded%20idea%20of%20using%20digital%20electronics%20for%20computing%20was%20the%201931%20paper%20%22The%20Use%20of%20Thyratrons%20for%20High%20Speed%20Automatic%20Counting%20of%20Physical%20Phenomena%22%20by%20C.%20E.%20Wynn-Williams."
        );

        fragment = TextFragment::new(
            String::from("linked URL"),
            Some(String::from("defining a value")),
            None,
            None,
        );

        assert_eq!(
            fragment.to_directive(),
            "#:~:text=linked%20URL,defining%20a%20value"
        );

        fragment = TextFragment::new(
            String::from("مِصر"),
            None,
            Some(String::from("البحرين")),
            None,
        );

        assert_eq!(
            fragment.to_directive(),
            "#:~:text=%D8%A7%D9%84%D8%A8%D8%AD%D8%B1%D9%8A%D9%86-,%D9%85%D9%90%D8%B5%D8%B1"
        );

        fragment = TextFragment::new(
            String::from("The Referer"),
            Some(String::from("be sent")),
            Some(String::from("downgrade:")),
            Some(String::from("to origins")),
        );

        assert_eq!(
            fragment.to_directive(),
            "#:~:text=downgrade%3A-,The%20Referer,be%20sent,-to%20origins"
        );
    }
}
