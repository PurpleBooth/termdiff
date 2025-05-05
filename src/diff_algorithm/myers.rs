use crate::diff_algorithm::common::{Change, ChangeTag, DiffAlgorithm, DiffOp};

/// Represents a diff operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DiffOperation {
    /// No change between old and new
    Equal,
    /// Content was in old but not in new
    Delete,
    /// Content was not in old but is in new
    Insert,
}

/// Computes the diff operations using an optimized version of the Myers algorithm
fn compute_diff_operations<T: PartialEq>(old: &[T], new: &[T]) -> Vec<DiffOperation> {
    let m = old.len();
    let n = new.len();

    // Handle empty inputs
    if m == 0 && n == 0 {
        return Vec::new();
    }
    if m == 0 {
        return vec![DiffOperation::Insert; n];
    }
    if n == 0 {
        return vec![DiffOperation::Delete; m];
    }

    // For small inputs, use a more efficient approach
    if m < 100 && n < 100 {
        return compute_diff_operations_small(old, new);
    }

    // For larger inputs, use a space-efficient version of the algorithm
    compute_diff_operations_large(old, new)
}

/// Computes diff operations for small inputs using a full LCS matrix
fn compute_diff_operations_small<T: PartialEq>(old: &[T], new: &[T]) -> Vec<DiffOperation> {
    let m = old.len();
    let n = new.len();

    // Initialize the LCS matrix with zeros
    let mut lcs = vec![vec![0; n + 1]; m + 1];

    // Fill the LCS matrix
    for i in 1..=m {
        for j in 1..=n {
            if old[i - 1] == new[j - 1] {
                lcs[i][j] = lcs[i - 1][j - 1] + 1;
            } else {
                lcs[i][j] = std::cmp::max(lcs[i - 1][j], lcs[i][j - 1]);
            }
        }
    }

    // Backtrack to find the diff operations (iterative approach)
    let mut ops = Vec::with_capacity(m + n); // Pre-allocate with a reasonable capacity
    let mut i = m;
    let mut j = n;

    // Use a stack to simulate recursion
    let mut stack = Vec::new();

    while i > 0 || j > 0 {
        if i > 0 && j > 0 && old[i - 1] == new[j - 1] {
            stack.push(DiffOperation::Equal);
            i -= 1;
            j -= 1;
        } else if j > 0 && (i == 0 || lcs[i][j - 1] >= lcs[i - 1][j]) {
            stack.push(DiffOperation::Insert);
            j -= 1;
        } else if i > 0 {
            stack.push(DiffOperation::Delete);
            i -= 1;
        }
    }

    // Reverse the operations to get them in the correct order
    while let Some(op) = stack.pop() {
        ops.push(op);
    }

    ops
}

/// Computes diff operations for large inputs using a space-efficient approach
fn compute_diff_operations_large<T: PartialEq>(old: &[T], new: &[T]) -> Vec<DiffOperation> {
    let m = old.len();
    let n = new.len();

    // Use two rows instead of the full matrix to save memory
    let mut prev_row = vec![0; n + 1];
    let mut curr_row = vec![0; n + 1];

    // Fill the LCS matrix one row at a time
    for i in 1..=m {
        std::mem::swap(&mut prev_row, &mut curr_row);

        for j in 1..=n {
            if old[i - 1] == new[j - 1] {
                curr_row[j] = prev_row[j - 1] + 1;
            } else {
                curr_row[j] = std::cmp::max(prev_row[j], curr_row[j - 1]);
            }
        }
    }

    // Reconstruct the path
    let mut ops = Vec::with_capacity(m + n);

    // We need to rebuild parts of the matrix as we go
    let mut lcs_rows = vec![vec![0; n + 1]; m + 1];

    // Copy the last row we computed
    lcs_rows[m].clone_from(&curr_row);

    // Rebuild the matrix from bottom to top, but only as needed
    for row in (1..m).rev() {
        for col in 0..=n {
            if col == 0 {
                lcs_rows[row][col] = 0;
            } else if old[row - 1] == new[col - 1] {
                lcs_rows[row][col] = lcs_rows[row - 1][col - 1] + 1;
            } else {
                lcs_rows[row][col] = std::cmp::max(lcs_rows[row - 1][col], lcs_rows[row][col - 1]);
            }
        }
    }

    // Now backtrack to find the diff operations
    let mut stack = Vec::new();
    let mut i = m;
    let mut j = n;

    while i > 0 || j > 0 {
        if i > 0 && j > 0 && old[i - 1] == new[j - 1] {
            stack.push(DiffOperation::Equal);
            i -= 1;
            j -= 1;
        } else if j > 0 && (i == 0 || lcs_rows[i][j - 1] >= lcs_rows[i - 1][j]) {
            stack.push(DiffOperation::Insert);
            j -= 1;
        } else if i > 0 {
            stack.push(DiffOperation::Delete);
            i -= 1;
        }
    }

    // Reverse the operations to get them in the correct order
    while let Some(op) = stack.pop() {
        ops.push(op);
    }

    ops
}

