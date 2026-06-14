/// Edge case tests
#[cfg(test)]
mod tests {
    use crate::{ArrowsTheme, DrawDiff, SignsTheme, Theme};

    /// The trailing newline marker (␊) should appear when old ends with \n
    /// but new does not, even when the last lines are completely different.
    #[test]
    fn test_trailing_nl_marker_on_different_lines() {
        let old = "apple\norange\n";
        let new = "apple\nbanana";

        let theme = ArrowsTheme::default();
        let output = format!("{}", DrawDiff::new(old, new, &theme));

        // The marker should appear on the old side (it had a trailing newline)
        assert!(
            output.contains('␊'),
            "Expected trailing LF marker when old has trailing newline and new does not"
        );
    }

    /// CRLF in old should produce a visible \r in the diff output.
    #[test]
    fn test_crlf_handling() {
        let old = "line\r\n";
        let new = "line";

        let theme = ArrowsTheme::default();
        let output = format!("{}", DrawDiff::new(old, new, &theme));

        // The \r should be visible in the diff because the library treats
        // \n as the line separator, leaving \r as line content.
        assert!(
            output.contains('\r'),
            "Expected carriage return in diff output for CRLF input"
        );
    }

    /// When both strings are completely different and one has a trailing
    /// newline, the marker should still appear.
    #[test]
    fn test_completely_different_with_trailing_nl() {
        let old = "hello\n";
        let new = "world";

        let theme = ArrowsTheme::default();
        let output = format!("{}", DrawDiff::new(old, new, &theme));

        assert!(
            output.contains('␊'),
            "Expected trailing LF marker when old has trailing newline"
        );
        assert!(output.contains("<hello"));
        assert!(output.contains(">world"));
    }

    /// When only the trailing newline differs in multiline input, the diff
    /// should show only the newline difference, not the entire content.
    #[test]
    fn test_multiline_trailing_nl_only() {
        let old = "line1\nline2\nline3\n";
        let new = "line1\nline2\nline3";

        let theme = ArrowsTheme::default();
        let output = format!("{}", DrawDiff::new(old, new, &theme));

        // The first two lines should be unchanged
        assert!(output.contains(" line1\n"));
        assert!(output.contains(" line2\n"));
        // The trailing newline difference should be shown with the marker
        assert!(output.contains('␊'));
    }

    /// Unicode characters should be handled correctly in diffs.
    #[test]
    fn test_unicode() {
        let old = "héllo\nwörld";
        let new = "héllo\nwörld!";

        let theme = ArrowsTheme::default();
        let output = format!("{}", DrawDiff::new(old, new, &theme));

        assert!(output.contains(" héllo\n"));
        assert!(output.contains("<wörld"));
        assert!(output.contains(">wörld!"));
    }

    /// A standard diff should produce the expected output.
    #[test]
    fn test_simple_output() {
        let old = "The quick brown fox and\njumps over the sleepy dog";
        let new = "The quick red fox and\njumps over the lazy dog";
        let theme = ArrowsTheme::default();
        let output = format!("{}", DrawDiff::new(old, new, &theme));

        assert!(output.contains("<The quick brown fox and"));
        assert!(output.contains("<jumps over the sleepy dog"));
        assert!(output.contains(">The quick red fox and"));
        assert!(output.contains(">jumps over the lazy dog"));
    }

    /// Single character strings should produce a complete replacement diff.
    #[test]
    fn test_single_char() {
        let old = "a";
        let new = "b";

        let theme = ArrowsTheme::default();
        let output = format!("{}", DrawDiff::new(old, new, &theme));

        assert!(output.contains("<a"));
        assert!(output.contains(">b"));
    }

    /// When new has a trailing newline but old doesn't, the marker should
    /// appear on the new side.
    #[test]
    fn test_new_has_trailing_nl() {
        let old = "line";
        let new = "line\n";

        let theme = ArrowsTheme::default();
        let output = format!("{}", DrawDiff::new(old, new, &theme));

        assert!(
            output.contains('␊'),
            "Expected trailing LF marker when new has trailing newline and old does not"
        );
    }

    /// `SignsTheme` should return the expected header.
    #[test]
    fn test_signs_header() {
        let theme = SignsTheme::default();
        let header = theme.header();
        assert_eq!(header, "--- remove | insert +++\n");
    }

    /// The cmd.rs doc example should produce the expected output.
    #[test]
    fn test_cmd_doc_example() {
        let old = "a\nb\nc";
        let new = "a\nc\n";

        let theme = ArrowsTheme::default();
        let output = format!("{}", DrawDiff::new(old, new, &theme));

        let expected = "< left / > right\n a\n<b\n<c\n>c␊\n";
        assert_eq!(output, expected);
    }
}
