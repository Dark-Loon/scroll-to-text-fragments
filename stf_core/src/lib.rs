pub mod encode;
pub mod fragment;
pub mod url;

pub use fragment::TextFragment;
pub use url::build_url;

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
