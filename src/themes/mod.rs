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
mod tests {
    use super::*;
    use std::borrow::Cow;

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
