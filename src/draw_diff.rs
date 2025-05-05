use std::{
    borrow::{Borrow, Cow},
    fmt::{Display, Formatter},
};

use super::themes::Theme;
use crate::diff_algorithm::{Algorithm, ChangeTag, DiffAlgorithmFactory};

/// The struct that draws the diff
///
/// Can use either the similar crate or our own Myers algorithm implementation
#[derive(Debug)]
pub struct DrawDiff<'a> {
    old: &'a str,
    new: &'a str,
    theme: &'a dyn Theme,
    algorithm: Algorithm,
}

impl<'input> DrawDiff<'input> {
    /// Make a new instance of the diff drawer
    ///
    /// # Examples
    ///
    /// ```
    /// use termdiff::{ArrowsTheme, DrawDiff};
    /// let theme = ArrowsTheme::default();
    /// assert_eq!(
    ///     format!(
    ///         "{}",
    ///         DrawDiff::new(
    ///             "The quick brown fox and\njumps over the sleepy dog",
    ///             "The quick red fox and\njumps over the lazy dog",
    ///             &theme
    ///         )
    ///     ),
    ///     "< left / > right
    /// <The quick brown fox and
    /// <jumps over the sleepy dog
    /// >The quick red fox and
    /// >jumps over the lazy dog
    /// "
    /// );
    /// ```
    #[must_use]
    pub fn new<'a>(old: &'a str, new: &'a str, theme: &'a dyn Theme) -> DrawDiff<'a> {
        DrawDiff {
            old,
            new,
            theme,
            algorithm: Algorithm::default(),
        }
    }

    /// Make a new instance of the diff drawer with a specific algorithm
    ///
    /// # Examples
    ///
    /// ```
    /// use termdiff::{Algorithm, ArrowsTheme, DrawDiff};
    /// let theme = ArrowsTheme::default();
    /// assert_eq!(
    ///     format!(
    ///         "{}",
    ///         DrawDiff::with_algorithm(
    ///             "The quick brown fox and\njumps over the sleepy dog",
    ///             "The quick red fox and\njumps over the lazy dog",
    ///             &theme,
    ///             Algorithm::Myers
    ///         )
    ///     ),
    ///     "< left / > right
    /// <The quick brown fox and
    /// <jumps over the sleepy dog
    /// >The quick red fox and
    /// >jumps over the lazy dog
    /// "
    /// );
    /// ```
    #[must_use]
    pub fn with_algorithm<'a>(
        old: &'a str,
        new: &'a str,
        theme: &'a dyn Theme,
        algorithm: Algorithm,
    ) -> DrawDiff<'a> {
        DrawDiff {
            old,
            new,
            theme,
            algorithm,
        }
    }

    fn highlight(&self, text: &'input str, tag: ChangeTag) -> Cow<'input, str> {
        match tag {
            ChangeTag::Equal => text.into(),
            ChangeTag::Delete => self.theme.highlight_delete(text),
            ChangeTag::Insert => self.theme.highlight_insert(text),
        }
    }

    fn format_line(&self, line: &'input str, tag: ChangeTag) -> Cow<'input, str> {
        match tag {
            ChangeTag::Equal => self.theme.equal_content(line),
            ChangeTag::Delete => self.theme.delete_content(line),
            ChangeTag::Insert => self.theme.insert_line(line),
        }
    }

    fn prefix(&self, tag: ChangeTag) -> Cow<'input, str> {
        match tag {
            ChangeTag::Equal => self.theme.equal_prefix(),
            ChangeTag::Delete => self.theme.delete_prefix(),
            ChangeTag::Insert => self.theme.insert_prefix(),
        }
    }

    fn replace_trailing_if_needed(
        &self,
        old: &'input str,
        new: &'input str,
    ) -> (Cow<'input, str>, Cow<'input, str>) {
        if old.chars().last() == new.chars().last() {
            (old.into(), new.into())
        } else {
            (self.replace_trailing_nl(old), self.replace_trailing_nl(new))
        }
    }

    fn replace_trailing_nl(&self, x: &'input str) -> Cow<'input, str> {
        x.strip_suffix('\n').map_or_else(
            || x.into(),
            |stripped| {
                let marker = self.theme.trailing_lf_marker();
                if marker.is_empty() {
                    x.into()
                } else {
                    format!("{stripped}{marker}\n").into()
                }
            },
        )
    }
}

impl Display for DrawDiff<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Check if any algorithms are available
        if !Algorithm::has_available_algorithms() {
            return write!(f, "Error: No diff algorithms are available. Enable either 'myers' or 'similar' feature.");
        }

        // Process trailing newlines and write header
        let (old, new) = self.replace_trailing_if_needed(self.old, self.new);

        // Write header
        let header = self.theme.header();
        write!(f, "{header}")?;

        // Create diff algorithm
        let diff_algorithm = DiffAlgorithmFactory::create(self.algorithm);

        // Get operations
        let ops = diff_algorithm.ops(&old, &new);

        // Process operations
        for op in &ops {
            for change in diff_algorithm.iter_inline_changes(&old, &new, op) {
                // Write prefix
                write!(f, "{}", self.prefix(change.tag()))?;

                // Process each value in the change using functional approach
                for (highlight, inline_change) in change.values() {
                    if *highlight {
                        write!(
                            f,
                            "{}",
                            self.format_line(
                                self.highlight(inline_change.borrow(), change.tag())
                                    .borrow(),
                                change.tag()
                            )
                        )?;
                    } else {
                        write!(
                            f,
                            "{}",
                            self.format_line(inline_change.borrow(), change.tag())
                        )?;
                    }
                }

                // Add line end if needed
                if change.missing_newline() {
                    write!(f, "{}", self.theme.line_end())?;
                }
            }
        }

        Ok(())
    }
}

