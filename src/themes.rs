use std::{borrow::Cow, fmt::Debug};

use crossterm::style::Stylize;

/// A [`Theme`] for the diff
///
/// This is to allows some control over what the diff looks like without having
/// to parse it yourself
pub trait Theme: Debug {
    /// How to format the text when highlighting it for inserts
    fn highlight_insert<'this>(&self, input: &'this str) -> Cow<'this, str> {
        input.into()
    }
    /// How to format the text when highlighting it for deletes
    fn highlight_delete<'this>(&self, input: &'this str) -> Cow<'this, str> {
        input.into()
    }
    /// How to format unchanged content
    fn equal_content<'this>(&self, input: &'this str) -> Cow<'this, str> {
        input.into()
    }
    /// How to format bits of text that are being removed
    fn delete_content<'this>(&self, input: &'this str) -> Cow<'this, str> {
        input.into()
    }
    /// The prefix to give lines that are equal
    fn equal_prefix<'this>(&self) -> Cow<'this, str>;
    /// The prefix to give lines that are being removed
    fn delete_prefix<'this>(&self) -> Cow<'this, str>;
    /// How to format bits of text that are being added
    fn insert_line<'this>(&self, input: &'this str) -> Cow<'this, str> {
        input.into()
    }
    /// The prefix to give lines that are being added
    fn insert_prefix<'this>(&self) -> Cow<'this, str>;
    /// If a diff line doesn't end with a newline, what should we insert
    fn line_end<'this>(&self) -> Cow<'this, str> {
        "\n".into()
    }

    /// If one of the two strings ends with a newline, and the other does not,
    /// insert this character before the newline, and then re-add the newline
    fn trailing_lf_marker<'this>(&self) -> Cow<'this, str> {
        "␊".into()
    }

    /// A header to put above the diff
    fn header<'this>(&self) -> Cow<'this, str>;
}

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
        "<".into()
    }

    fn insert_prefix<'this>(&self) -> Cow<'this, str> {
        ">".into()
    }

    fn header<'this>(&self) -> Cow<'this, str> {
        "< left / > right\n".into()
    }
}

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
///     "\u{1b}[38;5;9m< left\u{1b}[39m / \u{1b}[38;5;10m> right\u{1b}[39m
/// \u{1b}[38;5;9m<\u{1b}[39m\u{1b}[38;5;9mThe quick \u{1b}[39m\u{1b}[38;5;9m\u{1b}[4mbrown\u{1b}[0m\u{1b}[39m\u{1b}[38;5;9m fox and
/// \u{1b}[39m\u{1b}[38;5;9m<\u{1b}[39m\u{1b}[38;5;9mjumps over the \u{1b}[39m\u{1b}[38;5;9m\u{1b}[4msleepy\u{1b}[0m\u{1b}[39m\u{1b}[38;5;9m dog\u{1b}[39m
/// \u{1b}[38;5;10m>\u{1b}[39m\u{1b}[38;5;10mThe quick \u{1b}[39m\u{1b}[38;5;10m\u{1b}[4mred\u{1b}[0m\u{1b}[39m\u{1b}[38;5;10m fox and
/// \u{1b}[39m\u{1b}[38;5;10m>\u{1b}[39m\u{1b}[38;5;10mjumps over the \u{1b}[39m\u{1b}[38;5;10m\u{1b}[4mlazy\u{1b}[0m\u{1b}[39m\u{1b}[38;5;10m dog\u{1b}[39m
/// "
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

/// A simple colorful theme using signs
///
/// ```
/// use termdiff::{diff, SignsColorTheme};
/// let old = "The quick brown fox and\njumps over the sleepy dog";
/// let new = "The quick red fox and\njumps over the lazy dog";
/// let mut buffer: Vec<u8> = Vec::new();
/// let  theme = SignsColorTheme::default();
/// diff(&mut buffer, old, new, &theme).unwrap();
/// let actual: String = String::from_utf8(buffer).expect("Not valid UTF-8");
///
/// assert_eq!(
///     actual,
///     "\u{1b}[38;5;9m--- remove\u{1b}[39m | \u{1b}[38;5;10minsert +++\u{1b}[39m
/// \u{1b}[38;5;9m-\u{1b}[39m\u{1b}[38;5;9mThe quick \u{1b}[39m\u{1b}[38;5;9m\u{1b}[38;5;9m\u{1b}[4mbrown\u{1b}[0m\u{1b}[39m\u{1b}[38;5;9m fox and
/// \u{1b}[39m\u{1b}[38;5;9m-\u{1b}[39m\u{1b}[38;5;9mjumps over the \u{1b}[39m\u{1b}[38;5;9m\u{1b}[38;5;9m\u{1b}[4msleepy\u{1b}[0m\u{1b}[39m\u{1b}[38;5;9m dog\u{1b}[39m
/// \u{1b}[38;5;10m+\u{1b}[39m\u{1b}[38;5;10mThe quick \u{1b}[39m\u{1b}[38;5;10m\u{1b}[38;5;10m\u{1b}[4mred\u{1b}[0m\u{1b}[39m\u{1b}[38;5;10m fox and
/// \u{1b}[39m\u{1b}[38;5;10m+\u{1b}[39m\u{1b}[38;5;10mjumps over the \u{1b}[39m\u{1b}[38;5;10m\u{1b}[38;5;10m\u{1b}[4mlazy\u{1b}[0m\u{1b}[39m\u{1b}[38;5;10m dog\u{1b}[39m
/// "
/// );
/// ```
#[derive(Default, Clone, Copy, Debug)]
pub struct SignsColorTheme {}

impl Theme for SignsColorTheme {
    fn highlight_insert<'this>(&self, input: &'this str) -> Cow<'this, str> {
        input.underlined().green().to_string().into()
    }

    fn highlight_delete<'this>(&self, input: &'this str) -> Cow<'this, str> {
        input.underlined().red().to_string().into()
    }

    fn equal_content<'this>(&self, input: &'this str) -> Cow<'this, str> {
        input.into()
    }

    fn delete_content<'this>(&self, input: &'this str) -> Cow<'this, str> {
        input.red().to_string().into()
    }

    fn equal_prefix<'this>(&self) -> Cow<'this, str> {
        " ".into()
    }

    fn delete_prefix<'this>(&self) -> Cow<'this, str> {
        "-".red().to_string().into()
    }

    fn insert_line<'this>(&self, input: &'this str) -> Cow<'this, str> {
        input.green().to_string().into()
    }

    fn insert_prefix<'this>(&self) -> Cow<'this, str> {
        "+".green().to_string().into()
    }

    fn line_end<'this>(&self) -> Cow<'this, str> {
        "\n".into()
    }

    fn header<'this>(&self) -> Cow<'this, str> {
        format!("{} | {}\n", "--- remove".red(), "insert +++".green()).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signs_theme_equal_prefix() {
        let theme = SignsTheme::default();
        assert_eq!(theme.equal_prefix(), Cow::Borrowed(" "));
    }

    #[test]
    fn test_signs_color_theme_equal_prefix() {
        let theme = SignsColorTheme::default();
        assert_eq!(theme.equal_prefix(), Cow::Borrowed(" "));
    }

    #[test]
    fn test_arrows_theme_equal_prefix() {
        let theme = ArrowsTheme::default();
        assert_eq!(theme.equal_prefix(), Cow::Borrowed(" "));
    }

    #[test]
    fn test_arrows_color_theme_equal_prefix() {
        let theme = ArrowsColorTheme::default();
        assert_eq!(theme.equal_prefix(), Cow::Borrowed(" "));
    }
}
