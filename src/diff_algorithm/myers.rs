use std::borrow::Cow;
use std::convert::TryFrom;

use crate::diff_algorithm::common::{Change, ChangeTag, DiffAlgorithm, DiffOp};

/// A single per-element edit produced by the Myers diff.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Edit {
    /// Element present in both sequences.
    Equal,
    /// Element present only in the old sequence (removed).
    Delete,
    /// Element present only in the new sequence (added).
    Insert,
}

/// Myers diff algorithm implementation
///
/// A from-scratch implementation of Eugene Myers' O(ND) difference algorithm
/// ("An O(ND) Difference Algorithm and Its Variations", 1986). It operates
/// line-by-line and does not depend on the `similar` crate.
#[derive(Debug, Default)]
pub struct MyersDiff;

impl DiffAlgorithm for MyersDiff {
    fn ops<'a>(&self, old: &'a str, new: &'a str) -> Vec<DiffOp> {
        let old_lines = split_lines_keep_newline(old);
        let new_lines = split_lines_keep_newline(new);
        group_ops(&myers_diff(&old_lines, &new_lines))
    }

    fn iter_inline_changes<'a>(&self, old: &'a str, new: &'a str, op: &DiffOp) -> Vec<Change<'a>> {
        let old_lines = split_lines_keep_newline(old);
        let new_lines = split_lines_keep_newline(new);
        inline_changes(&old_lines, &new_lines, op)
    }
}

/// Converts a non-negative coordinate produced by the Myers walk into a
/// `usize` for slice indexing.
///
/// # Panics
///
/// Panics only if `value` is negative, which never happens for a coordinate
/// that has passed the algorithm's non-negativity guards.
fn to_idx(value: i64) -> usize {
    usize::try_from(value).expect("Myers coordinates are always non-negative")
}

/// Computes the shortest edit script between two sequences using the Myers
/// O(ND) algorithm ("An O(ND) Difference Algorithm and Its Variations",
/// Myers 1986).
///
/// Returns one [`Edit`] per element of the merged walk, in forward order.
fn myers_diff<T: PartialEq>(old: &[T], new: &[T]) -> Vec<Edit> {
    let old_len = old.len();
    let new_len = new.len();
    if old_len == 0 && new_len == 0 {
        return Vec::new();
    }

    let old_i = i64::try_from(old_len).expect("old length fits in i64");
    let new_i = i64::try_from(new_len).expect("new length fits in i64");
    let offset = old_i + new_i;
    let width = to_idx(2 * offset + 1);

    // `frontier[diag + offset]` holds the furthest-reaching column reached on
    // diagonal `diag`; `-1` marks a diagonal that has not been reached yet.
    let mut frontier: Vec<i64> = vec![-1; width];
    frontier[to_idx(offset + 1)] = 0;

    // Snapshot of `frontier` at the start of each iteration, used to reconstruct
    // the path once the far corner is reached.
    let mut trace: Vec<Vec<i64>> = Vec::with_capacity(to_idx(offset + 1));

    for dist in 0..=old_len + new_len {
        trace.push(frontier.clone());
        let dist_i = i64::try_from(dist).expect("edit distance fits in i64");
        let mut diag = -dist_i;
        while diag <= dist_i {
            let idx = to_idx(diag + offset);
            // Prefer moving down (insert) when forced to the lower edge, or when
            // the diag+1 diagonal reached further than the diag-1 diagonal.
            let go_down =
                diag == -dist_i || (diag != dist_i && frontier[idx - 1] < frontier[idx + 1]);
            let mut old_pos = if go_down {
                frontier[idx + 1]
            } else {
                frontier[idx - 1] + 1
            };
            let mut new_pos = old_pos - diag;
            // Snake: slide along the diagonal while elements match.
            while old_pos >= 0
                && new_pos >= 0
                && to_idx(old_pos) < old_len
                && to_idx(new_pos) < new_len
                && old[to_idx(old_pos)] == new[to_idx(new_pos)]
            {
                old_pos += 1;
                new_pos += 1;
            }
            frontier[idx] = old_pos;
            if old_pos >= old_i && new_pos >= new_i {
                return backtrack(&trace, offset, old_i, new_i, dist);
            }
            diag += 2;
        }
    }

    // Unreachable for well-formed input: the far corner is always reached once
    // `dist` reaches `old_len + new_len` at the latest.
    Vec::new()
}

