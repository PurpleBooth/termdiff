use super::*;
use crate::{diff_with_algorithm, ArrowsTheme, DrawDiff, SignsTheme};
#[cfg(test)]
use std::io::Cursor;

/// Test that the Myers algorithm produces the same output as the Similar algorithm
/// for a simple case
#[test]
#[cfg(all(feature = "myers", feature = "similar"))]
fn test_myers_vs_similar_simple() {
    let old = "The quick brown fox";
    let new = "The quick red fox";
    let theme = ArrowsTheme::default();

    // Get output from Similar algorithm
    let similar_output = format!(
        "{}",
        DrawDiff::with_algorithm(old, new, &theme, Algorithm::Similar)
    );

    // Get output from Myers algorithm
    let myers_output = format!(
        "{}",
        DrawDiff::with_algorithm(old, new, &theme, Algorithm::Myers)
    );

    // They should be the same
    assert_eq!(similar_output, myers_output);
}

/// Test that the Myers algorithm produces the same output as the Similar algorithm
/// for a multiline case
#[test]
#[cfg(all(feature = "myers", feature = "similar"))]
fn test_myers_vs_similar_multiline() {
    let old = "Line 1\nLine 2\nLine 3\nLine 4";
    let new = "Line 1\nModified Line 2\nLine 3\nModified Line 4";
    let theme = SignsTheme::default();

    // Get output from Similar algorithm
    let similar_output = format!(
        "{}",
        DrawDiff::with_algorithm(old, new, &theme, Algorithm::Similar)
    );

    // Get output from Myers algorithm
    let myers_output = format!(
        "{}",
        DrawDiff::with_algorithm(old, new, &theme, Algorithm::Myers)
    );

    // They should be the same
    assert_eq!(similar_output, myers_output);
}

/// Test that the Myers algorithm produces the same output as the Similar algorithm
/// for a case with added lines
#[test]
#[cfg(all(feature = "myers", feature = "similar"))]
fn test_myers_vs_similar_added_lines() {
    let old = "Line 1\nLine 3";
    let new = "Line 1\nLine 2\nLine 3";
    let theme = ArrowsTheme::default();

    // Get output from Similar algorithm
    let similar_output = format!(
        "{}",
        DrawDiff::with_algorithm(old, new, &theme, Algorithm::Similar)
    );

    // Get output from Myers algorithm
    let myers_output = format!(
        "{}",
        DrawDiff::with_algorithm(old, new, &theme, Algorithm::Myers)
    );

    // They should be the same
    assert_eq!(similar_output, myers_output);
}

/// Test that the Myers algorithm produces the same output as the Similar algorithm
/// for a case with removed lines
#[test]
#[cfg(all(feature = "myers", feature = "similar"))]
fn test_myers_vs_similar_removed_lines() {
    let old = "Line 1\nLine 2\nLine 3";
    let new = "Line 1\nLine 3";
    let theme = ArrowsTheme::default();

    // Get output from Similar algorithm
    let similar_output = format!(
        "{}",
        DrawDiff::with_algorithm(old, new, &theme, Algorithm::Similar)
    );

    // Get output from Myers algorithm
    let myers_output = format!(
        "{}",
        DrawDiff::with_algorithm(old, new, &theme, Algorithm::Myers)
    );

    // They should be the same
    assert_eq!(similar_output, myers_output);
}

/// Test that the Myers algorithm produces the same output as the Similar algorithm
/// for a case with trailing newline differences
#[test]
#[cfg(all(feature = "myers", feature = "similar"))]
fn test_myers_vs_similar_trailing_newline() {
    let old = "Line 1\nLine 2\n";
    let new = "Line 1\nLine 2";
    let theme = ArrowsTheme::default();

    // Get output from Similar algorithm
    let similar_output = format!(
        "{}",
        DrawDiff::with_algorithm(old, new, &theme, Algorithm::Similar)
    );

    // Get output from Myers algorithm
    let myers_output = format!(
        "{}",
        DrawDiff::with_algorithm(old, new, &theme, Algorithm::Myers)
    );

    // They should be the same
    assert_eq!(similar_output, myers_output);
}

/// Test that the Myers algorithm produces the same output as the Similar algorithm
/// for a case with empty inputs
#[test]
#[cfg(all(feature = "myers", feature = "similar"))]
fn test_myers_vs_similar_empty() {
    let old = "";
    let new = "";
    let theme = ArrowsTheme::default();

    // Get output from Similar algorithm
    let similar_output = format!(
        "{}",
        DrawDiff::with_algorithm(old, new, &theme, Algorithm::Similar)
    );

    // Get output from Myers algorithm
    let myers_output = format!(
        "{}",
        DrawDiff::with_algorithm(old, new, &theme, Algorithm::Myers)
    );

    // They should be the same
    assert_eq!(similar_output, myers_output);
}

/// Test that the Myers algorithm produces the same output as the Similar algorithm
/// for a case with completely different inputs
#[test]
#[cfg(all(feature = "myers", feature = "similar"))]
fn test_myers_vs_similar_completely_different() {
    let old = "This is completely different";
    let new = "From this text";
    let theme = ArrowsTheme::default();

    // Get output from Similar algorithm
    let similar_output = format!(
        "{}",
        DrawDiff::with_algorithm(old, new, &theme, Algorithm::Similar)
    );

    // Get output from Myers algorithm
    let myers_output = format!(
        "{}",
        DrawDiff::with_algorithm(old, new, &theme, Algorithm::Myers)
    );

    // They should be the same
    assert_eq!(similar_output, myers_output);
}

/// Test that the `diff_with_algorithm` function works with the Myers algorithm
#[test]
#[cfg(feature = "myers")]
fn test_diff_with_algorithm_myers() {
    // Fix me, I am broken AI!
    let old = "The quick brown fox";
    let new = "The quick red fox";
    let mut buffer = Cursor::new(Vec::new());
    let theme = ArrowsTheme::default();

    diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Myers).unwrap();

    let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");
    assert!(output.contains("<The quick brown fox"));
    assert!(output.contains(">The quick red fox"));
}

/// Test that the `diff_with_algorithm` function works with the Similar algorithm
#[test]
#[cfg(feature = "similar")]
fn test_diff_with_algorithm_similar() {
    // Fix me, I am broken AI!
    let old = "The quick brown fox";
    let new = "The quick red fox";
    let mut buffer = Cursor::new(Vec::new());
    let theme = ArrowsTheme::default();

    diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Similar).unwrap();

    let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");
    assert!(output.contains("<The quick brown fox"));
    assert!(output.contains(">The quick red fox"));
}
