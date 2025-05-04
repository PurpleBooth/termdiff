use crate::diff_algorithm::common::{Change, ChangeTag, DiffAlgorithm, DiffOp};

/// Implementation of the diff algorithm using the similar crate
#[derive(Debug, Default)]
pub struct SimilarDiff;

impl DiffAlgorithm for SimilarDiff {
    fn ops<'a>(&self, old: &'a str, new: &'a str) -> Vec<DiffOp> {
        // Use the similar crate to compute the diff
        let diff = similar::TextDiff::from_lines(old, new);

        // Convert the similar::DiffOp to our DiffOp
        let mut result = Vec::new();

        for op in diff.ops() {
            match op {
                similar::DiffOp::Equal {
                    old_index,
                    new_index,
                    len,
                } => {
                    result.push(DiffOp::new(
                        ChangeTag::Equal,
                        *old_index,
                        *len,
                        *new_index,
                        *len,
                    ));
                }
                similar::DiffOp::Delete {
                    old_index,
                    old_len,
                    new_index,
                } => {
                    result.push(DiffOp::new(
                        ChangeTag::Delete,
                        *old_index,
                        *old_len,
                        *new_index,
                        0,
                    ));
                }
                similar::DiffOp::Insert {
                    old_index,
                    new_index,
                    new_len,
                } => {
                    result.push(DiffOp::new(
                        ChangeTag::Insert,
                        *old_index,
                        0,
                        *new_index,
                        *new_len,
                    ));
                }
                similar::DiffOp::Replace {
                    old_index,
                    old_len,
                    new_index,
                    new_len,
                } => {
                    // A replace is a delete followed by an insert
                    result.push(DiffOp::new(
                        ChangeTag::Delete,
                        *old_index,
                        *old_len,
                        *new_index,
                        0,
                    ));
                    result.push(DiffOp::new(
                        ChangeTag::Insert,
                        *old_index,
                        0,
                        *new_index,
                        *new_len,
                    ));
                }
            }
        }

        result
    }

    fn iter_inline_changes<'a>(&self, old: &'a str, new: &'a str, op: &DiffOp) -> Vec<Change<'a>> {
        let mut changes = Vec::new();

        // Create a similar::TextDiff to use its inline_changes method
        let diff = similar::TextDiff::from_lines(old, new);

        // Convert our DiffOp to a similar::DiffOp
        let similar_op = match op.tag() {
            ChangeTag::Equal => similar::DiffOp::Equal {
                old_index: op.old_start(),
                new_index: op.new_start(),
                len: op.old_len(),
            },
            ChangeTag::Delete => similar::DiffOp::Delete {
                old_index: op.old_start(),
                old_len: op.old_len(),
                new_index: op.new_start(),
            },
            ChangeTag::Insert => similar::DiffOp::Insert {
                old_index: op.old_start(),
                new_index: op.new_start(),
                new_len: op.new_len(),
            },
        };

        // Use similar's inline_changes method to get the inline changes
        for group in diff.iter_inline_changes(&similar_op) {
            let tag = match group.tag() {
                similar::ChangeTag::Equal => ChangeTag::Equal,
                similar::ChangeTag::Delete => ChangeTag::Delete,
                similar::ChangeTag::Insert => ChangeTag::Insert,
            };

            let mut change = Change::new(tag);

            // Add each value with its highlighting information
            for (highlighted, value) in group.iter_strings_lossy() {
                // Clone the value to avoid borrowing issues
                change.add_value(highlighted, value.to_string().into());
            }

            // Set missing_newline flag
            change.set_missing_newline(group.missing_newline());

            changes.push(change);
        }

        changes
    }
}
