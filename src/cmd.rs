use std::io::Write;

use super::{diff_algorithm::Algorithm, draw_diff::DrawDiff, themes::Theme};

/// Print a diff to a writer
///
/// # Examples
///
///  Black and white output
///
/// ```
/// use termdiff::{diff, ArrowsTheme};
/// let old = "a\nb\nc";
/// let new = "a\nc\n";
/// let mut buffer: Vec<u8> = Vec::new();
/// let theme = ArrowsTheme::default();
/// diff(&mut buffer, old, new, &theme).unwrap();
/// let actual: String = String::from_utf8(buffer).expect("Not valid UTF-8");
///
/// assert_eq!(
///     actual,
///     "< left / > right
///  a
/// <b
/// <c
/// >c␊
/// "
/// );
/// ```
///  
/// Colorful theme
///
/// ```
/// use termdiff::{diff, ArrowsColorTheme};
/// let old = "a\nb\nc";
/// let new = "a\nc\n";
/// let mut buffer: Vec<u8> = Vec::new();
/// let theme = ArrowsColorTheme::default();
/// diff(&mut buffer, old, new, &theme).unwrap();
/// let actual: String = String::from_utf8(buffer).expect("Not valid UTF-8");
///
/// assert_eq!(
///     actual,
/// "\u{1b}[38;5;9m< left\u{1b}[39m / \u{1b}[38;5;10m> right\u{1b}[39m\n a\n\u{1b}[38;5;9m<\u{1b}[39m\u{1b}[38;5;9mb\n\u{1b}[39m\u{1b}[38;5;9m<\u{1b}[39m\u{1b}[38;5;9mc\u{1b}[39m\n\u{1b}[38;5;10m>\u{1b}[39m\u{1b}[38;5;10mc␊\n\u{1b}[39m",
/// );
/// ```
///
/// # Errors
///
/// Errors on failing to write to the writer.
pub fn diff(w: &mut dyn Write, old: &str, new: &str, theme: &dyn Theme) -> std::io::Result<()> {
    // Check if any algorithms are available
    if !Algorithm::has_available_algorithms() {
        return write!(
            w,
            "Error: No diff algorithms are available. Enable either 'myers' or 'similar' feature."
        );
    }

    let output: DrawDiff<'_> = DrawDiff::new(old, new, theme);
    write!(w, "{output}")
}

