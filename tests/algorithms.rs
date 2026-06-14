//! Tests for algorithm selection and feature-gated availability.
//!
//! When both `myers` and `similar` features are enabled, the parity tests
//! verify they produce identical output. When only one is enabled, the
//! availability tests confirm the other gracefully falls back.

use termdiff::{Algorithm, DrawDiff};

// ---------------------------------------------------------------------------
// Myers vs Similar output parity (requires both features)
// ---------------------------------------------------------------------------

/// Helper: render a diff with a specific algorithm.
#[cfg(all(feature = "myers", feature = "similar"))]
fn render_with(old: &str, new: &str, algorithm: Algorithm) -> String {
    use termdiff::ArrowsTheme;
    format!(
        "{}",
        DrawDiff::with_algorithm(old, new, &ArrowsTheme::default(), algorithm)
    )
}

#[test]
#[cfg(all(feature = "myers", feature = "similar"))]
fn parity_simple_change() {
    let old = "The quick brown fox";
    let new = "The quick red fox";
    assert_eq!(
        render_with(old, new, Algorithm::Similar),
        render_with(old, new, Algorithm::Myers),
    );
}

#[test]
#[cfg(all(feature = "myers", feature = "similar"))]
fn parity_multiline_interleaved() {
    let old = "Line 1\nLine 2\nLine 3\nLine 4";
    let new = "Line 1\nModified Line 2\nLine 3\nModified Line 4";
    assert_eq!(
        render_with(old, new, Algorithm::Similar),
        render_with(old, new, Algorithm::Myers),
    );
}

#[test]
#[cfg(all(feature = "myers", feature = "similar"))]
fn parity_added_lines() {
    let old = "Line 1\nLine 3";
    let new = "Line 1\nLine 2\nLine 3";
    assert_eq!(
        render_with(old, new, Algorithm::Similar),
        render_with(old, new, Algorithm::Myers),
    );
}

#[test]
#[cfg(all(feature = "myers", feature = "similar"))]
fn parity_removed_lines() {
    let old = "Line 1\nLine 2\nLine 3";
    let new = "Line 1\nLine 3";
    assert_eq!(
        render_with(old, new, Algorithm::Similar),
        render_with(old, new, Algorithm::Myers),
    );
}

#[test]
#[cfg(all(feature = "myers", feature = "similar"))]
fn parity_trailing_newline() {
    let old = "Line 1\nLine 2\n";
    let new = "Line 1\nLine 2";
    assert_eq!(
        render_with(old, new, Algorithm::Similar),
        render_with(old, new, Algorithm::Myers),
    );
}

#[test]
#[cfg(all(feature = "myers", feature = "similar"))]
fn parity_empty_inputs() {
    assert_eq!(
        render_with("", "", Algorithm::Similar),
        render_with("", "", Algorithm::Myers),
    );
}

#[test]
#[cfg(all(feature = "myers", feature = "similar"))]
fn parity_completely_different() {
    let old = "This is completely different";
    let new = "From this text";
    assert_eq!(
        render_with(old, new, Algorithm::Similar),
        render_with(old, new, Algorithm::Myers),
    );
}

// ---------------------------------------------------------------------------
// Feature availability — only one algorithm enabled
// ---------------------------------------------------------------------------

#[test]
#[cfg(all(feature = "myers", not(feature = "similar")))]
fn only_myers_available() {
    use termdiff::ArrowsTheme;
    let old = "The quick brown fox";
    let new = "The quick red fox";
    let theme = ArrowsTheme::default();

    // Myers should work directly
    let mut buf = Cursor::new(Vec::new());
    diff_with_algorithm(&mut buf, old, new, &theme, Algorithm::Myers).unwrap();
    let output = String::from_utf8(buf.into_inner()).expect("Not valid UTF-8");
    assert!(output.contains("<The quick brown fox"));
    assert!(output.contains(">The quick red fox"));

    // Similar should fall back to an available algorithm
    let mut buf = Cursor::new(Vec::new());
    diff_with_algorithm(&mut buf, old, new, &theme, Algorithm::Similar).unwrap();
    let output = String::from_utf8(buf.into_inner()).expect("Not valid UTF-8");
    assert!(output.contains("<The quick brown fox"));
    assert!(output.contains(">The quick red fox"));
}

#[test]
#[cfg(all(feature = "similar", not(feature = "myers")))]
fn only_similar_available() {
    use termdiff::ArrowsTheme;
    let old = "The quick brown fox";
    let new = "The quick red fox";
    let theme = ArrowsTheme::default();

    // Similar should work directly
    let mut buf = Cursor::new(Vec::new());
    diff_with_algorithm(&mut buf, old, new, &theme, Algorithm::Similar).unwrap();
    let output = String::from_utf8(buf.into_inner()).expect("Not valid UTF-8");
    assert!(output.contains("<The quick brown fox"));
    assert!(output.contains(">The quick red fox"));

    // Myers should fall back to an available algorithm
    let mut buf = Cursor::new(Vec::new());
    diff_with_algorithm(&mut buf, old, new, &theme, Algorithm::Myers).unwrap();
    let output = String::from_utf8(buf.into_inner()).expect("Not valid UTF-8");
    assert!(output.contains("<The quick brown fox"));
    assert!(output.contains(">The quick red fox"));
}

// ---------------------------------------------------------------------------
// No algorithms available
// ---------------------------------------------------------------------------

#[test]
#[cfg(not(any(feature = "myers", feature = "similar")))]
fn no_algorithms_shows_error() {
    use termdiff::ArrowsTheme;
    let theme = ArrowsTheme::default();

    // diff() should produce the error message
    let mut buf = Cursor::new(Vec::new());
    diff(&mut buf, "old", "new", &theme).unwrap();
    let output = String::from_utf8(buf.into_inner()).expect("Not valid UTF-8");
    assert!(output.contains("Error: No diff algorithms are available"));

    // diff_with_algorithm should also produce the error
    let mut buf = Cursor::new(Vec::new());
    diff_with_algorithm(&mut buf, "old", "new", &theme, Algorithm::Myers).unwrap();
    let output = String::from_utf8(buf.into_inner()).expect("Not valid UTF-8");
    assert!(output.contains("Error: No diff algorithms are available"));
}
