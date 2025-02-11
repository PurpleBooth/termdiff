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

#![warn(clippy::nursery, clippy::suspicious)]
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
    clippy::style
)]
#![allow(clippy::multiple_crate_versions)]

pub use cmd::diff;
pub use draw_diff::DrawDiff;
pub use themes::{ArrowsColorTheme, ArrowsTheme, SignsColorTheme, SignsTheme, Theme};

mod cmd;
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
