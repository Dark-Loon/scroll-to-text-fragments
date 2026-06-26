//! Generate [text fragment](https://wicg.github.io/scroll-to-text-fragment) URLs
//! that link directly to specific text on a web page.
//!
//! The main entry point is [`build_url`], which takes a base URL and a
//! [`TextFragment`] and returns the complete text fragment URL.

pub(crate) mod encode;
pub mod fragment;
pub mod range;
pub mod url;

pub use fragment::TextFragment;
pub use range::extract_range;
pub use url::{FragmentError, build_url};
