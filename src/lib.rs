//! This library is for helping you create diff for displaying on the terminal
//!
//! # Examples
//!
//! ```
//! use termdiff::{arrows_theme, diff};
//! let old = "Double, double toil and trouble;
//! Fire burn and
//! Caldron bubble.";
//! let new = "Double, double toil and trouble;
//! Fire burn and
//! caldron bubble.
//! Cool it with a baboon's blood,
//! Then the charm is firm and good.";
//! let mut buffer: Vec<u8> = Vec::new();
//! diff(&mut buffer, old, new, arrows_theme()).unwrap();
//! let actual: String = String::from_utf8(buffer).expect("Not valid UTF-8");
//!
//! assert_eq!(
//!     actual,
//!     "< left / > right
//!  Double, double toil and trouble;
//!  Fire burn and
//! <Caldron bubble.
//! >caldron bubble.
//! >Cool it with a baboon's blood,
//! >Then the charm is firm and good.
//! "
//! );
//! ```
//!
//! Alternatively if you are dropping this into a `format!` or similar, you
//! might want to use the displayable instead
//!
//! ```
//! use termdiff::{signs_theme, DrawDiff};
//! let old = "Double, double toil and trouble;
//! Fire burn and
//! Caldron bubble.";
//! let new = "Double, double toil and trouble;
//! Fire burn and
//! caldron bubble.
//! Cool it with a baboon's blood,
//! Then the charm is firm and good.";
//! let actual = format!("{}", DrawDiff::new(old, new, signs_theme()));
//!
//! assert_eq!(
//!     actual,
//!     "--- remove | insert +++
//!  Double, double toil and trouble;
//!  Fire burn and
//! -Caldron bubble.
//! +caldron bubble.
//! +Cool it with a baboon's blood,
//! +Then the charm is firm and good.
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
