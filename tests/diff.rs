//! End-to-end tests for the diff rendering pipeline.
//!
//! These tests exercise the public API (`diff`, `diff_with_algorithm`,
//! `DrawDiff`) with various themes and input scenarios. They consolidate
//! what was previously spread across inline test modules in `cmd.rs`,
//! `draw_diff.rs`, and `lib.rs`.

use std::borrow::Cow;
use std::io::Write;

use termdiff::{diff, DrawDiff, Theme};

/// Render a diff through `DrawDiff` and return the output string.
fn render(old: &str, new: &str, theme: &dyn Theme) -> String {
    format!("{}", DrawDiff::new(old, new, theme))
}

// ---------------------------------------------------------------------------
// Exact-output tests with built-in themes
// ---------------------------------------------------------------------------

#[test]
#[cfg(feature = "arrows")]
fn arrows_theme_single_line_change() {
    let theme = termdiff::ArrowsTheme::default();
    assert_eq!(
        render("old", "new", &theme),
        "< left / > right\n<old\n>new\n",
    );
}

#[test]
#[cfg(feature = "arrows")]
fn arrows_theme_multiline_change() {
    let old = "The quick brown fox and\njumps over the sleepy dog";
    let new = "The quick red fox and\njumps over the lazy dog";
    let theme = termdiff::ArrowsTheme::default();
    assert_eq!(
        render(old, new, &theme),
        "\
< left / > right
<The quick brown fox and
<jumps over the sleepy dog
>The quick red fox and
>jumps over the lazy dog
",
    );
}

#[test]
#[cfg(feature = "signs")]
fn signs_theme_single_line_change() {
    let old = "The quick brown fox and\njumps over the sleepy dog";
    let new = "The quick red fox and\njumps over the lazy dog";
    let theme = termdiff::SignsTheme::default();
    assert_eq!(
        render(old, new, &theme),
        "\
--- remove | insert +++
-The quick brown fox and
-jumps over the sleepy dog
+The quick red fox and
+jumps over the lazy dog
",
    );
}

// ---------------------------------------------------------------------------
// diff() writer API
// ---------------------------------------------------------------------------

/// Verify `diff()` writes the same output as `DrawDiff` Display.
#[test]
#[cfg(feature = "arrows")]
fn diff_writes_to_writer() {
    let old = "The quick brown fox";
    let new = "The quick red fox";
    let theme = termdiff::ArrowsTheme::default();

    let mut buffer = Vec::new();
    diff(&mut buffer, old, new, &theme).unwrap();
    let output = String::from_utf8(buffer).expect("Not valid UTF-8");

    assert_eq!(output, render(old, new, &theme));
    assert!(output.contains("<The quick brown fox"));
    assert!(output.contains(">The quick red fox"));
    assert!(output.contains("< left / > right"));
}

/// A failing writer should propagate the `io::Error`.
#[test]
fn diff_writer_error() {
    struct ErrorWriter;

    impl Write for ErrorWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::other("write failed"))
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[derive(Debug)]
    struct StubTheme;

    impl Theme for StubTheme {
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
            "\n".into()
        }
    }

    let result = diff(&mut ErrorWriter, "old", "new", &StubTheme);
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// diff_with_algorithm
// ---------------------------------------------------------------------------

#[test]
#[cfg(all(feature = "arrows", feature = "myers"))]
fn diff_with_algorithm_myers() {
    use termdiff::{diff_with_algorithm, Algorithm};

    let old = "The quick brown fox";
    let new = "The quick red fox";
    let theme = termdiff::ArrowsTheme::default();

    let mut buffer = Vec::new();
    diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Myers).unwrap();
    let output = String::from_utf8(buffer).expect("Not valid UTF-8");

    assert!(output.contains("<The quick brown fox"));
    assert!(output.contains(">The quick red fox"));
}

#[test]
#[cfg(all(feature = "arrows", feature = "similar"))]
fn diff_with_algorithm_similar() {
    use termdiff::{diff_with_algorithm, Algorithm};

    let old = "The quick brown fox";
    let new = "The quick red fox";
    let theme = termdiff::ArrowsTheme::default();

    let mut buffer = Vec::new();
    diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Similar).unwrap();
    let output = String::from_utf8(buffer).expect("Not valid UTF-8");

    assert!(output.contains("<The quick brown fox"));
    assert!(output.contains(">The quick red fox"));
}

// ---------------------------------------------------------------------------
// DrawDiff API
// ---------------------------------------------------------------------------

#[test]
#[cfg(feature = "arrows")]
fn draw_diff_to_string() {
    let theme = termdiff::ArrowsTheme::default();
    let output: String = DrawDiff::new("old", "new", &theme).into();
    assert!(output.contains("<old"));
    assert!(output.contains(">new"));
}

// ---------------------------------------------------------------------------
// Common diff scenarios
// ---------------------------------------------------------------------------

#[test]
#[cfg(feature = "arrows")]
fn empty_inputs_produce_header_only() {
    let theme = termdiff::ArrowsTheme::default();
    assert_eq!(render("", "", &theme), "< left / > right\n");
}

#[test]
#[cfg(feature = "arrows")]
fn identical_inputs_show_no_changes() {
    let theme = termdiff::ArrowsTheme::default();
    let output = render("same text", "same text", &theme);
    assert!(output.contains(" same text"));
    assert!(!output.contains("<same text"));
    assert!(!output.contains(">same text"));
}

#[test]
#[cfg(feature = "arrows")]
fn multiline_preserves_unchanged_lines() {
    let old = "line 1\nline 2\nline 3\nline 4";
    let new = "line 1\nmodified line 2\nline 3\nmodified line 4";
    let theme = termdiff::ArrowsTheme::default();
    let output = render(old, new, &theme);

    assert!(output.contains(" line 1\n"));
    assert!(output.contains("<line 2\n"));
    assert!(output.contains(">modified line 2\n"));
    assert!(output.contains(" line 3\n"));
    assert!(output.contains("<line 4"));
    assert!(output.contains(">modified line 4"));
}

