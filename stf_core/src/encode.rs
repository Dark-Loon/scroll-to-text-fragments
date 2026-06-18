#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn encodes_text_fragment() {
        assert_eq!("%3A", encode_special_characters(":"));
        assert_eq!("%2F", encode_special_characters("/"));
        assert_eq!("%3F", encode_special_characters("?"));
        assert_eq!("%23", encode_special_characters("#"));
        assert_eq!("%5B", encode_special_characters("["));
        assert_eq!("%5D", encode_special_characters("]"));
        assert_eq!("%40", encode_special_characters("@"));
        assert_eq!("%21", encode_special_characters("!"));
        assert_eq!("%24", encode_special_characters("$"));
        assert_eq!("%26", encode_special_characters("&"));
        assert_eq!("%27", encode_special_characters("'"));
        assert_eq!("%28", encode_special_characters("("));
        assert_eq!("%29", encode_special_characters(")"));
        assert_eq!("%2A", encode_special_characters("*"));
        assert_eq!("%2B", encode_special_characters("+"));
        assert_eq!("%2C", encode_special_characters(","));
        assert_eq!("%3B", encode_special_characters(";"));
        assert_eq!("%3D", encode_special_characters("="));
        assert_eq!("%25", encode_special_characters("%"));
        assert_eq!("%20", encode_special_characters(" "));
    }
}

pub fn encode_special_characters(char: &str) -> &str {
    todo!()
}
