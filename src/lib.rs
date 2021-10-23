//! This library is for helping you create diff for displaying on the terminal
//!
//! # Examples
//!
//! ```
//! use termdiff::{arrows_theme, diff};
//! let old = "The quick brown fox and\njumps over the sleepy dog";
//! let new = "The quick red fox and\njumps over the lazy dog";
//! let mut buffer: Vec<u8> = Vec::new();
//! diff(&mut buffer, old, new, arrows_theme()).unwrap();
//! let actual: String = String::from_utf8(buffer).expect("Not valid UTF-8");
//!
//! assert_eq!(
//!     actual,
//!     "< left / > right
//! <The quick brown fox and
//! <jumps over the sleepy dog
//! >The quick red fox and
//! >jumps over the lazy dog
//! "
//! );
//! ```
//!
//! Alternatively if you are dropping this into a `format!` or similar, you
//! might want to use the displayable instead
//!
//! ```
//! use termdiff::{signs_theme, DrawDiff};
//! let old = "The quick brown fox and\njumps over the sleepy dog";
//! let new = "The quick red fox and\njumps over the lazy dog";
//! let actual = format!("{}", DrawDiff::new(old, new, signs_theme()));
//!
//! assert_eq!(
//!     actual,
//!     "--- remove | insert +++
//! -The quick brown fox and
//! -jumps over the sleepy dog
//! +The quick red fox and
//! +jumps over the lazy dog
//! "
//! );
//! ```
//!
//! You can define your own theme if you like
//!
//!
//! ``` rust
//! use termdiff::DrawDiff;
//! use termdiff::Theme;
//! use crossterm::style::Stylize;
//!
//! let my_theme = Theme {
//! header: format!("{}\n", "Header"),
//! highlight_insert: crossterm::style::Stylize::stylize,
//! highlight_delete: crossterm::style::Stylize::stylize,
//! equal_prefix: "=".to_string(),
//! equal_content: crossterm::style::Stylize::stylize,
//! delete_prefix: "!".to_string(),
//! delete_content: crossterm::style::Stylize::stylize,
//! insert_prefix: "|".to_string(),
//! insert_line: crossterm::style::Stylize::stylize,
//! line_end: "\n".into(),
//! };
//!
//! let old = "The quick brown fox and\njumps over the sleepy dog";
//! let new = "The quick red fox and\njumps over the lazy dog";
//! let actual = format!("{}", DrawDiff::new(old, new, my_theme));
//!
//! assert_eq!(
//!     actual,
//!     "Header
//! !The quick brown fox and
//! !jumps over the sleepy dog
//! |The quick red fox and
//! |jumps over the lazy dog
//! "
//! );
//! ```

mod cmd;
mod draw_diff;
mod themes;

pub use cmd::diff;
pub use draw_diff::DrawDiff;
pub use themes::{arrows_color_theme, arrows_theme, signs_color_theme, signs_theme, Theme};

#[cfg(doctest)]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            extern "C" {}
        };
    }

    external_doc_test!(include_str!("../README.md"));
}