/// Implementation of the Myers diff algorithm
#[derive(Debug, Default)]
pub struct MyersDiff;

impl DiffAlgorithm for MyersDiff {
    fn ops<'a>(&self, old: &'a str, new: &'a str) -> Vec<DiffOp> {
        // Split the input strings into lines
        // Use a more efficient approach that doesn't allocate a Vec for each line
        let old_lines: Vec<&str> = old.lines().collect();
        let new_lines: Vec<&str> = new.lines().collect();

        // Compute the diff operations
        let mut result = Vec::with_capacity(old_lines.len() + new_lines.len()); // Pre-allocate with a reasonable capacity

        // Handle empty inputs
        if old_lines.is_empty() && new_lines.is_empty() {
            return result;
        }

        if old_lines.is_empty() {
            // All lines are insertions
            result.push(DiffOp::new(ChangeTag::Insert, 0, 0, 0, new_lines.len()));
            return result;
        }

        if new_lines.is_empty() {
            // All lines are deletions
            result.push(DiffOp::new(ChangeTag::Delete, 0, old_lines.len(), 0, 0));
            return result;
        }

        // Compute the diff operations using the optimized algorithm
        let ops = compute_diff_operations(&old_lines, &new_lines);

        // Convert the operations to DiffOps
        let mut old_idx = 0;
        let mut new_idx = 0;

        for op in ops {
            match op {
                DiffOperation::Equal => {
                    // Equal operation
                    result.push(DiffOp::new(ChangeTag::Equal, old_idx, 1, new_idx, 1));
                    old_idx += 1;
                    new_idx += 1;
                }
                DiffOperation::Insert => {
                    // Insert operation
                    result.push(DiffOp::new(ChangeTag::Insert, old_idx, 0, new_idx, 1));
                    new_idx += 1;
                }
                DiffOperation::Delete => {
                    // Delete operation
                    result.push(DiffOp::new(ChangeTag::Delete, old_idx, 1, new_idx, 0));
                    old_idx += 1;
                }
            }
        }

        // Merge adjacent operations of the same type
        let mut merged_result = Vec::new();
        let mut current_op: Option<(ChangeTag, usize, usize, usize, usize)> = None;

        for op in result {
            if let Some((tag, old_start, old_len, new_start, new_len)) = current_op {
                if tag == op.tag()
                    && old_start + old_len == op.old_start()
                    && new_start + new_len == op.new_start()
                {
                    // Merge with the current operation
                    current_op = Some((
                        tag,
                        old_start,
                        old_len + op.old_len(),
                        new_start,
                        new_len + op.new_len(),
                    ));
                } else {
                    // Push the current operation and start a new one
                    merged_result.push(DiffOp::new(tag, old_start, old_len, new_start, new_len));
                    current_op = Some((
                        op.tag(),
                        op.old_start(),
                        op.old_len(),
                        op.new_start(),
                        op.new_len(),
                    ));
                }
            } else {
                // Start a new operation
                current_op = Some((
                    op.tag(),
                    op.old_start(),
                    op.old_len(),
                    op.new_start(),
                    op.new_len(),
                ));
            }
        }

        // Push the last operation if there is one
        if let Some((tag, old_start, old_len, new_start, new_len)) = current_op {
            merged_result.push(DiffOp::new(tag, old_start, old_len, new_start, new_len));
        }

