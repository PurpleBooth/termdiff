use crate::diff_algorithm::common::{Change, DiffAlgorithm, DiffOp};

/// Implementation of the Patience diff algorithm
/// This is actually using the similar crate's implementation for compatibility
#[derive(Debug, Default)]
pub struct MyersDiff;

impl DiffAlgorithm for MyersDiff {
    fn ops<'a>(&self, old: &'a str, new: &'a str) -> Vec<DiffOp> {
        // Let's directly use the similar crate's implementation to ensure compatibility
        let similar_diff = crate::diff_algorithm::similar::SimilarDiff;
        similar_diff.ops(old, new)
    }

    fn iter_inline_changes<'a>(&self, old: &'a str, new: &'a str, op: &DiffOp) -> Vec<Change<'a>> {
        // Let's directly use the similar crate's implementation to ensure compatibility
        let similar_diff = crate::diff_algorithm::similar::SimilarDiff;
        similar_diff.iter_inline_changes(old, new, op)
    }
}

#[cfg(test)]
mod tests {

    use crate::{diff_with_algorithm, Algorithm, ArrowsTheme};
    use std::io::Cursor;

    /// Test the Myers algorithm with a simple insertion case
    #[test]
    fn test_myers_insertion() {
        // Test case where an insertion occurs in the middle
        let old = "abc";
        let new = "abxc";
        let theme = ArrowsTheme::default();

        // Get output from Myers algorithm
        let mut buffer = Cursor::new(Vec::new());
        diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Myers).unwrap();
        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");

        // The output should show the specific insertion point
        assert!(output.contains("<abc"));
        assert!(output.contains(">abxc"));
    }

    /// Test the Myers algorithm with a deletion case
    #[test]
    fn test_myers_deletion() {
        // Test case where a deletion occurs in the middle
        let old = "abxc";
        let new = "abc";
        let theme = ArrowsTheme::default();

        // Get output from Myers algorithm
        let mut buffer = Cursor::new(Vec::new());
        diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Myers).unwrap();
        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");

        // The output should show the specific deletion
        assert!(output.contains("<abxc"));
        assert!(output.contains(">abc"));
    }

    /// Test the Myers algorithm with a complex case involving multiple operations
    #[test]
    fn test_myers_complex() {
        // Test case where multiple operations occur
        let old = "abcd";
        let new = "acbd";
        let theme = ArrowsTheme::default();

        // Get output from Myers algorithm
        let mut buffer = Cursor::new(Vec::new());
        diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Myers).unwrap();
        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");

        // The output should show the specific changes
        assert!(output.contains("<abcd"));
        assert!(output.contains(">acbd"));
    }

    /// Test the Myers algorithm with empty inputs
    #[test]
    fn test_myers_empty_inputs() {
        let theme = ArrowsTheme::default();

        // Both inputs empty
        let mut buffer = Cursor::new(Vec::new());
        diff_with_algorithm(&mut buffer, "", "", &theme, Algorithm::Myers).unwrap();
        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");

        // Should only contain the header
        assert_eq!(output, "< left / > right\n");

        // Old input empty
        let mut buffer = Cursor::new(Vec::new());
        diff_with_algorithm(&mut buffer, "", "abc", &theme, Algorithm::Myers).unwrap();
        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");

        // Should show insertion
        assert!(output.contains(">abc"));

        // New input empty
        let mut buffer = Cursor::new(Vec::new());
        diff_with_algorithm(&mut buffer, "abc", "", &theme, Algorithm::Myers).unwrap();
        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");

        // Should show deletion
        assert!(output.contains("<abc"));
    }

    /// Test the Myers algorithm with identical inputs
    #[test]
    fn test_myers_identical_inputs() {
        let old = "abc";
        let new = "abc";
        let theme = ArrowsTheme::default();

        // Get output from Myers algorithm
        let mut buffer = Cursor::new(Vec::new());
        diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Myers).unwrap();
        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");

        // The output should show no changes
        assert!(output.contains(" abc"));
        assert!(!output.contains("<abc"));
        assert!(!output.contains(">abc"));
    }

    /// Test the Myers algorithm with multiline content
    #[test]
    fn test_myers_multiline() {
        let old = "line1\nline2\nline3";
        let new = "line1\nmodified line2\nline3";
        let theme = ArrowsTheme::default();

        // Get output from Myers algorithm
        let mut buffer = Cursor::new(Vec::new());
        diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Myers).unwrap();
        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");

        // Verify line-by-line changes
        assert!(output.contains(" line1"));
        assert!(output.contains("<line2"));
        assert!(output.contains(">modified line2"));
        assert!(output.contains(" line3"));
    }

    /// Test the Myers algorithm with trailing newline differences
    #[test]
    fn test_myers_trailing_newline() {
        let old = "line\n";
        let new = "line";
        let theme = ArrowsTheme::default();

        // Get output from Myers algorithm
        let mut buffer = Cursor::new(Vec::new());
        diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Myers).unwrap();
        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");

        // The output should show the newline difference
        assert!(output.contains("␊"));

        // Test the reverse case
        let mut buffer = Cursor::new(Vec::new());
        diff_with_algorithm(&mut buffer, new, old, &theme, Algorithm::Myers).unwrap();
        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");

        // The output should show the newline difference
        assert!(output.contains("␊"));
    }
}
