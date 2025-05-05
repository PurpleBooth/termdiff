use std::borrow::Cow;
use std::cmp::min;

use crate::diff_algorithm::common::{Change, ChangeTag, DiffAlgorithm, DiffOp};

/// Implementation of the Myers diff algorithm based on "An O(ND) Difference Algorithm"
/// by Eugene W. Myers.
#[derive(Debug, Default)]
pub struct MyersDiff;

impl DiffAlgorithm for MyersDiff {
    fn ops<'a>(&self, old: &'a str, new: &'a str) -> Vec<DiffOp> {
        // Split the input strings into lines for line-by-line comparison
        let old_lines: Vec<&str> = old.lines().collect();
        let new_lines: Vec<&str> = new.lines().collect();

        // Handle empty inputs
        if old_lines.is_empty() && new_lines.is_empty() {
            return Vec::new();
        }

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
        let old_lines: Vec<&str> = old.lines().collect();
        let new_lines: Vec<&str> = new.lines().collect();
        let mut changes = Vec::new();

        match op.tag() {
            ChangeTag::Equal => {
                for i in 0..op.old_len() {
                    let old_idx = op.old_start() + i;
                    if old_idx >= old_lines.len() {
                        continue;
                    }

                    let mut change = Change::new(ChangeTag::Equal);
                    change.add_value(false, old_lines[old_idx].into());
                    
                    // Check if this is the last line and it's missing a newline
                    let missing_newline = old_idx == old_lines.len() - 1 && !old.ends_with('\n');
                    change.set_missing_newline(missing_newline);
                    
                    changes.push(change);
                }
            }
            ChangeTag::Delete => {
                for i in 0..op.old_len() {
                    let old_idx = op.old_start() + i;
                    if old_idx >= old_lines.len() {
                        continue;
                    }

                    let mut change = Change::new(ChangeTag::Delete);
                    change.add_value(true, old_lines[old_idx].into());
                    
                    // Check if this is the last line and it's missing a newline
                    let missing_newline = old_idx == old_lines.len() - 1 && !old.ends_with('\n');
                    change.set_missing_newline(missing_newline);
                    
                    changes.push(change);
                }
            }
            ChangeTag::Insert => {
                for i in 0..op.new_len() {
                    let new_idx = op.new_start() + i;
                    if new_idx >= new_lines.len() {
                        continue;
                    }

                    let mut change = Change::new(ChangeTag::Insert);
                    change.add_value(true, new_lines[new_idx].into());
                    
                    // Check if this is the last line and it's missing a newline
                    let missing_newline = new_idx == new_lines.len() - 1 && !new.ends_with('\n');
                    change.set_missing_newline(missing_newline);
                    
                    changes.push(change);
                }
            }
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
fn compute_edit_script<T: PartialEq>(a: &[T], b: &[T]) -> Vec<EditOp> {
    let n = a.len();
    let m = b.len();
    let max = n + m;
    
    // Handle edge cases
    if n == 0 {
        return vec![EditOp::Insert; m];
    }
    if m == 0 {
        return vec![EditOp::Delete; n];
    }

    // The algorithm uses a vector v to store the furthest reaching D-paths
    // v[k + max] = x means that the furthest reaching D-path ending at diagonal k
    // has reached position (x, x - k)
    let mut v = vec![0; 2 * max + 1];
    
    // Store the entire edit history to reconstruct the path
    let mut trace = Vec::with_capacity(max + 1);
    
    // For each edit distance d
    for d in 0..=max {
        // Save the current state of v for backtracking
        trace.push(v.clone());
        
        // For each diagonal k from -d to d in steps of 2
        for k in (-d..=d).step_by(2) {
            // Determine whether to go down or right
            let mut x = if k == -d || (k != d && v[k - 1 + max] < v[k + 1 + max]) {
                v[k + 1 + max] // Move down: (x, y-1) -> (x, y)
            } else {
                v[k - 1 + max] + 1 // Move right: (x-1, y) -> (x, y)
            };
            
            let mut y = x - k;
            
            // Follow diagonal moves (matches) as far as possible
            while x < n as i32 && y < m as i32 && a[x as usize] == b[y as usize] {
                x += 1;
                y += 1;
            }
            
            // Store the furthest reaching path for this diagonal
            v[k + max] = x;
            
            // If we've reached the bottom right corner, we're done
            if x >= n as i32 && y >= m as i32 {
                // Reconstruct the edit script from the trace
                return backtrack_path(trace, n, m);
            }
        }
    }
    
    // This should never happen if the algorithm is implemented correctly
    Vec::new()
}

/// Reconstructs the edit script by backtracking through the trace
fn backtrack_path(trace: Vec<Vec<i32>>, n: usize, m: usize) -> Vec<EditOp> {
    let max = n + m;
    let mut edit_script = Vec::new();
    let mut x = n as i32;
    let mut y = m as i32;
    
    // Start from the last edit distance and work backwards
    for d in (0..trace.len()).rev() {
        let v = &trace[d];
        let k = x - y;
        
        // Determine whether we came from a vertical, horizontal, or diagonal move
        let prev_k = if k == -d as i32 || (k != d as i32 && v[k - 1 + max] < v[k + 1 + max]) {
            k + 1
        } else {
            k - 1
        };
        
        let prev_x = v[prev_k + max];
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
    let mut current = ops[0].clone();
    
    for op in ops.into_iter().skip(1) {
        if current.tag() == op.tag()
            && current.old_start() + current.old_len() == op.old_start()
            && current.new_start() + current.new_len() == op.new_start()
        {
            // Merge with the current operation
            current = DiffOp::new(
                current.tag(),
                current.old_start(),
                current.old_len() + op.old_len(),
                current.new_start(),
                current.new_len() + op.new_len(),
            );
        } else {
            // Push the current operation and start a new one
            merged.push(current);
            current = op;
        }
    }
    
    // Don't forget to push the last operation
    merged.push(current);
    
    merged
}

#[cfg(test)]
mod tests {
    use super::*;
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
    }
}
