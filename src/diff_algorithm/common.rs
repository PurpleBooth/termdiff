use std::borrow::Cow;

/// Represents a change in the diff
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeTag {
    /// No change between old and new
    Equal,
    /// Content was in old but not in new
    Delete,
    /// Content was not in old but is in new
    Insert,
}

/// Represents a change with inline highlighting information
#[derive(Debug)]
pub struct Change<'a> {
    /// The tag indicating the type of change
    tag: ChangeTag,
    /// The values with highlighting information
    values: Vec<(bool, Cow<'a, str>)>,
    /// Whether the line is missing a newline
    missing_newline: bool,
}

impl<'a> Change<'a> {
    /// Creates a new change
    #[must_use]
    pub const fn new(tag: ChangeTag) -> Self {
        Self {
            tag,
            values: Vec::new(),
            missing_newline: false,
        }
    }

    /// Adds a value to the change
    pub fn add_value(&mut self, highlight: bool, value: Cow<'a, str>) {
        self.values.push((highlight, value));
    }

    /// Sets whether the line is missing a newline
    pub const fn set_missing_newline(&mut self, missing_newline: bool) {
        self.missing_newline = missing_newline;
    }

    /// Returns the tag for this change
    #[must_use]
    pub const fn tag(&self) -> ChangeTag {
        self.tag
    }

    /// Returns the values with highlighting information
    #[must_use]
    pub fn values(&self) -> &[(bool, Cow<'a, str>)] {
        &self.values
    }

    /// Returns whether the line is missing a newline
    #[must_use]
    pub const fn missing_newline(&self) -> bool {
        self.missing_newline
    }
}

/// Represents a diff operation
#[derive(Debug)]
pub struct DiffOp {
    /// The type of change
    tag: ChangeTag,
    /// The start index in the old text
    old_start: usize,
    /// The number of elements in the old text
    old_len: usize,
    /// The start index in the new text
    new_start: usize,
    /// The number of elements in the new text
    new_len: usize,
}

impl DiffOp {
    /// Creates a new diff operation
    #[must_use]
    pub const fn new(
        tag: ChangeTag,
        old_start: usize,
        old_len: usize,
        new_start: usize,
        new_len: usize,
    ) -> Self {
        Self {
            tag,
            old_start,
            old_len,
            new_start,
            new_len,
        }
    }

    /// Returns the tag for this operation
    #[must_use]
    pub const fn tag(&self) -> ChangeTag {
        self.tag
    }

    /// Returns the start index in the old text
    #[must_use]
    pub const fn old_start(&self) -> usize {
        self.old_start
    }

    /// Returns the number of elements in the old text
    #[must_use]
    pub const fn old_len(&self) -> usize {
        self.old_len
    }

    /// Returns the start index in the new text
    #[must_use]
    pub const fn new_start(&self) -> usize {
        self.new_start
    }

    /// Returns the number of elements in the new text
    #[must_use]
    pub const fn new_len(&self) -> usize {
        self.new_len
    }
}

/// Trait for diff algorithms
pub trait DiffAlgorithm {
    /// Computes the diff operations between old and new text
    fn ops<'a>(&self, old: &'a str, new: &'a str) -> Vec<DiffOp>;

    /// Computes the inline changes for a diff operation
    fn iter_inline_changes<'a>(&self, old: &'a str, new: &'a str, op: &DiffOp) -> Vec<Change<'a>>;
}

/// The algorithm to use for computing diffs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algorithm {
    /// Use the similar crate's algorithm (default)
    Similar,
    /// Use our implementation of the Myers algorithm
    Myers,
}

impl Algorithm {
    /// Returns a list of available algorithms based on enabled features
    #[must_use]
    pub fn available_algorithms() -> Vec<Self> {
        let algorithms = vec![
            #[cfg(feature = "similar")]
            Self::Similar,
            #[cfg(feature = "myers")]
            Self::Myers,
        ];

        algorithms
    }

    /// Checks if any algorithms are available
    #[must_use]
    pub fn has_available_algorithms() -> bool {
        !Self::available_algorithms().is_empty()
    }

    /// Returns the first available algorithm, or None if no algorithms are available
    #[must_use]
    pub fn first_available() -> Option<Self> {
        Self::available_algorithms().first().copied()
    }
}

impl Default for Algorithm {
    fn default() -> Self {
        Self::first_available().unwrap_or(Self::Similar)
    }
}