impl From<DrawDiff<'_>> for String {
    fn from(diff: DrawDiff<'_>) -> Self {
        format!("{diff}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::themes::ArrowsTheme;
    use std::borrow::Cow;

    /// Test that `DrawDiff::new` creates a new instance with the provided values
    #[test]
    fn test_draw_diff_new() {
        let old = "old";
        let new = "new";
        let theme = ArrowsTheme::default();

        let diff = DrawDiff::new(old, new, &theme);
        let output = format!("{diff}");
        
        // Check header and formatted output
        assert!(output.starts_with("< left / > right\n"));
        assert!(output.contains("< old\n> new"));
    }

    /// Test that `DrawDiff` correctly handles identical inputs
    #[test]
    fn test_draw_diff_identical_inputs() {
        let text = "same text";
        let theme = ArrowsTheme::default();

        let diff = DrawDiff::new(text, text, &theme);
        let output = format!("{diff}");

        // Should only contain the header and the unchanged text
        assert!(output.contains("< left / > right"));
        assert!(output.contains(" same text"));
        assert!(!output.contains("<same text"));
        assert!(!output.contains(">same text"));
    }

    /// Test that `DrawDiff` correctly handles empty inputs
    #[test]
    fn test_draw_diff_empty_inputs() {
        let theme = ArrowsTheme::default();

        let diff = DrawDiff::new("", "", &theme);
        let output = format!("{diff}");

        // Should only contain the header
        assert_eq!(output, "< left / > right\n");
    }

    /// Test that `DrawDiff` correctly handles inputs with only whitespace differences
    #[test]
    fn test_draw_diff_whitespace_differences() {
        let old = "text with spaces";
        let new = "text  with  spaces"; // Extra spaces
        let theme = ArrowsTheme::default();

        let diff = DrawDiff::new(old, new, &theme);
        let output = format!("{diff}");

        // Should show the differences
        assert!(output.contains("<text with spaces"));
        assert!(output.contains(">text  with  spaces"));
    }

    /// Test that `DrawDiff` correctly handles multiline inputs
    #[test]
    fn test_draw_diff_multiline() {
        let old = "line 1\nline 2\nline 3";
        let new = "line 1\nmodified line 2\nline 3";
        let theme = ArrowsTheme::default();

        let diff = DrawDiff::new(old, new, &theme);
        let output = format!("{diff}");

        // Verify the diff shows changes correctly
        assert!(output.contains(" line 1\n"));
        assert!(output.contains("< line 2\n"));
        assert!(output.contains("> modified line 2\n"));
        assert!(output.contains(" line 3"));
    }

    /// Test that `DrawDiff` correctly handles trailing newline differences
    #[test]
    fn test_draw_diff_trailing_newline() {
        let old = "line\n";
        let new = "line";
        let theme = ArrowsTheme::default();

        let diff = DrawDiff::new(old, new, &theme);
        let output = format!("{diff}");

        // Should show the newline difference with the marker
        assert!(output.contains("line␊"));
    }

    /// Test that `DrawDiff` correctly handles inputs with only newline differences
    #[test]
    fn test_draw_diff_only_newline_differences() {
        let old = "line 1\nline 2\n";
        let new = "line 1\nline 2";
        let theme = ArrowsTheme::default();

        let diff = DrawDiff::new(old, new, &theme);
        let output = format!("{diff}");

        // Should show the newline difference with the marker
        assert!(output.contains("line 2␊"));
    }

    /// Test that `DrawDiff` correctly handles completely different inputs
    #[test]
    fn test_draw_diff_completely_different() {
        let old = "old text";
        let new = "new text";
        let theme = ArrowsTheme::default();

        let diff = DrawDiff::new(old, new, &theme);
        let output = format!("{diff}");

        // Verify complete replacement
        assert!(output.contains("< old text\n> new text"));
    }

    /// Test that `DrawDiff` correctly handles inputs with partial differences
    #[test]
    fn test_draw_diff_partial_differences() {
        
        let old = "the quick brown fox";
        let new = "the quick red fox";
        let theme = ArrowsTheme::default();

        let diff = DrawDiff::new(old, new, &theme);
        let output = format!("{diff}");

        // Should show the entire lines as changed
        assert!(output.contains("<the quick brown fox"));
        assert!(output.contains(">the quick red fox"));
    }

    /// Test that `DrawDiff` can be converted to a String
    #[test]
    fn test_draw_diff_to_string() {
        let old = "old";
        let new = "new";
        let theme = ArrowsTheme::default();

        let diff = DrawDiff::new(old, new, &theme);
        let output: String = diff.into();

        assert!(output.contains("<old"));
        assert!(output.contains(">new"));
    }

    /// Test that `DrawDiff` works with a custom theme
    #[test]
    fn test_draw_diff_with_custom_theme() {
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
        let theme = CustomTheme;

        let diff = DrawDiff::new(old, new, &theme);
        let output = format!("{diff}");

        assert!(output.contains("CUSTOM HEADER"));
        assert!(output.contains("-old"));
        assert!(output.contains("+new"));
    }

    /// Test that `DrawDiff` correctly handles inputs with multiple changes
    #[test]
    fn test_draw_diff_multiple_changes() {
        let old = "line 1\nline 2\nline 3\nline 4";
        let new = "line 1\nmodified line 2\nline 3\nmodified line 4";
        let theme = ArrowsTheme::default();

        let diff = DrawDiff::new(old, new, &theme);
        let output = format!("{diff}");

        // Verify multiple changes
        assert!(output.contains(" line 1\n"));
        assert!(output.contains("< line 2\n"));
        assert!(output.contains("> modified line 2\n"));
        assert!(output.contains(" line 3\n"));
        assert!(output.contains("< line 4\n"));
        assert!(output.contains("> modified line 4"));
    }

    /// Test that `DrawDiff` correctly handles inputs with added lines
    #[test]
    fn test_draw_diff_added_lines() {
        let old = "line 1\nline 3";
        let new = "line 1\nline 2\nline 3";
        let theme = ArrowsTheme::default();

        let diff = DrawDiff::new(old, new, &theme);
        let output = format!("{diff}");

        // Verify added line
        assert!(output.contains(" line 1\n"));
        assert!(output.contains("> line 2\n"));
        assert!(output.contains(" line 3"));
    }

    /// Test that `DrawDiff` correctly handles inputs with removed lines
    #[test]
    fn test_draw_diff_removed_lines() {
        let old = "line 1\nline 2\nline 3";
        let new = "line 1\nline 3";
        let theme = ArrowsTheme::default();

        let diff = DrawDiff::new(old, new, &theme);
        let output = format!("{diff}");

        // Should show the removed line
        assert!(output.contains(" line 1"));
        assert!(output.contains("<line 2"));
        assert!(output.contains(" line 3"));
    }

    /// Test that `DrawDiff` correctly handles custom trailing newline markers
    #[test]
    fn test_draw_diff_custom_newline_marker() {
        #[derive(Debug)]
        struct CustomMarkerTheme;

        impl Theme for CustomMarkerTheme {
            fn equal_prefix<'this>(&self) -> Cow<'this, str> {
                " ".into()
            }

            fn delete_prefix<'this>(&self) -> Cow<'this, str> {
                "-".into()
            }

            fn insert_prefix<'this>(&self) -> Cow<'this, str> {
                "+".into()
            }

            fn header<'this>(&self) -> Cow<'this, str> {
                "HEADER\n".into()
            }

            fn trailing_lf_marker<'this>(&self) -> Cow<'this, str> {
                "[NEWLINE]".into()
            }
        }

        let old = "line\n";
        let new = "line";
        let theme = CustomMarkerTheme;

        let diff = DrawDiff::new(old, new, &theme);
        let output = format!("{diff}");

        // Should show the custom newline marker
        assert!(output.contains("line[NEWLINE]"));
    }
}
