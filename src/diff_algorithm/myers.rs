use crate::diff_algorithm::common::{Change, ChangeTag, DiffAlgorithm, DiffOp};

/// Implementation of the Myers diff algorithm based on "An O(ND) Difference Algorithm"
/// by Eugene W. Myers.
#[derive(Debug, Default)]
pub struct MyersDiff;

impl DiffAlgorithm for MyersDiff {
    fn ops<'a>(&self, old: &'a str, new: &'a str) -> Vec<DiffOp> {
        // To match the behavior of the Similar algorithm, we need to handle the input differently
        // If both inputs are empty, return an empty vector
        if old.is_empty() && new.is_empty() {
            return Vec::new();
        }

        // Split the input strings into lines for line-by-line comparison
        // Use split_inclusive to preserve newlines
        let old_lines: Vec<&str> = if old.is_empty() {
            Vec::new()
        } else if old.ends_with('\n') {
            old.split_inclusive('\n').collect()
        } else {
            let mut lines = old.split_inclusive('\n').collect::<Vec<_>>();
            if !lines.is_empty() && !lines.last().unwrap().ends_with('\n') {
                lines.push(&old[old.rfind('\n').map_or(0, |i| i + 1)..]);
            }
            lines
        };

        let new_lines: Vec<&str> = if new.is_empty() {
            Vec::new()
        } else if new.ends_with('\n') {
            new.split_inclusive('\n').collect()
        } else {
            let mut lines = new.split_inclusive('\n').collect::<Vec<_>>();
            if !lines.is_empty() && !lines.last().unwrap().ends_with('\n') {
                lines.push(&new[new.rfind('\n').map_or(0, |i| i + 1)..]);
            }
            lines
        };

        // Handle empty inputs
        if old_lines.is_empty() {
            // All lines are insertions
            return vec![DiffOp::new(ChangeTag::Insert, 0, 0, 0, new_lines.len())];
        }

        if new_lines.is_empty() {
            // All lines are deletions
            return vec![DiffOp::new(ChangeTag::Delete, 0, old_lines.len(), 0, 0)];
        }

        // Compute the diff operations using the Myers algorithm
        let edit_script = compute_edit_script(&old_lines, &new_lines);

        // Convert the edit script to DiffOps
        let mut diff_ops = Vec::new();
        let mut old_idx = 0;
        let mut new_idx = 0;

        for op in edit_script {
            match op {
                EditOp::Equal => {
                    diff_ops.push(DiffOp::new(ChangeTag::Equal, old_idx, 1, new_idx, 1));
                    old_idx += 1;
                    new_idx += 1;
                }
                EditOp::Delete => {
                    diff_ops.push(DiffOp::new(ChangeTag::Delete, old_idx, 1, new_idx, 0));
                    old_idx += 1;
                }
                EditOp::Insert => {
                    diff_ops.push(DiffOp::new(ChangeTag::Insert, old_idx, 0, new_idx, 1));
                    new_idx += 1;
                }
            }
        }

        // Merge adjacent operations of the same type
        merge_adjacent_ops(diff_ops)
    }

    fn iter_inline_changes<'a>(&self, old: &'a str, new: &'a str, op: &DiffOp) -> Vec<Change<'a>> {
        let mut changes = Vec::new();

        // Create a single change for the entire operation to match Similar algorithm's behavior
        let mut change = Change::new(op.tag());

        match op.tag() {
            ChangeTag::Equal => {
                // Extract the relevant portion of the old text
                let start = op.old_start();
                let end = start + op.old_len();

                // Get the lines from the old text
                let old_lines: Vec<&str> = old.lines().collect();

                // Make sure we don't go out of bounds
                if start < old_lines.len() {
                    let end = end.min(old_lines.len());

                    for i in start..end {
                        let line = old_lines[i];
                        // Check if this is the last line and doesn't have a newline
                        let missing_newline = i == old_lines.len() - 1 && !old.ends_with('\n');

                        // Add a space before the value to match Similar algorithm's behavior
                        change.add_value(false, format!(" {}", line).into());
                        change.set_missing_newline(missing_newline);
                    }
                }
            }
            ChangeTag::Delete => {
                // Extract the relevant portion of the old text
                let start = op.old_start();
                let end = start + op.old_len();

                // Get the lines from the old text
                let old_lines: Vec<&str> = old.lines().collect();

                // Make sure we don't go out of bounds
                if start < old_lines.len() {
                    let end = end.min(old_lines.len());

                    for i in start..end {
                        let line = old_lines[i];
                        // Check if this is the last line and doesn't have a newline
                        let missing_newline = i == old_lines.len() - 1 && !old.ends_with('\n');

                        // Add a space before the value to match Similar algorithm's behavior
                        change.add_value(true, format!(" {}", line).into());
                        change.set_missing_newline(missing_newline);
                    }
                }
            }
            ChangeTag::Insert => {
                // Extract the relevant portion of the new text
                let start = op.new_start();
                let end = start + op.new_len();

                // Get the lines from the new text
                let new_lines: Vec<&str> = new.lines().collect();

                // Make sure we don't go out of bounds
                if start < new_lines.len() {
                    let end = end.min(new_lines.len());

                    for i in start..end {
                        let line = new_lines[i];
                        // Check if this is the last line and doesn't have a newline
                        let missing_newline = i == new_lines.len() - 1 && !new.ends_with('\n');

                        // Add a space before the value to match Similar algorithm's behavior
                        change.add_value(true, format!(" {}", line).into());
                        change.set_missing_newline(missing_newline);
                    }
                }
            }
        }

        // Only add the change if it has values
        if !change.values().is_empty() {
            changes.push(change);
        }

        changes
    }
}

