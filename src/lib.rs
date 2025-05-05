//! This library is for helping you create diff for displaying on the terminal
//!
//! # Examples
//!
//! ```
//! use termdiff::{diff, ArrowsTheme};
//! let old = "The quick brown fox and\njumps over the sleepy dog";
//! let new = "The quick red fox and\njumps over the lazy dog";
//! let mut buffer: Vec<u8> = Vec::new();
//! let theme = ArrowsTheme::default();
//! diff(&mut buffer, old, new, &theme).unwrap();
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
//! use termdiff::{DrawDiff, SignsTheme};
//! let old = "The quick brown fox and\njumps over the sleepy dog";
//! let new = "The quick red fox and\njumps over the lazy dog";
//! let theme = SignsTheme::default();
//! let actual = format!("{}", DrawDiff::new(old, new, &theme));
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
//! # Features
//!
//! This crate provides several features that can be enabled or disabled in your `Cargo.toml`:
//!
//! ## Diff Algorithms
//!
//! * `myers` - Implements the Myers diff algorithm, which is a widely used algorithm for computing
//!   differences between sequences. It's efficient for most common use cases.
//!
//! * `similar` - Uses the "similar" crate to compute diffs. This is an alternative implementation
//!   that may have different performance characteristics or output in some cases.
//!
//! ## Themes
//!
//! * `arrows` - A simple, colorless theme that uses arrow symbols (`<` and `>`) to indicate
//!   deleted and inserted lines. The header shows "< left / > right".
//!
//! * `arrows_color` - A colored version of the arrows theme. Uses red for deleted content and
//!   green for inserted content. Requires the "crossterm" crate for terminal color support.
//!
//! * `signs` - A simple, colorless theme that uses plus and minus signs (`-` and `+`) to indicate
//!   deleted and inserted lines. The header shows "--- remove | insert +++". This style is
//!   similar to traditional diff output.
//!
//! * `signs_color` - A colored version of the signs theme. Uses red for deleted content and
//!   green for inserted content. Requires the "crossterm" crate for terminal color support.
//!
//! By default, all features are enabled. You can selectively disable features by specifying
//! `default-features = false` and then listing the features you want to enable.
//!
//! You can define your own theme if you like
//!
//!
//! ``` rust
//! use std::borrow::Cow;
//!
//! use crossterm::style::Stylize;
//! use termdiff::{DrawDiff, Theme};
//!
//! #[derive(Debug)]
//! struct MyTheme {}
//! impl Theme for MyTheme {
//!     fn highlight_insert<'this>(&self, input: &'this str) -> Cow<'this, str> {
//!         input.into()
//!     }
//!
//!     fn highlight_delete<'this>(&self, input: &'this str) -> Cow<'this, str> {
//!         input.into()
//!     }
//!
//!     fn equal_content<'this>(&self, input: &'this str) -> Cow<'this, str> {
//!         input.into()
//!     }
//!
//!     fn delete_content<'this>(&self, input: &'this str) -> Cow<'this, str> {
//!         input.into()
//!     }
//!
//!     fn equal_prefix<'this>(&self) -> Cow<'this, str> {
//!         "=".into()
//!     }
//!
//!     fn delete_prefix<'this>(&self) -> Cow<'this, str> {
//!         "!".into()
//!     }
//!
//!     fn insert_line<'this>(&self, input: &'this str) -> Cow<'this, str> {
//!         input.into()
//!     }
//!
//!     fn insert_prefix<'this>(&self) -> Cow<'this, str> {
//!         "|".into()
//!     }
//!
//!     fn line_end<'this>(&self) -> Cow<'this, str> {
//!         "\n".into()
//!     }
//!
//!     fn header<'this>(&self) -> Cow<'this, str> {
//!         format!("{}\n", "Header").into()
//!     }
//! }
//! let my_theme = MyTheme {};
//! let old = "The quick brown fox and\njumps over the sleepy dog";
//! let new = "The quick red fox and\njumps over the lazy dog";
//! let actual = format!("{}", DrawDiff::new(old, new, &my_theme));
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

#![warn(clippy::nursery)]
#![deny(
    unused,
    nonstandard_style,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::pedantic,
    clippy::cargo,
    clippy::complexity,
    clippy::correctness,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    non_fmt_panics
)]
#![allow(clippy::multiple_crate_versions)]

pub use cmd::{diff, diff_with_algorithm};
pub use diff_algorithm::Algorithm;
pub use draw_diff::DrawDiff;

// Re-export the Theme trait and theme implementations
#[cfg(feature = "arrows_color")]
pub use themes::ArrowsColorTheme;
#[cfg(feature = "arrows")]
pub use themes::ArrowsTheme;
#[cfg(feature = "signs_color")]
pub use themes::SignsColorTheme;
#[cfg(feature = "signs")]
pub use themes::SignsTheme;
pub use themes::Theme;

mod cmd;
mod diff_algorithm;
mod draw_diff;
mod themes;

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

