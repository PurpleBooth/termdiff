//! Module for diff algorithm implementations
//!
//! This module contains the implementations of different diff algorithms,
//! as well as the common types and traits used by them.

pub mod common;
pub mod factory;
#[cfg(feature = "myers")]
pub mod myers;
#[cfg(feature = "similar")]
pub mod similar;
#[cfg(test)]
pub mod tests;

// Re-export the common types and traits
pub use common::{Algorithm, ChangeTag, DiffAlgorithm};

// Re-export the algorithm implementations
#[cfg(feature = "myers")]
pub use myers::MyersDiff;
#[cfg(feature = "similar")]
pub use similar::SimilarDiff;

// Re-export the factory
pub use factory::DiffAlgorithmFactory;

#[cfg(test)]
mod feature_tests {
    /// Test that the application works with only the Myers algorithm
    ///
    /// This test is only run when the "myers" feature is enabled and the "similar" feature is disabled.
    #[test]
    #[cfg(all(feature = "myers", not(feature = "similar")))]
    fn test_only_myers_algorithm() {
        use crate::{diff_with_algorithm, Algorithm, ArrowsTheme};
        use std::io::Cursor;

        let old = "The quick brown fox";
        let new = "The quick red fox";
        let mut buffer = Cursor::new(Vec::new());
        let theme = ArrowsTheme::default();

        // This should work because the Myers algorithm is available
        diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Myers).unwrap();

        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");
        assert!(output.contains("<The quick brown fox"));
        assert!(output.contains(">The quick red fox"));

        // Now try with the Similar algorithm, which should fall back to Myers
        let mut buffer = Cursor::new(Vec::new());
        diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Similar).unwrap();

        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");
        assert!(output.contains("<The quick brown fox"));
        assert!(output.contains(">The quick red fox"));
    }

    /// Test that the application works with only the Similar algorithm
    ///
    /// This test is only run when the "similar" feature is enabled and the "myers" feature is disabled.
    #[test]
    #[cfg(all(feature = "similar", not(feature = "myers")))]
    fn test_only_similar_algorithm() {
        use crate::{diff_with_algorithm, Algorithm, ArrowsTheme};
        use std::io::Cursor;

        let old = "The quick brown fox";
        let new = "The quick red fox";
        let mut buffer = Cursor::new(Vec::new());
        let theme = ArrowsTheme::default();

        // This should work because the Similar algorithm is available
        diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Similar).unwrap();

        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");
        assert!(output.contains("<The quick brown fox"));
        assert!(output.contains(">The quick red fox"));

        // Now try with the Myers algorithm, which should fall back to Similar
        let mut buffer = Cursor::new(Vec::new());
        diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Myers).unwrap();

        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");
        assert!(output.contains("<The quick brown fox"));
        assert!(output.contains(">The quick red fox"));
    }

    /// Test that the application produces a sensible error when no algorithms are available
    ///
    /// This test is only run when both the "myers" and "similar" features are disabled.
    #[test]
    #[cfg(not(any(feature = "myers", feature = "similar")))]
    fn test_no_algorithms_available() {
        use crate::{diff, diff_with_algorithm, Algorithm, ArrowsTheme};
        use std::io::Cursor;

        let old = "The quick brown fox";
        let new = "The quick red fox";
        let mut buffer = Cursor::new(Vec::new());
        let theme = ArrowsTheme::default();

        // This should still work but produce an error message
        diff(&mut buffer, old, new, &theme).unwrap();

        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");
        assert!(output.contains("Error: No diff algorithms are available"));

        // Try with diff_with_algorithm as well
        let mut buffer = Cursor::new(Vec::new());
        diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Myers).unwrap();

        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");
        assert!(output.contains("Error: No diff algorithms are available"));
    }
}
