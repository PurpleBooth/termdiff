use std::borrow::Cow;

use crate::themes::Theme;

/// A simple colorless using signs theme
///
/// # Examples
///
/// ```
/// use termdiff::{diff, SignsTheme};
/// let old = "The quick brown fox and\njumps over the sleepy dog";
/// let new = "The quick red fox and\njumps over the lazy dog";
/// let mut buffer: Vec<u8> = Vec::new();
/// let theme = SignsTheme::default();
/// diff(&mut buffer, old, new, &theme).unwrap();
/// let actual: String = String::from_utf8(buffer).expect("Not valid UTF-8");
///
/// assert_eq!(
///     actual,
///     "--- remove | insert +++
/// -The quick brown fox and
/// -jumps over the sleepy dog
/// +The quick red fox and
/// +jumps over the lazy dog
/// "
/// );
/// ```
#[derive(Default, Copy, Clone, Debug)]
pub struct SignsTheme {}

impl Theme for SignsTheme {
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
        format!("{} | {}\n", "--- remove", "insert +++").into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;

    /// Test that `SignsTheme` returns the expected prefixes
    #[test]
    fn test_signs_theme_prefixes() {
        let theme = SignsTheme::default();
        assert_eq!(theme.equal_prefix(), Cow::Borrowed(" "));
        assert_eq!(theme.delete_prefix(), Cow::Borrowed("-"));
        assert_eq!(theme.insert_prefix(), Cow::Borrowed("+"));
    }

    /// Test that `SignsTheme` returns the expected header
    #[test]
    fn test_signs_theme_header() {
        let theme = SignsTheme::default();
        assert_eq!(theme.header(), Cow::Borrowed("--- remove | insert +++\n"));
    }

    /// Test that `SignsTheme` uses default implementations for line endings and markers
    #[test]
    fn test_signs_theme_defaults() {
        let theme = SignsTheme::default();
        assert_eq!(theme.line_end(), Cow::Borrowed("\n"));
        assert_eq!(theme.trailing_lf_marker(), Cow::Borrowed("␊"));
    }
}
