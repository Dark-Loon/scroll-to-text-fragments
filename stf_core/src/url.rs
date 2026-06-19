use thiserror::Error;
use url::Url;

use crate::fragment::TextFragment;

pub fn build_url(base: &str, fragment: &TextFragment) -> Result<String, FragmentError> {
    if fragment.start.trim().is_empty() {
        return Err(FragmentError::EmptyTextStart);
    }

    let base = Url::parse(base).map_err(|e| FragmentError::InvalidBaseUrl(e.to_string()))?;

    let directive = fragment.to_directive();

    let combined = base
        .join(&directive)
        .map_err(|e| FragmentError::InvalidBaseUrl(e.to_string()))?;
    let result = combined.to_string();

    Ok(combined.to_string())
}

#[derive(Debug, Error, PartialEq)]
pub enum FragmentError {
    #[error("invalid base URL: {0}")]
    InvalidBaseUrl(String),

    #[error("text_start must not be empty")]
    EmptyTextStart,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_text_start() {
        let fragment = TextFragment::new(String::new(), None, None, None);

        assert_eq!(
            build_url("https://example.com", &fragment),
            Err(FragmentError::EmptyTextStart)
        );
    }

    #[test]
    fn rejects_whitespace_only_text_start() {
        let fragment = TextFragment::new(String::from("    "), None, None, None);

        assert_eq!(
            build_url("https://example.com", &fragment),
            Err(FragmentError::EmptyTextStart)
        );
    }

    #[test]
    fn rejects_invalid_base_url() {
        let fragment = TextFragment::new(String::from("human"), None, None, None);

        assert_eq!(
            build_url("not a url", &fragment),
            Err(FragmentError::InvalidBaseUrl(
                "relative URL without a base".into()
            ))
        );
    }

    #[test]
    fn should_return_full_url() {
        let mut fragment =
            TextFragment::new(String::from("human"), Some(String::from("URL")), None, None);

        assert_eq!(
            build_url("https://example.com", &fragment),
            Ok(String::from("https://example.com/#:~:text=human,URL"))
        );

        fragment = TextFragment::new(
            String::from(
                "The first recorded idea of using digital electronics for computing was the 1931 paper \"The Use of Thyratrons for High Speed Automatic Counting of Physical Phenomena\" by C. E. Wynn-Williams.",
            ),
            None,
            None,
            None,
        );

        assert_eq!(
            build_url("https://example.com", &fragment),
            Ok(String::from(
                "https://example.com/#:~:text=The%20first%20recorded%20idea%20of%20using%20digital%20electronics%20for%20computing%20was%20the%201931%20paper%20%22The%20Use%20of%20Thyratrons%20for%20High%20Speed%20Automatic%20Counting%20of%20Physical%20Phenomena%22%20by%20C.%20E.%20Wynn-Williams."
            ))
        );

        fragment = TextFragment::new(
            String::from("linked URL"),
            Some(String::from("defining a value")),
            None,
            None,
        );

        assert_eq!(
            build_url("https://example.com", &fragment),
            Ok(String::from(
                "https://example.com/#:~:text=linked%20URL,defining%20a%20value"
            ))
        );

        fragment = TextFragment::new(
            String::from("مِصر"),
            None,
            Some(String::from("البحرين")),
            None,
        );

        assert_eq!(
            build_url("https://example.com", &fragment),
            Ok(String::from(
                "https://example.com/#:~:text=%D8%A7%D9%84%D8%A8%D8%AD%D8%B1%D9%8A%D9%86-,%D9%85%D9%90%D8%B5%D8%B1"
            ))
        );

        fragment = TextFragment::new(
            String::from("The Referer"),
            Some(String::from("be sent")),
            Some(String::from("downgrade:")),
            Some(String::from("to origins")),
        );

        assert_eq!(
            build_url(
                "https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/a",
                &fragment
            ),
            Ok(String::from(
                "https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/a#:~:text=downgrade%3A-,The%20Referer,be%20sent,-to%20origins"
            ))
        );

        fragment = TextFragment::new(
            String::from("The Referer"),
            Some(String::from("be sent")),
            Some(String::from("downgrade:")),
            Some(String::from("to origins")),
        );

        assert_eq!(
            build_url(
                "https://example.com/path/to/page?name=ferret&color=purple",
                &fragment
            ),
            Ok(String::from(
                "https://example.com/path/to/page?name=ferret&color=purple#:~:text=downgrade%3A-,The%20Referer,be%20sent,-to%20origins"
            ))
        );
    }
}