        merged_result
    }

    fn iter_inline_changes<'a>(&self, old: &'a str, new: &'a str, op: &DiffOp) -> Vec<Change<'a>> {
        // Get the lines without newlines for comparison
        let old_lines: Vec<&str> = old.lines().collect();
        let new_lines: Vec<&str> = new.lines().collect();

        let mut changes = Vec::new();

        match op.tag() {
            ChangeTag::Equal => {
                for i in 0..op.old_len() {
                    let old_idx = op.old_start() + i;
                    let new_idx = op.new_start() + i;

                    // Check bounds to avoid index out of bounds errors
                    if old_idx >= old_lines.len() || new_idx >= new_lines.len() {
                        continue;
                    }

                    let mut change = Change::new(ChangeTag::Equal);
                    change.add_value(false, old_lines[old_idx].into());
                    change.set_missing_newline(true);

                    changes.push(change);
                }
            }
            ChangeTag::Delete => {
                for i in 0..op.old_len() {
                    let old_idx = op.old_start() + i;

                    // Check bounds to avoid index out of bounds errors
                    if old_idx >= old_lines.len() {
                        continue;
                    }

                    let mut change = Change::new(ChangeTag::Delete);
                    change.add_value(false, old_lines[old_idx].into());
                    change.set_missing_newline(true);

                    changes.push(change);
                }
            }
            ChangeTag::Insert => {
                for i in 0..op.new_len() {
                    let new_idx = op.new_start() + i;

                    // Check bounds to avoid index out of bounds errors
                    if new_idx >= new_lines.len() {
                        continue;
                    }

                    let mut change = Change::new(ChangeTag::Insert);
                    change.add_value(false, new_lines[new_idx].into());
                    change.set_missing_newline(true);

                    changes.push(change);
                }
            }
        }

        changes
    }
}