/// Represents an edit operation in the Myers algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EditOp {
    Equal,
    Insert,
    Delete,
}

/// Computes the shortest edit script using the Myers algorithm
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]
fn compute_edit_script<T: PartialEq>(old_seq: &[T], new_seq: &[T]) -> Vec<EditOp> {
    let old_len = old_seq.len();
    let new_len = new_seq.len();
    let max_edit_distance = old_len + new_len;

    // Handle edge cases
    if old_len == 0 {
        return vec![EditOp::Insert; new_len];
    }
    if new_len == 0 {
        return vec![EditOp::Delete; old_len];
    }

    // The algorithm uses a vector v to store the furthest reaching D-paths
    // v[k + offset] = x means that the furthest reaching D-path ending at diagonal k
    // has reached position (x, x - k)
    let offset = max_edit_distance;
    let mut v = vec![0_i32; 2 * max_edit_distance + 1];

    // Store the entire edit history to reconstruct the path
    let mut trace = Vec::with_capacity(max_edit_distance + 1);

    // For each edit distance d
    for d in 0..=max_edit_distance {
        // Save the current state of v for backtracking
        trace.push(v.clone());

        // For each diagonal k from -d to d in steps of 2
        for k in (-(d as i32)..=d as i32).step_by(2) {
            // Determine whether to go down or right
            let mut x = if k == -(d as i32)
                || (k != d as i32
                    && v[(k - 1 + offset as i32) as usize] < v[(k + 1 + offset as i32) as usize])
            {
                v[(k + 1 + offset as i32) as usize] // Move down: (x, y-1) -> (x, y)
            } else {
                v[(k - 1 + offset as i32) as usize] + 1 // Move right: (x-1, y) -> (x, y)
            };

            let mut y = x - k;

            // Follow diagonal moves (matches) as far as possible
            while x < old_len as i32
                && y < new_len as i32
                && old_seq[x as usize] == new_seq[y as usize]
            {
                x += 1;
                y += 1;
            }

            // Store the furthest reaching path for this diagonal
            v[(k + offset as i32) as usize] = x;

            // If we've reached the bottom right corner, we're done
            if x >= old_len as i32 && y >= new_len as i32 {
                // Reconstruct the edit script from the trace
                return backtrack_path(&trace, old_len, new_len);
            }
        }
    }

    // This should never happen if the algorithm is implemented correctly
    Vec::new()
}

/// Reconstructs the edit script by backtracking through the trace
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]
fn backtrack_path(trace: &[Vec<i32>], old_len: usize, new_len: usize) -> Vec<EditOp> {
    let offset = old_len + new_len;
    let mut edit_script = Vec::new();
    let mut x = old_len as i32;
    let mut y = new_len as i32;

    // Start from the last edit distance and work backwards
    for d in (0..trace.len()).rev() {
        let v = &trace[d];
        let k = x - y;

        // Determine whether we came from a vertical, horizontal, or diagonal move
        let prev_k = if k == -(d as i32)
            || (k != d as i32
                && v[(k - 1 + offset as i32) as usize] < v[(k + 1 + offset as i32) as usize])
        {
            k + 1
        } else {
            k - 1
        };

        let prev_x = v[(prev_k + offset as i32) as usize];
        let prev_y = prev_x - prev_k;

        // Add diagonal moves (matches)
        while x > prev_x && y > prev_y {
            edit_script.push(EditOp::Equal);
            x -= 1;
            y -= 1;
        }

        // Add the non-diagonal move
        if d > 0 {
            if x == prev_x {
                edit_script.push(EditOp::Insert);
                y -= 1;
            } else {
                edit_script.push(EditOp::Delete);
                x -= 1;
            }
        }
    }

    // Reverse the edit script to get the correct order
    edit_script.reverse();
    edit_script
}

/// Merges adjacent operations of the same type
fn merge_adjacent_ops(ops: Vec<DiffOp>) -> Vec<DiffOp> {
    if ops.is_empty() {
        return ops;
    }

    let mut merged = Vec::new();
    let mut current_tag = ops[0].tag();
    let mut current_old_start = ops[0].old_start();
    let mut current_old_len = ops[0].old_len();
    let mut current_new_start = ops[0].new_start();
    let mut current_new_len = ops[0].new_len();

    for op in ops.into_iter().skip(1) {
        if current_tag == op.tag()
            && current_old_start + current_old_len == op.old_start()
            && current_new_start + current_new_len == op.new_start()
        {
            // Merge with the current operation
            current_old_len += op.old_len();
            current_new_len += op.new_len();
        } else {
            // Push the current operation and start a new one
            merged.push(DiffOp::new(
                current_tag,
                current_old_start,
                current_old_len,
                current_new_start,
                current_new_len,
            ));
            current_tag = op.tag();
            current_old_start = op.old_start();
            current_old_len = op.old_len();
            current_new_start = op.new_start();
            current_new_len = op.new_len();
        }
    }

    // Don't forget to push the last operation
    merged.push(DiffOp::new(
        current_tag,
        current_old_start,
        current_old_len,
        current_new_start,
        current_new_len,
    ));

    merged
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
