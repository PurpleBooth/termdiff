pub mod theme;
pub use theme::Theme;

#[cfg(feature = "arrows")]
pub use self::arrows::ArrowsTheme;

#[cfg(feature = "arrows_color")]
pub use self::arrows_color::ArrowsColorTheme;

#[cfg(feature = "signs")]
pub use self::signs::SignsTheme;

#[cfg(feature = "signs_color")]
pub use self::signs_color::SignsColorTheme;

#[cfg(feature = "arrows")]
pub mod arrows;

#[cfg(feature = "arrows_color")]
pub mod arrows_color;

#[cfg(feature = "signs")]
pub mod signs;

#[cfg(feature = "signs_color")]
pub mod signs_color;

#[cfg(test)]
mod feature_tests {
    /// Test that the `ArrowsTheme` is available when the "arrows" feature is enabled
    ///
    /// This test is only run when the "arrows" feature is enabled.
    #[test]
    #[cfg(feature = "arrows")]
    fn test_arrows_theme_available() {
        use crate::{diff, ArrowsTheme};
        use std::io::Cursor;

        let old = "The quick brown fox";
        let new = "The quick red fox";
        let mut buffer = Cursor::new(Vec::new());
        let theme = ArrowsTheme::default();

        diff(&mut buffer, old, new, &theme).unwrap();

        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");
        // Verify the theme's arrow prefixes are present
        assert!(output.contains("<The quick brown fox"));
        assert!(output.contains(">The quick red fox"));
    }

    /// Test that the `ArrowsColorTheme` is available when the "`arrows_color`" feature is enabled
    ///
    /// This test is only run when the "`arrows_color`" feature is enabled.
    #[test]
    #[cfg(feature = "arrows_color")]
    fn test_arrows_color_theme_available() {
        use crate::{diff, ArrowsColorTheme};
        use std::io::Cursor;

        let old = "The quick brown fox";
        let new = "The quick red fox";
        let mut buffer = Cursor::new(Vec::new());
        let theme = ArrowsColorTheme::default();

        // This should work because the ArrowsColorTheme is available
        diff(&mut buffer, old, new, &theme).unwrap();

        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");
        assert!(output.contains("The quick brown fox"));
        assert!(output.contains("The quick red fox"));
    }

    /// Test that the `SignsTheme` is available when the "signs" feature is enabled
    ///
    /// This test is only run when the "signs" feature is enabled.
    #[test]
    #[cfg(feature = "signs")]
    fn test_signs_theme_available() {
        use crate::{diff, SignsTheme};
        use std::io::Cursor;

        let old = "The quick brown fox";
        let new = "The quick red fox";
        let mut buffer = Cursor::new(Vec::new());
        let theme = SignsTheme::default();

        // This should work because the SignsTheme is available
        diff(&mut buffer, old, new, &theme).unwrap();

        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");
        assert!(output.contains("-The quick brown fox"));
        assert!(output.contains("+The quick red fox"));
    }

    /// Test that the `SignsColorTheme` is available when the "`signs_color`" feature is enabled
    ///
    /// This test is only run when the "`signs_color`" feature is enabled.
    #[test]
    #[cfg(feature = "signs_color")]
    fn test_signs_color_theme_available() {
        use crate::{diff, SignsColorTheme};
        use std::io::Cursor;

        let old = "The quick brown fox";
        let new = "The quick red fox";
        let mut buffer = Cursor::new(Vec::new());
        let theme = SignsColorTheme::default();

        // This should work because the SignsColorTheme is available
        diff(&mut buffer, old, new, &theme).unwrap();

        let output = String::from_utf8(buffer.into_inner()).expect("Not valid UTF-8");
        assert!(output.contains("The quick brown fox"));
        assert!(output.contains("The quick red fox"));
    }
}

#[cfg(test)]
mod tests {
    use crate::{ArrowsColorTheme, ArrowsTheme, SignsColorTheme, SignsTheme, Theme};
    use std::borrow::Cow;

    /// Test that `ArrowsTheme` returns the expected values for all methods
    #[test]
    fn test_arrows_theme_prefixes() {
        let theme = ArrowsTheme::default();
        assert_eq!(theme.equal_prefix(), Cow::Borrowed(" "));
        assert_eq!(theme.delete_prefix(), Cow::Borrowed("<"));
        assert_eq!(theme.insert_prefix(), Cow::Borrowed(">"));
    }

