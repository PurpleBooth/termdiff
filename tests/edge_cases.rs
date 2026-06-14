//! Edge-case tests for unusual inputs and boundary conditions.
//!
//! Covers trailing newlines, CRLF, Unicode, empty/identical inputs, and
//! single-character diffs.

use std::borrow::Cow;

use termdiff::{DrawDiff, Theme};

fn render(old: &str, new: &str, theme: &dyn Theme) -> String {
    format!("{}", DrawDiff::new(old, new, theme))
}

// ---------------------------------------------------------------------------
// Trailing newline differences
// ---------------------------------------------------------------------------

#[test]
#[cfg(feature = "arrows")]
fn trailing_nl_old_has_newline_new_does_not() {
    let theme = termdiff::ArrowsTheme::default();
    let output = render("line\n", "line", &theme);
    assert!(
        output.contains('␊'),
        "Expected trailing LF marker, got:\\n{}",
        output
    );
}

#[test]
#[cfg(feature = "arrows")]
fn trailing_nl_new_has_newline_old_does_not() {
    let theme = termdiff::ArrowsTheme::default();
    let output = render("line", "line\n", &theme);
    assert!(
        output.contains('␊'),
        "Expected trailing LF marker, got:\\n{}",
        output
    );
}

#[test]
#[cfg(feature = "arrows")]
fn trailing_nl_on_completely_different_lines() {
    let theme = termdiff::ArrowsTheme::default();
    let output = render("hello\n", "world", &theme);

    assert!(output.contains('␊'));
    assert!(output.contains("<hello"));
    assert!(output.contains(">world"));
}

#[test]
#[cfg(feature = "arrows")]
fn trailing_nl_multiline_only_difference() {
    let old = "line1\nline2\nline3\n";
    let new = "line1\nline2\nline3";
    let theme = termdiff::ArrowsTheme::default();
    let output = render(old, new, &theme);

    // The first two lines should be unchanged
    assert!(output.contains(" line1\n"));
    assert!(output.contains(" line2\n"));
    // The trailing newline difference should be shown with the marker
    assert!(output.contains('␊'));
}

#[test]
#[cfg(feature = "arrows")]
fn trailing_nl_on_different_last_lines() {
    let theme = termdiff::ArrowsTheme::default();
    let output = render("apple\norange\n", "apple\nbanana", &theme);

    assert!(
        output.contains('␊'),
        "Expected trailing LF marker when old has trailing newline and new does not"
    );
}

/// Verify the exact output when the only difference is a trailing newline
/// on a single-line input.
#[test]
#[cfg(feature = "arrows")]
fn trailing_nl_exact_output() {
    let theme = termdiff::ArrowsTheme::default();
    let output = render("a\nb\nc", "a\nc\n", &theme);

    assert_eq!(
        output,
        "\
< left / > right
 a
<b
<c
>c␊
",
    );
}

#[test]
fn custom_trailing_lf_marker() {
    #[derive(Debug)]
    struct CustomMarkerTheme;

    impl Theme for CustomMarkerTheme {
        fn equal_prefix<'a>(&self) -> Cow<'a, str> {
            " ".into()
        }
        fn delete_prefix<'a>(&self) -> Cow<'a, str> {
            "<".into()
        }
        fn insert_prefix<'a>(&self) -> Cow<'a, str> {
            ">".into()
        }
        fn header<'a>(&self) -> Cow<'a, str> {
            "HEADER\n".into()
        }
        fn trailing_lf_marker<'a>(&self) -> Cow<'a, str> {
            "[NEWLINE]".into()
        }
    }

    let output = render("line\n", "line", &CustomMarkerTheme);
    assert!(output.contains("line[NEWLINE]"));
}

// ---------------------------------------------------------------------------
// CRLF
// ---------------------------------------------------------------------------

#[test]
#[cfg(feature = "arrows")]
fn crlf_produces_visible_carriage_return() {
    let theme = termdiff::ArrowsTheme::default();
    let output = render("line\r\n", "line", &theme);

    // The library treats \n as the line separator, leaving \r as line content
    assert!(
        output.contains('\r'),
        "Expected carriage return in diff output for CRLF input"
    );
}

// ---------------------------------------------------------------------------
// Unicode
// ---------------------------------------------------------------------------

#[test]
#[cfg(feature = "arrows")]
fn unicode_handled_correctly() {
    let theme = termdiff::ArrowsTheme::default();
    let output = render("héllo\nwörld", "héllo\nwörld!", &theme);

    assert!(output.contains(" héllo\n"));
    assert!(output.contains("<wörld"));
    assert!(output.contains(">wörld!"));
}

// ---------------------------------------------------------------------------
// Minimal inputs
// ---------------------------------------------------------------------------

#[test]
#[cfg(feature = "arrows")]
fn single_char_replacement() {
    let theme = termdiff::ArrowsTheme::default();
    let output = render("a", "b", &theme);

    assert!(output.contains("<a"));
    assert!(output.contains(">b"));
}

#[test]
#[cfg(feature = "arrows")]
fn whitespace_only_difference() {
    let theme = termdiff::ArrowsTheme::default();
    let output = render("text with spaces", "text  with  spaces", &theme);

    assert!(output.contains("<text with spaces"));
    assert!(output.contains(">text  with  spaces"));
}
