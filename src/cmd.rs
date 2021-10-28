use std::io::Write;

use super::{draw_diff::DrawDiff, themes::Theme};

/// Print a diff to a writer
///
/// # Examples
///
///  Black and white output
///
/// ```
/// use termdiff::{arrows_theme, diff};
/// let old = "a\nb\nc";
/// let new = "a\nc\n";
/// let mut buffer: Vec<u8> = Vec::new();
/// let theme = arrows_theme();
/// diff(&mut buffer, old, new, &theme).unwrap();
/// let actual: String = String::from_utf8(buffer).expect("Not valid UTF-8");
///
/// assert_eq!(
///     actual,
///     "< left / > right
///  a
/// <b
/// <c
/// >c
/// "
/// );
/// ```
///  
/// Colorful theme
///
/// ```
/// use termdiff::{arrows_color_theme, diff};
/// let old = "a\nb\nc";
/// let new = "a\nc\n";
/// let mut buffer: Vec<u8> = Vec::new();
/// let theme = arrows_color_theme();
/// diff(&mut buffer, old, new, &theme).unwrap();
/// let actual: String = String::from_utf8(buffer).expect("Not valid UTF-8");
///
/// assert_eq!(
///     actual,
///     "\u{1b}[38;5;9m< left\u{1b}[39m / \u{1b}[38;5;10m> right\u{1b}[39m
///  a
/// \u{1b}[38;5;9m<\u{1b}[39m\u{1b}[38;5;9mb
/// \u{1b}[39m\u{1b}[38;5;9m<\u{1b}[39m\u{1b}[38;5;9mc\u{1b}[39m
/// \u{1b}[38;5;10m>\u{1b}[39m\u{1b}[38;5;10mc
/// \u{1b}[39m"
/// );
/// ```
///
/// # Errors
///
/// Errors on failing to write to the writer.
pub fn diff(w: &mut dyn Write, old: &str, new: &str, theme: &Theme) -> std::io::Result<()> {
    let output: DrawDiff = DrawDiff::new(old, new, theme);
    write!(w, "{}", output)
}

#[cfg(test)]
mod tests {
    use super::super::{arrows_color_theme, arrows_theme};

    #[test]
    fn single_characters() {
        let old = "a\nb\nc";
        let new = "a\nc\n";
        let mut buffer: Vec<u8> = Vec::new();
        super::diff(&mut buffer, old, new, &arrows_theme()).unwrap();
        let actual: String = String::from_utf8(buffer).expect("Not valid UTF-8");

        assert_eq!(
            actual,
            "< left / > right
 a
<b
<c
>c
"
        );
    }

    #[test]
    fn color_single_characters() {
        let old = "a\nb\nc";
        let new = "a\nc\n";
        let mut buffer: Vec<u8> = Vec::new();
        super::diff(&mut buffer, old, new, &arrows_color_theme()).unwrap();
        let actual: String = String::from_utf8(buffer).expect("Not valid UTF-8");
        assert_eq!(
            actual,
            "\u{1b}[38;5;9m< left\u{1b}[39m / \u{1b}[38;5;10m> right\u{1b}[39m
 a
\u{1b}[38;5;9m<\u{1b}[39m\u{1b}[38;5;9mb
\u{1b}[39m\u{1b}[38;5;9m<\u{1b}[39m\u{1b}[38;5;9mc\u{1b}[39m
\u{1b}[38;5;10m>\u{1b}[39m\u{1b}[38;5;10mc
\u{1b}[39m"
        );
    }
}
