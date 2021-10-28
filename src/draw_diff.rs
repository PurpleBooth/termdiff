use std::fmt::{Display, Formatter};

use crossterm::style::{StyledContent, Stylize};
use similar::{ChangeTag, DiffableStr, TextDiff};

use super::themes::Theme;

/// The struct that draws the diff
///
/// Uses similar under the hood
pub struct DrawDiff<'a> {
    old: &'a str,
    new: &'a str,
    theme: &'a Theme,
}

impl DrawDiff<'_> {
    /// Make a new instance of the diff drawer
    ///
    /// # Examples
    ///
    /// ```
    /// use termdiff::{arrows_theme, DrawDiff};
    /// let  theme = arrows_theme();
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
    pub const fn new<'a>(old: &'a str, new: &'a str, theme: &'a Theme) -> DrawDiff<'a> {
        DrawDiff { old, new, theme }
    }

    fn highlight(&self, text: String, tag: ChangeTag) -> StyledContent<String> {
        match tag {
            ChangeTag::Equal => text.stylize(),
            ChangeTag::Delete => (self.theme.highlight_delete)(text),
            ChangeTag::Insert => (self.theme.highlight_insert)(text),
        }
    }

    fn format_line(&self, line: String, tag: ChangeTag) -> StyledContent<String> {
        match tag {
            ChangeTag::Equal => (self.theme.equal_content)(line),
            ChangeTag::Delete => (self.theme.delete_content)(line),
            ChangeTag::Insert => (self.theme.insert_line)(line),
        }
    }

    fn prefix(&self, tag: ChangeTag) -> &str {
        match tag {
            ChangeTag::Equal => &self.theme.equal_prefix,
            ChangeTag::Delete => &self.theme.delete_prefix,
            ChangeTag::Insert => &self.theme.insert_prefix,
        }
    }
}

impl Display for DrawDiff<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.theme.header)?;
        let diff = TextDiff::from_lines(self.old, self.new);

        for op in diff.ops() {
            for change in diff.iter_inline_changes(op) {
                write!(f, "{}", self.prefix(change.tag()))?;

                for (highlight, inline_change) in change.values() {
                    if *highlight {
                        let highlighted = self
                            .highlight(inline_change.to_string_lossy().to_string(), change.tag());
                        write!(
                            f,
                            "{}",
                            self.format_line(highlighted.to_string(), change.tag())
                        )?;
                    } else {
                        write!(
                            f,
                            "{}",
                            self.format_line((*inline_change).to_string(), change.tag())
                        )?;
                    }
                }

                if change.missing_newline() {
                    write!(f, "{}", self.theme.line_end)?;
                }
            }
        }

        Ok(())
    }
}

impl From<DrawDiff<'_>> for String {
    fn from(diff: DrawDiff<'_>) -> Self {
        format!("{}", diff)
    }
}

#[cfg(test)]
mod test {
    use super::{
        super::themes::{arrows_color_theme, arrows_theme},
        DrawDiff,
    };

    #[test]
    fn single_characters() {
        let old = "a\nb\nc";
        let new = "a\nc\n";
        let theme = arrows_theme();
        let actual: DrawDiff = DrawDiff::new(old, new, &theme);

        assert_eq!(
            format!("{}", actual),
            "< left / > right
 a
<b
<c
>c
"
        );
    }

    #[test]
    fn one_line() {
        let old = "adc";
        let new = "abc";
        let theme = arrows_theme();
        let actual: DrawDiff = DrawDiff::new(old, new, &theme);
        assert_eq!(
            format!("{}", actual),
            "< left / > right
<adc
>abc
"
        );
    }

    #[test]
    fn line_by_line() {
        let old = "The quick brown fox and\njumps over the sleepy dog";
        let new = "The quick red fox and\njumps over the lazy dog";
        let theme = arrows_theme();
        let actual: DrawDiff = DrawDiff::new(old, new, &theme);
        assert_eq!(
            format!("{}", actual),
            "< left / > right
<The quick brown fox and
<jumps over the sleepy dog
>The quick red fox and
>jumps over the lazy dog
"
        );
    }

    #[test]
    fn into_string() {
        let old = "The quick brown fox and\njumps over the sleepy dog";
        let new = "The quick red fox and\njumps over the lazy dog";
        let actual: String = DrawDiff::new(old, new, &arrows_theme()).into();
        assert_eq!(
            actual,
            "< left / > right
<The quick brown fox and
<jumps over the sleepy dog
>The quick red fox and
>jumps over the lazy dog
"
        );
    }

    #[test]
    fn its_customisable() {
        let old = "The quick brown fox and\njumps over the sleepy dog";
        let new = "The quick red fox and\njumps over the lazy dog";
        let theme = arrows_color_theme();
        let actual: DrawDiff = DrawDiff::new(old, new, &theme);

        assert_eq!(
            format!("{}", actual),
            "\u{1b}[38;5;9m< left\u{1b}[39m / \u{1b}[38;5;10m> right\u{1b}[39m
\u{1b}[38;5;9m<\u{1b}[39m\u{1b}[38;5;9mThe quick \u{1b}[39m\u{1b}[38;5;9m\u{1b}[4mbrown\u{1b}[0m\u{1b}[39m\u{1b}[38;5;9m fox and
\u{1b}[39m\u{1b}[38;5;9m<\u{1b}[39m\u{1b}[38;5;9mjumps over the \u{1b}[39m\u{1b}[38;5;9m\u{1b}[4msleepy\u{1b}[0m\u{1b}[39m\u{1b}[38;5;9m dog\u{1b}[39m
\u{1b}[38;5;10m>\u{1b}[39m\u{1b}[38;5;10mThe quick \u{1b}[39m\u{1b}[38;5;10m\u{1b}[4mred\u{1b}[0m\u{1b}[39m\u{1b}[38;5;10m fox and
\u{1b}[39m\u{1b}[38;5;10m>\u{1b}[39m\u{1b}[38;5;10mjumps over the \u{1b}[39m\u{1b}[38;5;10m\u{1b}[4mlazy\u{1b}[0m\u{1b}[39m\u{1b}[38;5;10m dog\u{1b}[39m
"
        );
    }
}
