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

pub use cmd::diff;
pub use themes::{arrows_color_theme, arrows_theme, signs_color_theme, signs_theme, Theme};

mod cmd;
mod draw_diff;
mod themes;