/// Reconstructs the edit sequence by walking the recorded `trace` backwards.
///
/// `offset` is the diagonal-to-index offset shared by every `frontier`
/// snapshot. The walk starts from the far corner `(old_i, new_i)`.
///
/// # Panics
///
/// Delegates to [`to_idx`] for coordinate conversion; panics only if a
/// coordinate were negative, which the algorithm's invariants forbid.
fn backtrack(
    trace: &[Vec<i64>],
    offset: i64,
    old_i: i64,
    new_i: i64,
    dist_total: usize,
) -> Vec<Edit> {
    let mut edits: Vec<Edit> = Vec::new();
    let mut old_pos = old_i;
    let mut new_pos = new_i;

    for dist in (1..=dist_total).rev() {
        let frontier = &trace[dist];
        let diag = old_pos - new_pos;
        let idx = to_idx(diag + offset);
        let dist_i = i64::try_from(dist).expect("edit distance fits in i64");
        let go_down = diag == -dist_i || (diag != dist_i && frontier[idx - 1] < frontier[idx + 1]);
        let prev_diag = if go_down { diag + 1 } else { diag - 1 };
        let prev_old = frontier[to_idx(prev_diag + offset)];
        let prev_new = prev_old - prev_diag;

        // The edit step landed at (step_old, step_new); undo the snake back to
        // it, then undo the single step.
        let (step_old, step_new) = if go_down {
            (prev_old, prev_new + 1)
        } else {
            (prev_old + 1, prev_new)
        };
        while old_pos > step_old && new_pos > step_new {
            edits.push(Edit::Equal);
            old_pos -= 1;
            new_pos -= 1;
        }
        if go_down {
            edits.push(Edit::Insert);
            new_pos -= 1;
        } else {
            edits.push(Edit::Delete);
            old_pos -= 1;
        }
    }

    // Whatever remains back to the origin is the initial equal run (the
    // dist = 0 snake), emitted as equals.
    while old_pos > 0 && new_pos > 0 {
        edits.push(Edit::Equal);
        old_pos -= 1;
        new_pos -= 1;
    }

    edits.reverse();
    edits
}

/// Groups a flat list of per-element edits into run-length [`DiffOp`]s.
fn group_ops(edits: &[Edit]) -> Vec<DiffOp> {
    let mut ops = Vec::new();
    let mut old_idx = 0usize;
    let mut new_idx = 0usize;
    let mut i = 0;
    while i < edits.len() {
        let edit = edits[i];
        let mut len = 1;
        while i + len < edits.len() && edits[i + len] == edit {
            len += 1;
        }
        match edit {
            Edit::Equal => {
                ops.push(DiffOp::new(ChangeTag::Equal, old_idx, len, new_idx, len));
                old_idx += len;
                new_idx += len;
            }
            Edit::Delete => {
                ops.push(DiffOp::new(ChangeTag::Delete, old_idx, len, new_idx, 0));
                old_idx += len;
            }
            Edit::Insert => {
                ops.push(DiffOp::new(ChangeTag::Insert, old_idx, 0, new_idx, len));
                new_idx += len;
            }
        }
        i += len;
    }
    ops
}

/// Builds the per-line [`Change`] values for a single [`DiffOp`].
fn inline_changes<'a>(
    old_lines: &[&'a str],
    new_lines: &[&'a str],
    op: &DiffOp,
) -> Vec<Change<'a>> {
    let mut changes = Vec::new();
    match op.tag() {
        ChangeTag::Equal => {
            for (offset, line) in (op.old_start()..op.old_start() + op.old_len())
                .map(|i| old_lines[i])
                .enumerate()
            {
                let mut change = Change::new(ChangeTag::Equal);
                change.add_value(false, Cow::Borrowed(line));
                change.set_missing_newline(!line.ends_with('\n'));
                let _ = offset;
                changes.push(change);
            }
        }
        ChangeTag::Delete => {
            for line in (op.old_start()..op.old_start() + op.old_len()).map(|i| old_lines[i]) {
                let mut change = Change::new(ChangeTag::Delete);
                change.add_value(false, Cow::Borrowed(line));
                change.set_missing_newline(!line.ends_with('\n'));
                changes.push(change);
            }
        }
        ChangeTag::Insert => {
            for line in (op.new_start()..op.new_start() + op.new_len()).map(|i| new_lines[i]) {
                let mut change = Change::new(ChangeTag::Insert);
                change.add_value(false, Cow::Borrowed(line));
                change.set_missing_newline(!line.ends_with('\n'));
                changes.push(change);
            }
        }
    }
    changes
}

