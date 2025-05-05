use std::borrow::Cow;

use crate::themes::Theme;

/// A simple colorless using arrows theme
///
/// # Examples
///
/// ```
/// use termdiff::{diff, ArrowsTheme};
/// let old = "The quick brown fox and\njumps over the sleepy dog";
/// let new = "The quick red fox and\njumps over the lazy dog";
/// let mut buffer: Vec<u8> = Vec::new();
/// diff(&mut buffer, old, new, &ArrowsTheme::default()).unwrap();
/// let actual: String = String::from_utf8(buffer).expect("Not valid UTF-8");
///
/// assert_eq!(
///     actual,
///     "< left / > right
/// <The quick brown fox and
/// <jumps over the sleepy dog
/// >The quick red fox and
/// >jumps over the lazy dog
/// "
/// );
/// ```
#[derive(Default, Debug, Copy, Clone)]
pub struct ArrowsTheme {}

impl Theme for ArrowsTheme {
    fn equal_prefix<'this>(&self) -> Cow<'this, str> {
        " ".into()
    }

    fn delete_prefix<'this>(&self) -> Cow<'this, str> {
        "< ".into()
    }

    fn insert_prefix<'this>(&self) -> Cow<'this, str> {
        "> ".into()
    }

    fn header<'this>(&self) -> Cow<'this, str> {
        "< left / > right\n".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;

    /// Test that `ArrowsTheme` returns the expected values for all methods
    #[test]
    fn test_arrows_theme_prefixes() {
        let theme = ArrowsTheme::default();
        assert_eq!(theme.equal_prefix(), Cow::Borrowed(" "));
        assert_eq!(theme.delete_prefix(), Cow::Borrowed("<"));
        assert_eq!(theme.insert_prefix(), Cow::Borrowed(">"));
    }

    /// Test that `ArrowsTheme` returns the expected header
    #[test]
    fn test_arrows_theme_header() {
        let theme = ArrowsTheme::default();
        assert_eq!(theme.header(), Cow::Borrowed("< left / > right\n"));
    }

    /// Test that `ArrowsTheme` uses default implementations for content formatting
    #[test]
    fn test_arrows_theme_content_formatting() {
        let theme = ArrowsTheme::default();
        let input = "test";
        assert_eq!(theme.highlight_insert(input), Cow::Borrowed(input));
        assert_eq!(theme.highlight_delete(input), Cow::Borrowed(input));
        assert_eq!(theme.equal_content(input), Cow::Borrowed(input));
        assert_eq!(theme.delete_content(input), Cow::Borrowed(input));
        assert_eq!(theme.insert_line(input), Cow::Borrowed(input));
    }
}