#[test]
#[cfg(feature = "arrows")]
fn added_lines() {
    let old = "line 1\nline 3";
    let new = "line 1\nline 2\nline 3";
    let theme = termdiff::ArrowsTheme::default();
    let output = render(old, new, &theme);

    assert!(output.contains(" line 1\n"));
    assert!(output.contains(">line 2\n"));
    assert!(output.contains(" line 3"));
}

#[test]
#[cfg(feature = "arrows")]
fn removed_lines() {
    let old = "line 1\nline 2\nline 3";
    let new = "line 1\nline 3";
    let theme = termdiff::ArrowsTheme::default();
    let output = render(old, new, &theme);

    assert!(output.contains(" line 1"));
    assert!(output.contains("<line 2"));
    assert!(output.contains(" line 3"));
}

#[test]
#[cfg(feature = "arrows")]
fn large_input_stress() {
    let old = "a\n".repeat(1000);
    let new = "a\n".repeat(500) + &"b\n".repeat(500);
    let theme = termdiff::ArrowsTheme::default();
    let output = render(&old, &new, &theme);

    // Header + 500 equal 'a' lines + 500 deleted 'a' lines + 500 inserted 'b' lines
    assert_eq!(output.lines().count(), 1 + 500 + 500 + 500);
    assert!(output.contains(" a"));
    assert!(output.contains("<a"));
    assert!(output.contains(">b"));
}

// ---------------------------------------------------------------------------
// Custom themes
// ---------------------------------------------------------------------------

#[test]
fn custom_theme_minimal() {
    #[derive(Debug)]
    struct MinimalTheme;

    impl Theme for MinimalTheme {
        fn equal_prefix<'a>(&self) -> Cow<'a, str> {
            "=".into()
        }
        fn delete_prefix<'a>(&self) -> Cow<'a, str> {
            "!".into()
        }
        fn insert_prefix<'a>(&self) -> Cow<'a, str> {
            "|".into()
        }
        fn header<'a>(&self) -> Cow<'a, str> {
            "CUSTOM\n".into()
        }
    }

    let output = render("old", "new", &MinimalTheme);
    assert!(output.contains("CUSTOM"));
    assert!(output.contains("!old"));
    assert!(output.contains("|new"));
}

#[test]
fn custom_theme_overrides_all_methods() {
    #[derive(Debug)]
    struct FullTheme;

    impl Theme for FullTheme {
        fn equal_prefix<'a>(&self) -> Cow<'a, str> {
            "=".into()
        }
        fn delete_prefix<'a>(&self) -> Cow<'a, str> {
            "-".into()
        }
        fn insert_prefix<'a>(&self) -> Cow<'a, str> {
            "+".into()
        }
        fn header<'a>(&self) -> Cow<'a, str> {
            "HEADER\n".into()
        }
        fn highlight_insert<'a>(&self, input: &'a str) -> Cow<'a, str> {
            format!("*{input}*").into()
        }
        fn highlight_delete<'a>(&self, input: &'a str) -> Cow<'a, str> {
            format!("~{input}~").into()
        }
        fn equal_content<'a>(&self, input: &'a str) -> Cow<'a, str> {
            format!("={input}").into()
        }
        fn delete_content<'a>(&self, input: &'a str) -> Cow<'a, str> {
            format!("-{input}").into()
        }
        fn insert_line<'a>(&self, input: &'a str) -> Cow<'a, str> {
            format!("+{input}").into()
        }
        fn line_end<'a>(&self) -> Cow<'a, str> {
            "\r\n".into()
        }
        fn trailing_lf_marker<'a>(&self) -> Cow<'a, str> {
            "[LF]".into()
        }
    }

    let theme = FullTheme;
    assert_eq!(theme.equal_prefix(), "=");
    assert_eq!(theme.delete_prefix(), "-");
    assert_eq!(theme.insert_prefix(), "+");
    assert_eq!(theme.header(), "HEADER\n");

    let input = "test";
    assert_eq!(theme.highlight_insert(input), "*test*");
    assert_eq!(theme.highlight_delete(input), "~test~");
    assert_eq!(theme.equal_content(input), "=test");
    assert_eq!(theme.delete_content(input), "-test");
    assert_eq!(theme.insert_line(input), "+test");
    assert_eq!(theme.line_end(), "\r\n");
    assert_eq!(theme.trailing_lf_marker(), "[LF]");
}

// ---------------------------------------------------------------------------
// Color themes (feature-gated)
// ---------------------------------------------------------------------------

#[test]
#[cfg(feature = "arrows_color")]
fn arrows_color_theme_renders() {
    let old = "The quick brown fox and\njumps over the sleepy dog";
    let new = "The quick red fox and\njumps over the lazy dog";
    let theme = termdiff::ArrowsColorTheme::default();
    let output = render(old, new, &theme);

    assert!(output.contains("The quick brown fox and"));
    assert!(output.contains("The quick red fox and"));
    // Colored output should contain ANSI escape codes
    assert!(output.contains('\u{1b}'));
}

#[test]
#[cfg(feature = "signs_color")]
fn signs_color_theme_renders() {
    let old = "The quick brown fox and\njumps over the sleepy dog";
    let new = "The quick red fox and\njumps over the lazy dog";
    let theme = termdiff::SignsColorTheme::default();
    let output = render(old, new, &theme);

    assert!(output.contains("The quick brown fox and"));
    assert!(output.contains("The quick red fox and"));
    assert!(output.contains('\u{1b}'));
}
