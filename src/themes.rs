use crossterm::style::{StyledContent, Stylize};

/// Take a string and format it for some purpose
type LineFormatter = fn(String) -> StyledContent<String>;

/// A [`Theme`] for the diff
///
/// This is to allows some control over what the diff looks like without having
/// to parse it yourself
pub struct Theme {
    /// How to format the text when highlighting it for inserts
    pub highlight_insert: LineFormatter,
    /// How to format the text when highlighting it for deletes
    pub highlight_delete: LineFormatter,
    /// How to format unchanged content
    pub equal_content: LineFormatter,
    /// The prefix to give lines that are equal
    pub equal_prefix: String,
    /// How to format bits of text that are being removed
    pub delete_content: LineFormatter,
    /// The prefix to give lines that are being removed
    pub delete_prefix: String,
    /// How to format bits of text that are being added
    pub insert_line: LineFormatter,
    /// The prefix to give lines that are being added
    pub insert_prefix: String,
    /// If a diff line doesn't end with a newline, what should we insert
    pub line_end: String,
    /// A header to put above the diff
    pub header: String,
}

/// A simple colorless using arrows theme
///
/// # Examples
///
/// ```
/// use termdiff::{arrows_theme, diff};
/// let old = "The quick brown fox and\njumps over the sleepy dog";
/// let new = "The quick red fox and\njumps over the lazy dog";
/// let mut buffer: Vec<u8> = Vec::new();
/// diff(&mut buffer, old, new, arrows_theme()).unwrap();
/// let actual: String = String::from_utf8(buffer).expect("Not valid UTF-8");
///
/// assert_eq!(
///     actual,
///     "< left / > right
/// <The quick brown fox and
/// <jumps over the sleepy dog
/// >The quick red fox and
/// >jumps over the lazy dog
/// "
/// );
/// ```
#[must_use]
pub fn arrows_theme() -> Theme {
    Theme {
        header: "< left / > right\n".to_string(),
        highlight_insert: crossterm::style::Stylize::stylize,
        highlight_delete: crossterm::style::Stylize::stylize,
        equal_prefix: " ".to_string(),
        equal_content: crossterm::style::Stylize::stylize,
        delete_prefix: "<".to_string(),
        delete_content: crossterm::style::Stylize::stylize,
        insert_prefix: ">".to_string(),
        insert_line: crossterm::style::Stylize::stylize,
        line_end: "\n".into(),
    }
}

/// A simple colorful theme using arrows
///
/// ```
/// use termdiff::{arrows_color_theme, diff};
/// let old = "The quick brown fox and\njumps over the sleepy dog";
/// let new = "The quick red fox and\njumps over the lazy dog";
/// let mut buffer: Vec<u8> = Vec::new();
/// diff(&mut buffer, old, new, arrows_color_theme()).unwrap();
/// let actual: String = String::from_utf8(buffer).expect("Not valid UTF-8");
///
/// assert_eq!(
///     actual,
///     "\u{1b}[38;5;9m< left\u{1b}[39m / \u{1b}[38;5;10m> right\u{1b}[39m
/// \u{1b}[38;5;9m<\u{1b}[39m\u{1b}[38;5;9mThe quick \u{1b}[39m\u{1b}[38;5;9m\u{1b}[4mbrown\u{1b}[0m\u{1b}[39m\u{1b}[38;5;9m fox and
/// \u{1b}[39m\u{1b}[38;5;9m<\u{1b}[39m\u{1b}[38;5;9mjumps over the \u{1b}[39m\u{1b}[38;5;9m\u{1b}[4msleepy\u{1b}[0m\u{1b}[39m\u{1b}[38;5;9m dog\u{1b}[39m
/// \u{1b}[38;5;10m>\u{1b}[39m\u{1b}[38;5;10mThe quick \u{1b}[39m\u{1b}[38;5;10m\u{1b}[4mred\u{1b}[0m\u{1b}[39m\u{1b}[38;5;10m fox and
/// \u{1b}[39m\u{1b}[38;5;10m>\u{1b}[39m\u{1b}[38;5;10mjumps over the \u{1b}[39m\u{1b}[38;5;10m\u{1b}[4mlazy\u{1b}[0m\u{1b}[39m\u{1b}[38;5;10m dog\u{1b}[39m
/// "
/// );
/// ```
#[must_use]
pub fn arrows_color_theme() -> Theme {
    Theme {
        header: format!("{} / {}\n", "< left".red(), "> right".green()),
        highlight_insert: crossterm::style::Stylize::underlined,
        highlight_delete: crossterm::style::Stylize::underlined,
        equal_prefix: " ".to_string(),
        equal_content: crossterm::style::Stylize::stylize,
        delete_prefix: "<".red().to_string(),
        delete_content: crossterm::style::Stylize::red,
        insert_prefix: ">".green().to_string(),
        insert_line: crossterm::style::Stylize::green,
        line_end: "\n".into(),
    }
}

