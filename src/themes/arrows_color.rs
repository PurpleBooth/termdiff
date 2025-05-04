use std::borrow::Cow;

use crate::themes::Theme;
use crossterm::style::Stylize;

/// A simple colorful theme using arrows
///
/// ```
/// use termdiff::{ArrowsColorTheme, diff};
/// let old = "The quick brown fox and\njumps over the sleepy dog";
/// let new = "The quick red fox and\njumps over the lazy dog";
/// let theme = ArrowsColorTheme::default();
/// let mut buffer: Vec<u8> = Vec::new();
/// diff(&mut buffer, old, new, &theme).unwrap();
/// let actual: String = String::from_utf8(buffer).expect("Not valid UTF-8");
///
/// assert_eq!(
///     actual,
///     "\u{1b}[38;5;9m< left\u{1b}[39m / \u{1b}[38;5;10m> right\u{1b}[39m\n\u{1b}[38;5;9m<\u{1b}[39m\u{1b}[38;5;9mThe quick brown fox and\n\u{1b}[39m\u{1b}[38;5;9m<\u{1b}[39m\u{1b}[38;5;9mjumps over the sleepy dog\u{1b}[39m\n\u{1b}[38;5;10m>\u{1b}[39m\u{1b}[38;5;10mThe quick red fox and\n\u{1b}[39m\u{1b}[38;5;10m>\u{1b}[39m\u{1b}[38;5;10mjumps over the lazy dog\u{1b}[39m\n"
/// );
/// ```
#[derive(Default, Debug, Clone, Copy)]
pub struct ArrowsColorTheme {}

impl Theme for ArrowsColorTheme {
    fn highlight_insert<'this>(&self, input: &'this str) -> Cow<'this, str> {
        input.underlined().to_string().into()
    }

    fn highlight_delete<'this>(&self, input: &'this str) -> Cow<'this, str> {
        input.underlined().to_string().into()
    }

    fn delete_content<'this>(&self, input: &'this str) -> Cow<'this, str> {
        input.red().to_string().into()
    }

    fn equal_prefix<'this>(&self) -> Cow<'this, str> {
        " ".into()
    }

    fn delete_prefix<'this>(&self) -> Cow<'this, str> {
        "<".red().to_string().into()
    }

    fn insert_line<'this>(&self, input: &'this str) -> Cow<'this, str> {
        input.green().to_string().into()
    }

    fn insert_prefix<'this>(&self) -> Cow<'this, str> {
        ">".green().to_string().into()
    }

    fn header<'this>(&self) -> Cow<'this, str> {
        format!("{} / {}\n", "< left".red(), "> right".green()).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;

    /// Test that `ArrowsColorTheme` returns the expected prefixes
    #[test]
    fn test_arrows_color_theme_prefixes() {
        let theme = ArrowsColorTheme::default();
        assert_eq!(theme.equal_prefix(), Cow::Borrowed(" "));
        // Can't directly compare colored strings, so check they contain the expected characters
        assert!(theme.delete_prefix().contains('<'));
        assert!(theme.insert_prefix().contains('>'));
    }

    /// Test that `ArrowsColorTheme` applies highlighting to content
    #[test]
    fn test_arrows_color_theme_highlighting() {
        let theme = ArrowsColorTheme::default();
        let input = "test";
        // Highlighting should modify the input
        assert_ne!(theme.highlight_insert(input), Cow::Borrowed(input));
        assert_ne!(theme.highlight_delete(input), Cow::Borrowed(input));
        assert_ne!(theme.delete_content(input), Cow::Borrowed(input));
        assert_ne!(theme.insert_line(input), Cow::Borrowed(input));
    }
}
