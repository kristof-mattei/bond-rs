use unicode_segmentation::UnicodeSegmentation as _;

/// Reverses a Unicode string.
pub fn reverse_unicode<S: AsRef<str>>(input: S) -> String {
    input.as_ref().graphemes(true).rev().collect()
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::reverse_unicode;

    #[test]
    fn ascii_reversed() {
        let input = "hello";

        let reversed = reverse_unicode(input);

        assert_eq!(reversed, "olleh");
    }

    #[test]
    fn utf8_single_character() {
        let input = "h\u{00e9}llo";

        let reversed = reverse_unicode(input);

        assert_eq!(reversed, "oll\u{00e9}h");
    }

    #[test]
    fn utf8_multi_character() {
        let input = "he\u{0301}llo";

        let reversed = reverse_unicode(input);

        // the `\u{0301}` should remain after the `e`
        assert_eq!(reversed, "olle\u{0301}h");
    }
}