/// A simple colorless using signs theme
///
/// # Examples
///
/// ```
/// use termdiff::{diff, signs_theme};
/// let old = "The quick brown fox and\njumps over the sleepy dog";
/// let new = "The quick red fox and\njumps over the lazy dog";
/// let mut buffer: Vec<u8> = Vec::new();
/// diff(&mut buffer, old, new, signs_theme()).unwrap();
/// let actual: String = String::from_utf8(buffer).expect("Not valid UTF-8");
///
/// assert_eq!(
///     actual,
///     "--- remove | insert +++
/// -The quick brown fox and
/// -jumps over the sleepy dog
/// +The quick red fox and
/// +jumps over the lazy dog
/// "
/// );
/// ```
#[must_use]
pub fn signs_theme() -> Theme {
    Theme {
        header: "--- remove | insert +++\n".to_string(),
        highlight_insert: crossterm::style::Stylize::stylize,
        highlight_delete: crossterm::style::Stylize::stylize,
        equal_prefix: " ".to_string(),
        equal_content: crossterm::style::Stylize::stylize,
        delete_prefix: "-".to_string(),
        delete_content: crossterm::style::Stylize::stylize,
        insert_prefix: "+".to_string(),
        insert_line: crossterm::style::Stylize::stylize,
        line_end: "\n".into(),
    }
}

/// A simple colorful theme using signs
///
/// ```
/// use termdiff::{diff, signs_color_theme};
/// let old = "The quick brown fox and\njumps over the sleepy dog";
/// let new = "The quick red fox and\njumps over the lazy dog";
/// let mut buffer: Vec<u8> = Vec::new();
/// diff(&mut buffer, old, new, signs_color_theme()).unwrap();
/// let actual: String = String::from_utf8(buffer).expect("Not valid UTF-8");
///
/// assert_eq!(
///     actual,
///     "\u{1b}[38;5;9m--- remove\u{1b}[39m | \u{1b}[38;5;10minsert +++\u{1b}[39m
/// \u{1b}[38;5;9m-\u{1b}[39m\u{1b}[38;5;9mThe quick \u{1b}[39m\u{1b}[38;5;9m\u{1b}[4mbrown\u{1b}[0m\u{1b}[39m\u{1b}[38;5;9m fox and
/// \u{1b}[39m\u{1b}[38;5;9m-\u{1b}[39m\u{1b}[38;5;9mjumps over the \u{1b}[39m\u{1b}[38;5;9m\u{1b}[4msleepy\u{1b}[0m\u{1b}[39m\u{1b}[38;5;9m dog\u{1b}[39m
/// \u{1b}[38;5;10m+\u{1b}[39m\u{1b}[38;5;10mThe quick \u{1b}[39m\u{1b}[38;5;10m\u{1b}[4mred\u{1b}[0m\u{1b}[39m\u{1b}[38;5;10m fox and
/// \u{1b}[39m\u{1b}[38;5;10m+\u{1b}[39m\u{1b}[38;5;10mjumps over the \u{1b}[39m\u{1b}[38;5;10m\u{1b}[4mlazy\u{1b}[0m\u{1b}[39m\u{1b}[38;5;10m dog\u{1b}[39m
/// "
/// );
/// ```
#[must_use]
pub fn signs_color_theme() -> Theme {
    Theme {
        header: format!("{} | {}\n", "--- remove".red(), "insert +++".green()),
        highlight_insert: crossterm::style::Stylize::underlined,
        highlight_delete: crossterm::style::Stylize::underlined,
        equal_prefix: " ".to_string(),
        equal_content: crossterm::style::Stylize::stylize,
        delete_prefix: "-".red().to_string(),
        delete_content: crossterm::style::Stylize::red,
        insert_prefix: "+".green().to_string(),
        insert_line: crossterm::style::Stylize::green,
        line_end: "\n".into(),
    }
}