/// Splits `s` into lines, keeping the trailing `'\n'` on each line (the way
/// the `similar` crate's `from_lines` does). An empty string yields no lines.
fn split_lines_keep_newline(s: &str) -> Vec<&str> {
    if s.is_empty() {
        return Vec::new();
    }
    let mut lines = Vec::new();
    let mut start = 0;
    for (idx, ch) in s.char_indices() {
        if ch == '\n' {
            lines.push(&s[start..=idx]);
            start = idx + ch.len_utf8();
        }
    }
    if start < s.len() {
        lines.push(&s[start..]);
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{diff_with_algorithm, Algorithm, ArrowsTheme};
    use std::io::Cursor;

    // ---- Pure algorithm unit tests ----

    #[test]
    fn myers_diff_insert_in_middle() {
        let a = ["a", "c"];
        let b = ["a", "b", "c"];
        assert_eq!(
            myers_diff(&a, &b),
            vec![Edit::Equal, Edit::Insert, Edit::Equal]
        );
    }

    #[test]
    fn myers_diff_delete_in_middle() {
        let a = ["a", "b", "c"];
        let b = ["a", "c"];
        assert_eq!(
            myers_diff(&a, &b),
            vec![Edit::Equal, Edit::Delete, Edit::Equal]
        );
    }

    #[test]
    fn myers_diff_identical() {
        let a = ["a", "b"];
        let b = ["a", "b"];
        assert_eq!(myers_diff(&a, &b), vec![Edit::Equal, Edit::Equal]);
    }

    #[test]
    fn myers_diff_empty_old() {
        let b = ["a", "b"];
        assert_eq!(myers_diff(&[], &b), vec![Edit::Insert, Edit::Insert]);
    }

    #[test]
    fn myers_diff_empty_new() {
        let a = ["a", "b"];
        assert_eq!(myers_diff(&a, &[]), vec![Edit::Delete, Edit::Delete]);
    }

    #[test]
    fn myers_diff_both_empty() {
        assert!(myers_diff::<&str>(&[], &[]).is_empty());
    }

    #[test]
    fn myers_diff_all_different() {
        let a = ["x"];
        let b = ["y"];
        assert_eq!(myers_diff(&a, &b), vec![Edit::Delete, Edit::Insert]);
    }

    #[test]
    fn myers_diff_interleaved_changes() {
        let a = ["a", "b", "c", "d"];
        let b = ["a", "x", "c", "y"];
        assert_eq!(
            myers_diff(&a, &b),
            vec![
                Edit::Equal,
                Edit::Delete,
                Edit::Insert,
                Edit::Equal,
                Edit::Delete,
                Edit::Insert,
            ]
        );
    }

    #[test]
    fn myers_diff_common_prefix_and_suffix() {
        // "ab|c|abc" vs "ab|X|abc" — keep ab, delete c, insert X, keep abc
        let a = ["a", "b", "c", "a", "b", "c"];
        let b = ["a", "b", "X", "a", "b", "c"];
        assert_eq!(
            myers_diff(&a, &b),
            vec![
                Edit::Equal,
                Edit::Equal,
                Edit::Delete,
                Edit::Insert,
                Edit::Equal,
                Edit::Equal,
                Edit::Equal,
            ]
        );
    }

    #[test]
    fn group_ops_collapses_runs() {
        let edits = vec![
            Edit::Equal,
            Edit::Equal,
            Edit::Delete,
            Edit::Insert,
            Edit::Insert,
            Edit::Equal,
        ];
        let ops = group_ops(&edits);
        assert_eq!(ops.len(), 4);
        assert_eq!(ops[0], DiffOp::new(ChangeTag::Equal, 0, 2, 0, 2));
        assert_eq!(ops[1], DiffOp::new(ChangeTag::Delete, 2, 1, 2, 0));
        assert_eq!(ops[2], DiffOp::new(ChangeTag::Insert, 3, 0, 2, 2));
        assert_eq!(ops[3], DiffOp::new(ChangeTag::Equal, 3, 1, 4, 1));
    }

    #[test]
    fn split_lines_keeps_newline_on_internal_lines() {
        assert_eq!(split_lines_keep_newline("a\nb\nc"), vec!["a\n", "b\n", "c"]);
        assert_eq!(split_lines_keep_newline("a\nb\n"), vec!["a\n", "b\n"]);
        assert_eq!(split_lines_keep_newline("abc"), vec!["abc"]);
        assert!(split_lines_keep_newline("").is_empty());
    }

    // ---- Integration tests ----

    #[test]
    fn test_myers_insertion() {
        let old = "abc";
        let new = "abxc";
        let theme = ArrowsTheme::default();

        let mut buffer = Cursor::new(Vec::new());
        diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Myers).unwrap();
        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");

        assert!(output.contains("<abc"));
        assert!(output.contains(">abxc"));
    }

    #[test]
    fn test_myers_deletion() {
        let old = "abxc";
        let new = "abc";
        let theme = ArrowsTheme::default();

        let mut buffer = Cursor::new(Vec::new());
        diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Myers).unwrap();
        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");

        assert!(output.contains("<abxc"));
        assert!(output.contains(">abc"));
    }

    #[test]
    fn test_myers_empty_inputs() {
        let theme = ArrowsTheme::default();

        let mut buffer = Cursor::new(Vec::new());
        diff_with_algorithm(&mut buffer, "", "", &theme, Algorithm::Myers).unwrap();
        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");
        assert_eq!(output, "< left / > right\n");

        let mut buffer = Cursor::new(Vec::new());
        diff_with_algorithm(&mut buffer, "", "abc", &theme, Algorithm::Myers).unwrap();
        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");
        assert!(output.contains(">abc"));
    }

    #[test]
    fn test_myers_identical_inputs() {
        let old = "abc";
        let new = "abc";
        let theme = ArrowsTheme::default();

        let mut buffer = Cursor::new(Vec::new());
        diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Myers).unwrap();
        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");

        assert!(output.contains(" abc"));
        assert!(!output.contains("<abc"));
        assert!(!output.contains(">abc"));
    }
}