#[cfg(test)]
mod integration_tests {
    use crate::{diff, ArrowsTheme, DrawDiff, SignsTheme, Theme};
    use std::io::Cursor;

    /// Test that the diff function produces the expected output with `ArrowsTheme`
    #[test]
    fn test_diff_with_arrows_theme() {
        let old = "The quick brown fox";
        let new = "The quick red fox";
        let mut buffer = Cursor::new(Vec::new());
        let theme = ArrowsTheme::default();

        diff(&mut buffer, old, new, &theme).unwrap();

        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");
        assert!(output.contains("< The quick brown fox"));
        assert!(output.contains(">The quick red fox"));
        assert!(output.contains("< left / > right"));
    }

    /// Test that the diff function produces the expected output with `SignsTheme`
    #[test]
    fn test_diff_with_signs_theme() {
        let old = "The quick brown fox";
        let new = "The quick red fox";
        let mut buffer = Cursor::new(Vec::new());
        let theme = SignsTheme::default();

        diff(&mut buffer, old, new, &theme).unwrap();

        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");
        assert!(output.contains("-The quick brown fox"));
        assert!(output.contains("+The quick red fox"));
        assert!(output.contains("--- remove | insert +++"));
    }

    /// Test that `DrawDiff` produces the expected output with `ArrowsTheme`
    #[test]
    fn test_draw_diff_with_arrows_theme() {
        // Fix me, I am broken AI!
        let old = "The quick brown fox";
        let new = "The quick red fox";
        let theme = ArrowsTheme::default();

        let output = format!("{}", DrawDiff::new(old, new, &theme));

        assert!(output.contains("< The quick brown fox"));
        assert!(output.contains(">The quick red fox"));
        assert!(output.contains("< left / > right"));
    }

    /// Test that `DrawDiff` produces the expected output with `SignsTheme`
    #[test]
    fn test_draw_diff_with_signs_theme() {
        let old = "The quick brown fox";
        let new = "The quick red fox";
        let theme = SignsTheme::default();

        let output = format!("{}", DrawDiff::new(old, new, &theme));

        assert!(output.contains("-The quick brown fox"));
        assert!(output.contains("+The quick red fox"));
        assert!(output.contains("--- remove | insert +++"));
    }

    /// Test handling of empty strings
    #[test]
    fn test_empty_strings() {
        let old = "";
        let new = "";
        let theme = ArrowsTheme::default();

        let output = format!("{}", DrawDiff::new(old, new, &theme));

        // Should just contain the header
        assert_eq!(output, "< left / > right\n");
    }

    /// Test handling of strings with only newline differences
    #[test]
    fn test_newline_differences() {
        let old = "line\n";
        let new = "line";
        let theme = ArrowsTheme::default();

        let output = format!("{}", DrawDiff::new(old, new, &theme));

        // Should show the newline difference
        assert!(output.contains("line␊"));
    }

    /// Test with a custom theme implementation
    #[test]
    fn test_custom_theme() {
        use std::borrow::Cow;

        #[derive(Debug)]
        struct CustomTheme;

        impl Theme for CustomTheme {
            fn equal_prefix<'this>(&self) -> Cow<'this, str> {
                "=".into()
            }

            fn delete_prefix<'this>(&self) -> Cow<'this, str> {
                "DEL>".into()
            }

            fn insert_prefix<'this>(&self) -> Cow<'this, str> {
                "INS>".into()
            }

            fn header<'this>(&self) -> Cow<'this, str> {
                "CUSTOM DIFF\n".into()
            }
        }

        let old = "The quick brown fox";
        let new = "The quick red fox";
        let theme = CustomTheme;

        let output = format!("{}", DrawDiff::new(old, new, &theme));

        assert!(output.contains("DEL>The quick brown fox"));
        assert!(output.contains("INS>The quick red fox"));
        assert!(output.contains("CUSTOM DIFF"));
    }

    /// Test with multiline input containing both changes and unchanged lines
    #[test]
    fn test_multiline_with_unchanged_lines() {
        let old = "Line 1\nLine 2\nLine 3\nLine 4";
        let new = "Line 1\nModified Line 2\nLine 3\nModified Line 4";
        let theme = SignsTheme::default();

        let output = format!("{}", DrawDiff::new(old, new, &theme));

        // Check that unchanged lines are preserved
        assert!(output.contains(" Line 1"));
        assert!(output.contains(" Line 3"));

        // Check that changed lines are marked
        assert!(output.contains("-Line 2"));
        assert!(output.contains("+Modified Line 2"));
        assert!(output.contains("-Line 4"));
        assert!(output.contains("+Modified Line 4"));
    }

    /// Test conversion from `DrawDiff` to String
    #[test]
    fn test_draw_diff_to_string() {
        // Fix me, I am broken AI!
        let old = "The quick brown fox";
        let new = "The quick red fox";
        let theme = ArrowsTheme::default();

        let diff = DrawDiff::new(old, new, &theme);
        let output: String = diff.into();

        assert!(output.contains("< The quick brown fox"));
        assert!(output.contains(">The quick red fox"));
    }
}