    /// Test that `ArrowsTheme` returns the expected header
    #[test]
    fn test_arrows_theme_header() {
        let theme = ArrowsTheme::default();
        assert_eq!(theme.header(), Cow::Borrowed("< left / > right\n"));
    }

    /// Test that `ArrowsTheme` uses default implementations for content formatting
    #[test]
    fn test_arrows_theme_content_formatting() {
        let theme = ArrowsTheme::default();
        let input = "test";
        assert_eq!(theme.highlight_insert(input), Cow::Borrowed(input));
        assert_eq!(theme.highlight_delete(input), Cow::Borrowed(input));
        assert_eq!(theme.equal_content(input), Cow::Borrowed(input));
        assert_eq!(theme.delete_content(input), Cow::Borrowed(input));
        assert_eq!(theme.insert_line(input), Cow::Borrowed(input));
    }

    /// Test that `ArrowsColorTheme` returns the expected prefixes
    #[test]
    fn test_arrows_color_theme_prefixes() {
        let theme = ArrowsColorTheme::default();
        assert_eq!(theme.equal_prefix(), Cow::Borrowed(" "));
        // Can't directly compare colored strings, so check they contain the expected characters
        assert!(theme.delete_prefix().contains('<'));
        assert!(theme.insert_prefix().contains('>'));
    }

    /// Test that `ArrowsColorTheme` applies highlighting to content
    #[test]
    fn test_arrows_color_theme_highlighting() {
        let theme = ArrowsColorTheme::default();
        let input = "test";
        // Highlighting should modify the input
        assert_ne!(theme.highlight_insert(input), Cow::Borrowed(input));
        assert_ne!(theme.highlight_delete(input), Cow::Borrowed(input));
        assert_ne!(theme.delete_content(input), Cow::Borrowed(input));
        assert_ne!(theme.insert_line(input), Cow::Borrowed(input));
    }

    /// Test that `SignsTheme` returns the expected prefixes
    #[test]
    fn test_signs_theme_prefixes() {
        let theme = SignsTheme::default();
        assert_eq!(theme.equal_prefix(), Cow::Borrowed(" "));
        assert_eq!(theme.delete_prefix(), Cow::Borrowed("-"));
        assert_eq!(theme.insert_prefix(), Cow::Borrowed("+"));
    }

    /// Test that `SignsTheme` returns the expected header
    #[test]
    fn test_signs_theme_header() {
        let theme = SignsTheme::default();
        assert_eq!(theme.header(), Cow::Borrowed("--- remove | insert +++\n"));
    }

    /// Test that `SignsTheme` uses default implementations for line endings and markers
    #[test]
    fn test_signs_theme_defaults() {
        let theme = SignsTheme::default();
        assert_eq!(theme.line_end(), Cow::Borrowed("\n"));
        assert_eq!(theme.trailing_lf_marker(), Cow::Borrowed("␊"));
    }

    /// Test that `SignsColorTheme` returns the expected prefixes
    #[test]
    fn test_signs_color_theme_prefixes() {
        let theme = SignsColorTheme::default();
        assert_eq!(theme.equal_prefix(), Cow::Borrowed(" "));
        // Can't directly compare colored strings, so check they contain the expected characters
        assert!(theme.delete_prefix().contains('-'));
        assert!(theme.insert_prefix().contains('+'));
    }

    /// Test that `SignsColorTheme` applies highlighting to content
    #[test]
    fn test_signs_color_theme_highlighting() {
        let theme = SignsColorTheme::default();
        let input = "test";
        // Highlighting should modify the input
        assert_ne!(theme.highlight_insert(input), Cow::Borrowed(input));
        assert_ne!(theme.highlight_delete(input), Cow::Borrowed(input));
        assert_ne!(theme.delete_content(input), Cow::Borrowed(input));
        assert_ne!(theme.insert_line(input), Cow::Borrowed(input));
    }

