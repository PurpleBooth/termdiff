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

#[cfg(test)]
mod availability_tests {
    use crate::{diff_with_algorithm, Algorithm, ArrowsTheme};
    use std::io::Cursor;

    /// Test the `Algorithm::has_available_algorithms` function
    #[test]
    fn test_has_available_algorithms() {
        // This should always be true in a test environment since at least one algorithm should be enabled
        assert!(Algorithm::has_available_algorithms());

        // Get the available algorithms to verify
        let available = Algorithm::available_algorithms();
        assert!(!available.is_empty());

        // Verify that has_available_algorithms matches the emptiness of available_algorithms
        assert_eq!(Algorithm::has_available_algorithms(), !available.is_empty());
    }

    /// Test the `Algorithm::first_available` function
    #[test]
    fn test_first_available() {
        // This should always return Some in a test environment
        let first = Algorithm::first_available();
        assert!(first.is_some());

        // Verify that the first available algorithm is in the list of available algorithms
        let available = Algorithm::available_algorithms();
        assert!(available.contains(&first.unwrap()));
    }


    /// Test that `diff_with_algorithm` correctly handles the case where the requested algorithm is not available
    /// This test specifically targets the condition in the function that checks if the algorithm is available
    #[test]
    fn test_diff_with_algorithm_unavailable_check() {
        let old = "The quick brown fox";
        let new = "The quick red fox";
        let theme = ArrowsTheme::default();

        // Get the available algorithms
        let available = Algorithm::available_algorithms();

        // Only run this test if at least one algorithm is available
        if !available.is_empty() {
            // Choose an algorithm that is not available
            let unavailable_algorithm = if available.contains(&Algorithm::Similar) {
                Algorithm::Myers
            } else {
                Algorithm::Similar
            };

            // If both algorithms are available, we can't test this directly
            if !(available.contains(&Algorithm::Myers) && available.contains(&Algorithm::Similar)) {
                // Get the expected output using the first available algorithm
                let mut expected_buffer = Cursor::new(Vec::new());
                let available_algorithm = Algorithm::first_available().unwrap();
                diff_with_algorithm(&mut expected_buffer, old, new, &theme, available_algorithm)
                    .unwrap();
                let expected_output =
                    String::from_utf8(expected_buffer.into_inner()).expect("Not valid UTF-8");

                // Now try with the unavailable algorithm
                let mut actual_buffer = Cursor::new(Vec::new());
                diff_with_algorithm(&mut actual_buffer, old, new, &theme, unavailable_algorithm)
                    .unwrap();
                let actual_output =
                    String::from_utf8(actual_buffer.into_inner()).expect("Not valid UTF-8");

                // The function should fall back to using the available algorithm
                // So the outputs should be the same
                assert_eq!(actual_output, expected_output);

                // Also verify that the output contains the expected diff information
                assert!(actual_output.contains("The quick brown fox"));
                assert!(actual_output.contains("The quick red fox"));
            }
        }
    }

    /// Test that `diff_with_algorithm` correctly uses the requested algorithm when available
    #[test]
    fn test_diff_with_algorithm_uses_requested() {
        let old = "The quick brown fox";
        let new = "The quick red fox";
        let theme = ArrowsTheme::default();

        // Get the available algorithms
        let available = Algorithm::available_algorithms();

        // Only run this test if at least one algorithm is available
        if !available.is_empty() {
            let algorithm = available[0]; // Use the first available algorithm

            let mut buffer = Cursor::new(Vec::new());
            diff_with_algorithm(&mut buffer, old, new, &theme, algorithm).unwrap();

            let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");

            // The output should contain the diff information
            assert!(output.contains("The quick brown fox"));
            assert!(output.contains("The quick red fox"));
        }
    }

    /// Test that diff_with_algorithm handles the case where no algorithms are available
    #[test]
    #[cfg(not(any(feature = "myers", feature = "similar")))]
    fn test_diff_with_algorithm_no_algorithms() {
        let old = "The quick brown fox";
        let new = "The quick red fox";
        let mut buffer = Cursor::new(Vec::new());
        let theme = ArrowsTheme::default();

        diff_with_algorithm(&mut buffer, old, new, &theme, Algorithm::Myers).unwrap();

        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");

        // Should contain an error message
        assert!(output.contains("Error: No diff algorithms are available"));
    }
}

// Re-export the common types and traits
pub use common::{Algorithm, ChangeTag, DiffAlgorithm};

// Re-export the algorithm implementations
#[cfg(feature = "myers")]
pub use myers::MyersDiff;
#[cfg(feature = "similar")]
pub use similar::SimilarDiff;

// Re-export the factory
pub use factory::DiffAlgorithmFactory;