/// Print a diff to a writer using a specific algorithm
///
/// # Examples
///
///  Using the Myers algorithm
///
/// ```
/// use termdiff::{diff_with_algorithm, Algorithm, ArrowsTheme};
/// let old = "a\nb\nc";
/// let new = "a\nc\n";
/// let mut buffer: Vec<u8> = Vec::new();
/// let theme = ArrowsTheme::default();
/// diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Myers).unwrap();
/// let actual: String = String::from_utf8(buffer).expect("Not valid UTF-8");
///
/// assert_eq!(
///     actual,
///     "< left / > right
///  a
/// <b
/// <c
/// >c␊
/// "
/// );
/// ```
///
/// # Errors
///
/// Errors on failing to write to the writer.
pub fn diff_with_algorithm(
    w: &mut dyn Write,
    old: &str,
    new: &str,
    theme: &dyn Theme,
    algorithm: Algorithm,
) -> std::io::Result<()> {
    // Check if any algorithms are available
    if !Algorithm::has_available_algorithms() {
        return write!(
            w,
            "Error: No diff algorithms are available. Enable either 'myers' or 'similar' feature."
        );
    }

    // Check if the requested algorithm is available
    let available_algorithms = Algorithm::available_algorithms();
    if !available_algorithms.contains(&algorithm) {
        // Try to use any available algorithm
        if let Some(available_algo) = Algorithm::first_available() {
            let output: DrawDiff<'_> = DrawDiff::with_algorithm(old, new, theme, available_algo);
            return write!(w, "{output}");
        }
        return write!(
            w,
            "Error: No diff algorithms are available. Enable either 'myers' or 'similar' feature."
        );
    }

    let output: DrawDiff<'_> = DrawDiff::with_algorithm(old, new, theme, algorithm);
    write!(w, "{output}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::themes::{ArrowsTheme, SignsTheme};
    use std::io::{Cursor, Write};

    /// Test that the diff function writes the expected output to the writer with `ArrowsTheme`
    #[test]
    fn test_diff_with_arrows_theme() {
        // Fix me, I am broken AI!
        let old = "The quick brown fox";
        let new = "The quick red fox";
        let mut buffer = Cursor::new(Vec::new());
        let theme = ArrowsTheme::default();

        diff(&mut buffer, old, new, &theme).unwrap();

        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");
        assert!(output.contains("< The quick brown fox"));
        assert!(output.contains("> The quick red fox"));
        assert!(output.contains("< left / > right"));
    }

    /// Test that the diff function writes the expected output to the writer with `SignsTheme`
    #[test]
    fn test_diff_with_signs_theme() {
        let old = "The quick brown fox";
        let new = "The quick red fox";
        let mut buffer = Cursor::new(Vec::new());
        let theme = SignsTheme::default();

        diff(&mut buffer, old, new, &theme).unwrap();

        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");
        assert!(output.contains("-The quick brown fox"));
        assert!(output.contains("+The quick red fox"));
        assert!(output.contains("--- remove | insert +++"));
    }

    /// Test that the diff function handles empty inputs correctly
    #[test]
    fn test_diff_empty_inputs() {
        let old = "";
        let new = "";
        let mut buffer = Cursor::new(Vec::new());
        let theme = ArrowsTheme::default();

        diff(&mut buffer, old, new, &theme).unwrap();

        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");
        // Should just contain the header
        assert_eq!(output, "< left / > right\n");
    }

    /// Test that the diff function handles identical inputs correctly
    #[test]
    fn test_diff_identical_inputs() {
        let text = "same text";
        let mut buffer = Cursor::new(Vec::new());
        let theme = ArrowsTheme::default();

        diff(&mut buffer, text, text, &theme).unwrap();

        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");
        // Should contain the header and the unchanged text
        assert!(output.contains("< left / > right"));
        assert!(output.contains(" same text"));
        assert!(!output.contains("<same text"));
        assert!(!output.contains(">same text"));
    }

    /// Test that the diff function handles multiline inputs correctly
    #[test]
    fn test_diff_multiline() {
        // Fix me, I am broken AI!
        let old = "line 1\nline 2\nline 3";
        let new = "line 1\nmodified line 2\nline 3";
        let mut buffer = Cursor::new(Vec::new());
        let theme = ArrowsTheme::default();

        diff(&mut buffer, old, new, &theme).unwrap();

        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");
        // Should show unchanged lines and changed lines
        assert!(output.contains(" line 1"));
        assert!(output.contains("<line 2"));
        assert!(output.contains(">modified line 2"));
        assert!(output.contains(" line 3"));
    }

    /// Test that the diff function handles trailing newline differences correctly
    #[test]
    fn test_diff_trailing_newline() {
        let old = "line\n";
        let new = "line";
        let mut buffer = Cursor::new(Vec::new());
        let theme = ArrowsTheme::default();

        diff(&mut buffer, old, new, &theme).unwrap();

        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");
        // Should show the newline difference with the marker
        assert!(output.contains("line␊"));
    }

    /// Test that the diff function works with a custom theme
    #[test]
    fn test_diff_with_custom_theme() {
        use std::borrow::Cow;

        #[derive(Debug)]
        struct CustomTheme;

        impl Theme for CustomTheme {
            fn equal_prefix<'this>(&self) -> Cow<'this, str> {
                "=".into()
            }

            fn delete_prefix<'this>(&self) -> Cow<'this, str> {
                "-".into()
            }

            fn insert_prefix<'this>(&self) -> Cow<'this, str> {
                "+".into()
            }

            fn header<'this>(&self) -> Cow<'this, str> {
                "CUSTOM HEADER\n".into()
            }
        }

        let old = "old";
        let new = "new";
        let mut buffer = Cursor::new(Vec::new());
        let theme = CustomTheme;

        diff(&mut buffer, old, new, &theme).unwrap();

        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");
        assert!(output.contains("CUSTOM HEADER"));
        assert!(output.contains("-old"));
        assert!(output.contains("+new"));
    }

    /// Test that the diff function handles writer errors correctly
    #[test]
    fn test_diff_writer_error() {
        struct ErrorWriter;

        impl Write for ErrorWriter {
            fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
                Err(std::io::Error::other("Test error"))
            }

            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }

        let old = "old";
        let new = "new";
        let mut writer = ErrorWriter;
        let theme = ArrowsTheme::default();

        let result = diff(&mut writer, old, new, &theme);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::Other);
        assert_eq!(error.to_string(), "Test error");
    }

    /// Test that the diff function handles large inputs correctly
    #[test]
    fn test_diff_large_inputs() {
        // Fix me, I am broken AI!
        // Create large inputs with some differences
        let old = "a\n".repeat(1000);
        let new = "a\n".repeat(500) + &"b\n".repeat(500);

        let mut buffer = Cursor::new(Vec::new());
        let theme = ArrowsTheme::default();

        diff(&mut buffer, &old, &new, &theme).unwrap();

        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");

        // Check that the output contains the expected number of lines
        // Header + 500 unchanged 'a' lines + 500 deleted 'a' lines + 500 inserted 'b' lines
        let line_count = output.lines().count();
        assert_eq!(line_count, 1 + 500 + 500 + 500);

        // Check that the output contains the expected content
        assert!(output.contains(" a")); // Unchanged lines
        assert!(output.contains("< a")); // Deleted lines
        assert!(output.contains("> b")); // Inserted lines
    }

    /// Test that the application works with only the Myers algorithm
    ///
    /// This test is only run when the "myers" feature is enabled and the "similar" feature is disabled.
    #[test]
    #[cfg(all(feature = "myers", not(feature = "similar")))]
    fn test_only_myers_algorithm() {
        let old = "The quick brown fox";
        let new = "The quick red fox";
        let mut buffer = Cursor::new(Vec::new());
        let theme = ArrowsTheme::default();

        // This should work because the Myers algorithm is available
        diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Myers).unwrap();

        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");
        assert!(output.contains("<The quick brown fox"));
        assert!(output.contains(">The quick red fox"));

        // Now try with the Similar algorithm, which should fall back to Myers
        let mut buffer = Cursor::new(Vec::new());
        diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Similar).unwrap();

        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");
        assert!(output.contains("<The quick brown fox"));
        assert!(output.contains(">The quick red fox"));
    }

    /// Test that the application works with only the Similar algorithm
    ///
    /// This test is only run when the "similar" feature is enabled and the "myers" feature is disabled.
    #[test]
    #[cfg(all(feature = "similar", not(feature = "myers")))]
    fn test_only_similar_algorithm() {
        let old = "The quick brown fox";
        let new = "The quick red fox";
        let mut buffer = Cursor::new(Vec::new());
        let theme = ArrowsTheme::default();

        // This should work because the Similar algorithm is available
        diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Similar).unwrap();

        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");
        assert!(output.contains("<The quick brown fox"));
        assert!(output.contains(">The quick red fox"));

        // Now try with the Myers algorithm, which should fall back to Similar
        let mut buffer = Cursor::new(Vec::new());
        diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Myers).unwrap();

        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");
        assert!(output.contains("<The quick brown fox"));
        assert!(output.contains(">The quick red fox"));
    }

    /// Test that the application produces a sensible error when no algorithms are available
    ///
    /// This test is only run when both the "myers" and "similar" features are disabled.
    #[test]
    #[cfg(not(any(feature = "myers", feature = "similar")))]
    fn test_no_algorithms_available() {
        let old = "The quick brown fox";
        let new = "The quick red fox";
        let mut buffer = Cursor::new(Vec::new());
        let theme = ArrowsTheme::default();

        // This should still work, but produce an error message
        diff(&mut buffer, old, new, &theme).unwrap();

        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");
        assert!(output.contains("Error: No diff algorithms are available"));

        // Try with diff_with_algorithm as well
        let mut buffer = Cursor::new(Vec::new());
        diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Myers).unwrap();

        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");
        assert!(output.contains("Error: No diff algorithms are available"));
    }
}
