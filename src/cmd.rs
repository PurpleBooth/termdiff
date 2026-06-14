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
///     "< left / > right\n a\n<b\n<c\n>c␊\n"
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
///     "< left / > right\n a\n<b\n<c\n>c␊\n"
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
