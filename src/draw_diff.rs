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
    ///     "< left / > right\n<The quick brown fox and\n<jumps over the sleepy dog\n>The quick red fox and\n>jumps over the lazy dog\n"
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
    ///     "< left / > right\n<The quick brown fox and\n<jumps over the sleepy dog\n>The quick red fox and\n>jumps over the lazy dog\n"
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