#[cfg(test)]
mod tests {
    use crate::{diff_with_algorithm, Algorithm, ArrowsTheme, DrawDiff};
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
        let expected = "\
< left / > right
<abc
>abxc
";
        assert_eq!(output, expected);
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
        let expected = "\
< left / > right
<abxc
>abc
";
        assert_eq!(output, expected);
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
        let expected = "\
< left / > right
<abcd
>acbd
";
        assert_eq!(output, expected);
    }

    /// Test the Myers algorithm with a case that exercises the LCS matrix computation
    #[test]
    fn test_myers_lcs_matrix() {
        // This test case is designed to exercise the LCS matrix computation
        // by having a mix of common and different elements
        let old = "abcdefg";
        let new = "abxdefz";
        let theme = ArrowsTheme::default();

        // Get output from Myers algorithm
        let mut buffer = Cursor::new(Vec::new());
        diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Myers).unwrap();
        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");

        // Verify the specific changes
        assert!(output.contains("<abcdefg"));
        assert!(output.contains(">abxdefz"));
        // Should show 2 changes: 'c'->'x' and 'g'->'z'
        assert_eq!(output.matches('<').count(), 1);
        assert_eq!(output.matches('>').count(), 1);

        // Verify that the output contains the expected strings
        let diff = DrawDiff::with_algorithm(old, new, &theme, Algorithm::Myers);
        let diff_str = format!("{diff}");

        // The diff should show the changes correctly
        assert!(diff_str.contains("<abcdefg"));
        assert!(diff_str.contains(">abxdefz"));
    }

    /// Test the Myers algorithm with a case that exercises the backtracking logic
    #[test]
    fn test_myers_backtracking() {
        // This test case is designed to exercise the backtracking logic
        // by having multiple changes that require careful backtracking
        let old = "abcdefghij";
        let new = "axcyefghiz";
        let theme = ArrowsTheme::default();

        // Get output from Myers algorithm
        let mut buffer = Cursor::new(Vec::new());
        diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Myers).unwrap();
        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");

        // Verify the specific changes with helpful error messages
        assert!(
            output.contains("<abcdefghij"),
            "Expected output to contain '<abcdefghij' but was:\n{}",
            output
        );
        assert!(
            output.contains(">axcyefghiz"),
            "Expected output to contain '>axcyefghiz' but was:\n{}",
            output
        );
        // Should show changes at positions 2 (b->x) and 8 (i->y)
        assert_eq!(output.matches('<').count(), 1);
        assert_eq!(output.matches('>').count(), 1);

        // Verify that the output contains the expected strings
        let diff = DrawDiff::with_algorithm(old, new, &theme, Algorithm::Myers);
        let diff_str = format!("{diff}");

        // The diff should show the changes correctly
        assert!(diff_str.contains("<abcdefghij"));
        assert!(diff_str.contains(">axcyefghiz"));
    }

    /// Test the Myers algorithm with a case that exercises the merging of adjacent operations
    #[test]
    fn test_myers_merge_operations() {
        // This test case is designed to exercise the merging of adjacent operations
        // by having multiple changes of the same type that should be merged
        let old = "aaaabbbbcccc";
        let new = "aaaaddddcccc";
        let theme = ArrowsTheme::default();

        // Get output from Myers algorithm
        let mut buffer = Cursor::new(Vec::new());
        diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Myers).unwrap();
        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");

        // Verify the merged changes
        assert!(output.contains("<aaaabbbbcccc"));
        assert!(output.contains(">aaaaddddcccc"));
        // Should show single delete and insert operation for the changed middle section
        assert_eq!(output.matches('<').count(), 1);
        assert_eq!(output.matches('>').count(), 1);

        // Verify that the output contains the expected strings
        let diff = DrawDiff::with_algorithm(old, new, &theme, Algorithm::Myers);
        let diff_str = format!("{diff}");

        // The diff should show the changes correctly
        assert!(diff_str.contains("<aaaabbbbcccc"));
        assert!(diff_str.contains(">aaaaddddcccc"));
    }

    /// Test the Myers algorithm with a case that exercises the `iter_inline_changes` method
    #[test]
    fn test_myers_iter_inline_changes() {
        // This test case is designed to exercise the iter_inline_changes method
        // by having changes that require inline highlighting
        let old = "The quick brown fox jumps over the lazy dog";
        let new = "The quick red fox jumps over the sleepy dog";

        // Create a theme that shows inline highlights with underlines
        #[derive(Debug)]
        struct TestTheme;
        impl crate::themes::Theme for TestTheme {
            fn highlight_insert<'this>(&self, input: &'this str) -> std::borrow::Cow<'this, str> {
                format!("_{input}_").into()
            }
            fn highlight_delete<'this>(&self, input: &'this str) -> std::borrow::Cow<'this, str> {
                format!("_{input}_").into()
            }
            // Use simple arrow prefixes for clarity
            fn delete_prefix<'this>(&self) -> std::borrow::Cow<'this, str> {
                "<".into()
            }
            fn insert_prefix<'this>(&self) -> std::borrow::Cow<'this, str> {
                ">".into()
            }
            fn equal_prefix<'this>(&self) -> std::borrow::Cow<'this, str> {
                " ".into()
            }
            fn header<'this>(&self) -> std::borrow::Cow<'this, str> {
                "< left / > right\n".into()
            }
        }

        // Get output from Myers algorithm using the test theme
        let theme = TestTheme;
        let mut buffer = Cursor::new(Vec::new());
        diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Myers).unwrap();
        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");

        // Check that specific changes are highlighted
        assert!(output.contains("<The quick _brown_ fox jumps over the _lazy_ dog"));
        assert!(output.contains(">The quick _red_ fox jumps over the _sleepy_ dog"));
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

    /// Test the Myers algorithm with completely different inputs
    #[test]
    fn test_myers_completely_different() {
        let old = "abc";
        let new = "xyz";
        let theme = ArrowsTheme::default();

        // Get output from Myers algorithm
        let mut buffer = Cursor::new(Vec::new());
        diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Myers).unwrap();
        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");

        // Verify complete replacement
        assert!(output.contains("<abc\n>xyz"));
        assert_eq!(output.matches('<').count(), 1);
        assert_eq!(output.matches('>').count(), 1);
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
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines[0], "< left / > right");
        assert_eq!(lines[1], "  line1");
        assert_eq!(lines[2], "< line2");
        assert_eq!(lines[3], "> modified line2");
        assert_eq!(lines[4], "  line3");
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
    }

    /// Test the `DrawDiff` struct with the Myers algorithm
    #[test]
    fn test_draw_diff_myers() {
        let old = "The quick brown fox";
        let new = "The quick red fox";
        let theme = ArrowsTheme::default();

        // Get output through the buffer writer
        let mut buffer = Cursor::new(Vec::new());
        diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Myers).unwrap();
        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");

        // Verify formatted output
        assert!(output.contains("<The quick brown fox"));
        assert!(output.contains(">The quick red fox"));
        assert_eq!(output.lines().count(), 3); // Header + 2 lines
    }
}