    /// Test creating a custom theme with minimal implementation
    #[test]
    fn test_custom_theme_minimal() {
        #[derive(Debug)]
        struct MinimalTheme;

        impl Theme for MinimalTheme {
            fn equal_prefix<'this>(&self) -> Cow<'this, str> {
                "=".into()
            }

            fn delete_prefix<'this>(&self) -> Cow<'this, str> {
                "-".into()
            }

            fn insert_prefix<'this>(&self) -> Cow<'this, str> {
                "+".into()
            }

            fn header<'this>(&self) -> Cow<'this, str> {
                "HEADER\n".into()
            }
        }

        let theme = MinimalTheme;
        assert_eq!(theme.equal_prefix(), Cow::Borrowed("="));
        assert_eq!(theme.delete_prefix(), Cow::Borrowed("-"));
        assert_eq!(theme.insert_prefix(), Cow::Borrowed("+"));
        assert_eq!(theme.header(), Cow::Borrowed("HEADER\n"));

        // Default implementations should be used for other methods
        let input = "test";
        assert_eq!(theme.highlight_insert(input), Cow::Borrowed(input));
        assert_eq!(theme.highlight_delete(input), Cow::Borrowed(input));
        assert_eq!(theme.equal_content(input), Cow::Borrowed(input));
        assert_eq!(theme.delete_content(input), Cow::Borrowed(input));
        assert_eq!(theme.insert_line(input), Cow::Borrowed(input));
        assert_eq!(theme.line_end(), Cow::Borrowed("\n"));
        assert_eq!(theme.trailing_lf_marker(), Cow::Borrowed("␊"));
    }

    /// Test creating a custom theme with full implementation
    #[test]
    fn test_custom_theme_full() {
        #[derive(Debug)]
        struct FullTheme;

        impl Theme for FullTheme {
            fn equal_prefix<'this>(&self) -> Cow<'this, str> {
                "=".into()
            }

            fn delete_prefix<'this>(&self) -> Cow<'this, str> {
                "-".into()
            }

            fn insert_prefix<'this>(&self) -> Cow<'this, str> {
                "+".into()
            }

            fn header<'this>(&self) -> Cow<'this, str> {
                "HEADER\n".into()
            }

            fn highlight_insert<'this>(&self, input: &'this str) -> Cow<'this, str> {
                format!("*{input}*").into()
            }

            fn highlight_delete<'this>(&self, input: &'this str) -> Cow<'this, str> {
                format!("~{input}~").into()
            }

            fn equal_content<'this>(&self, input: &'this str) -> Cow<'this, str> {
                format!("={input}").into()
            }

            fn delete_content<'this>(&self, input: &'this str) -> Cow<'this, str> {
                format!("-{input}").into()
            }

            fn insert_line<'this>(&self, input: &'this str) -> Cow<'this, str> {
                format!("+{input}").into()
            }

            fn line_end<'this>(&self) -> Cow<'this, str> {
                "\r\n".into()
            }

            fn trailing_lf_marker<'this>(&self) -> Cow<'this, str> {
                "[LF]".into()
            }
        }

        let theme = FullTheme;
        let input = "test";

        // All methods should return the custom values
        assert_eq!(theme.equal_prefix(), Cow::Borrowed("="));
        assert_eq!(theme.delete_prefix(), Cow::Borrowed("-"));
        assert_eq!(theme.insert_prefix(), Cow::Borrowed("+"));
        assert_eq!(theme.header(), Cow::Borrowed("HEADER\n"));
        assert_eq!(
            theme.highlight_insert(input),
            Cow::<str>::Owned("*test*".to_string())
        );
        assert_eq!(
            theme.highlight_delete(input),
            Cow::<str>::Owned("~test~".to_string())
        );
        assert_eq!(
            theme.equal_content(input),
            Cow::<str>::Owned("=test".to_string())
        );
        assert_eq!(
            theme.delete_content(input),
            Cow::<str>::Owned("-test".to_string())
        );
        assert_eq!(
            theme.insert_line(input),
            Cow::<str>::Owned("+test".to_string())
        );
        assert_eq!(theme.line_end(), Cow::Borrowed("\r\n"));
        assert_eq!(theme.trailing_lf_marker(), Cow::Borrowed("[LF]"));
    }
}
