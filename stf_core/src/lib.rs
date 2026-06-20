pub mod encode;
pub mod fragment;
pub mod url;

pub use fragment::TextFragment;
pub use url::{FragmentError, build_url};
