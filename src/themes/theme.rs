use std::{borrow::Cow, fmt::Debug};

/// A [`Theme`] for customizing the appearance of diffs
///
/// This trait allows you to control how diffs are displayed without having
/// to parse the diff output yourself. You can customize prefixes, highlighting,
/// and formatting for different types of changes.
///
/// # Implementing a Custom Theme
///
/// To create a custom theme, you must implement at minimum:
/// - `equal_prefix`: Prefix for unchanged lines
/// - `delete_prefix`: Prefix for removed lines
/// - `insert_prefix`: Prefix for added lines
/// - `header`: Header text displayed at the top of the diff
///
/// All other methods have default implementations that you can override as needed.
///
/// # Example
///
/// ```rust
/// use std::borrow::Cow;
/// use termdiff::Theme;
///
/// #[derive(Debug)]
/// struct MyCustomTheme {}
///
/// impl Theme for MyCustomTheme {
///     // Required methods
///     fn equal_prefix<'this>(&self) -> Cow<'this, str> {
///         " ".into()  // Space for unchanged lines
///     }
///
///     fn delete_prefix<'this>(&self) -> Cow<'this, str> {
///         "[-]".into()  // Custom prefix for removed lines
///     }
///
///     fn insert_prefix<'this>(&self) -> Cow<'this, str> {
///         "[+]".into()  // Custom prefix for added lines
///     }
///
///     fn header<'this>(&self) -> Cow<'this, str> {
///         "=== DIFF RESULTS ===\n".into()  // Custom header
///     }
///
///     // Optional overrides for customizing content formatting
///     fn delete_content<'this>(&self, input: &'this str) -> Cow<'this, str> {
///         format!("REMOVED: {}", input).into()  // Custom formatting for removed content
///     }
///
///     fn insert_line<'this>(&self, input: &'this str) -> Cow<'this, str> {
///         format!("ADDED: {}", input).into()  // Custom formatting for added content
///     }
/// }
/// ```
pub trait Theme: Debug {
    /// How to format the text when highlighting specific changes in inserted lines
    ///
    /// This is used to highlight the specific parts of text that have changed within
    /// an inserted line. By default, it returns the input unchanged.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::borrow::Cow;
    /// # use termdiff::Theme;
    /// # #[derive(Debug)]
    /// # struct MyTheme;
    /// # impl Theme for MyTheme {
    /// #     fn equal_prefix<'a>(&self) -> Cow<'a, str> { " ".into() }
    /// #     fn delete_prefix<'a>(&self) -> Cow<'a, str> { "-".into() }
    /// #     fn insert_prefix<'a>(&self) -> Cow<'a, str> { "+".into() }
    /// #     fn header<'a>(&self) -> Cow<'a, str> { "".into() }
    /// fn highlight_insert<'this>(&self, input: &'this str) -> Cow<'this, str> {
    ///     format!("**{}**", input).into()  // Bold the inserted text
    /// }
    /// # }
    /// ```
    fn highlight_insert<'this>(&self, input: &'this str) -> Cow<'this, str> {
        input.into()
    }

    /// How to format the text when highlighting specific changes in deleted lines
    ///
    /// This is used to highlight the specific parts of text that have changed within
    /// a deleted line. By default, it returns the input unchanged.
    fn highlight_delete<'this>(&self, input: &'this str) -> Cow<'this, str> {
        input.into()
    }

    /// How to format unchanged content
    ///
    /// This method is called for content that exists in both the old and new text.
    /// By default, it returns the input unchanged.
    fn equal_content<'this>(&self, input: &'this str) -> Cow<'this, str> {
        input.into()
    }

    /// How to format content that is being removed
    ///
    /// This method is called for content that exists only in the old text.
    /// By default, it returns the input unchanged.
    fn delete_content<'this>(&self, input: &'this str) -> Cow<'this, str> {
        input.into()
    }

    /// The prefix to display before lines that are unchanged
    ///
    /// This is typically a space or other character that indicates the line is unchanged.
    /// This method is required for all theme implementations.
    fn equal_prefix<'this>(&self) -> Cow<'this, str>;

    /// The prefix to display before lines that are being removed
    ///
    /// This is typically a character like '-' that indicates the line is being removed.
    /// This method is required for all theme implementations.
    fn delete_prefix<'this>(&self) -> Cow<'this, str>;

    /// How to format content that is being added
    ///
    /// This method is called for content that exists only in the new text.
    /// By default, it returns the input unchanged.
    fn insert_line<'this>(&self, input: &'this str) -> Cow<'this, str> {
        input.into()
    }

    /// The prefix to display before lines that are being added
    ///
    /// This is typically a character like '+' that indicates the line is being added.
    /// This method is required for all theme implementations.
    fn insert_prefix<'this>(&self) -> Cow<'this, str>;

    /// The string to append when a diff line doesn't end with a newline
    ///
    /// By default, this adds a newline character.
    fn line_end<'this>(&self) -> Cow<'this, str> {
        "\n".into()
    }

    /// The marker to show when one string has a trailing newline and the other doesn't
    ///
    /// When one of the two strings ends with a newline and the other doesn't,
    /// this character is inserted before the newline to make the difference visible.
    /// By default, this uses the Unicode character '␊' (U+240A).
    fn trailing_lf_marker<'this>(&self) -> Cow<'this, str> {
        "␊".into()
    }

    /// The header text to display at the top of the diff
    ///
    /// This is typically a line explaining the diff format.
    /// This method is required for all theme implementations.
    fn header<'this>(&self) -> Cow<'this, str>;
}
