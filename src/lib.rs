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
//! * `myers` - Provides a Myers diff algorithm variant. This is a from-scratch
//!   implementation of Eugene Myers' O(ND) difference algorithm (1986) that
//!   does not depend on the `similar` crate. It can be used on its own
//!   (without the `similar` feature) and produces output compatible with the
//!   `similar` feature.
//!
//! * `similar` - Uses the "similar" crate to compute diffs. This is the primary
//!   algorithm implementation.
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
