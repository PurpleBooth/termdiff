use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
};

use crossterm::style::Stylize;
use similar::{ChangeTag, DiffableStr, TextDiff};

use super::protocol::Output;

type LineFormatter = fn(Cow<str>) -> String;

pub struct Theme {
    equal_line: LineFormatter,
    delete_line: LineFormatter,
    insert_line: LineFormatter,
    line_end: String,
}

fn colorless_theme() -> Theme {
    Theme {
        equal_line: |line| format!(" {}", line),
        delete_line: |line| format!("<{}", line),
        insert_line: |line| format!(">{}", line),
        line_end: "\n".into(),
    }
}
fn color_theme() -> Theme {
    Theme {
        equal_line: |line| format!(" {}", line),
        delete_line: |line| format!("<{}", line).red().to_string(),
        insert_line: |line| format!(">{}", line).green().to_string(),
        line_end: "\n".into(),
    }
}

pub struct Plain<'a, T: DiffableStr + ?Sized> {
    diff: TextDiff<'a, 'a, 'a, T>,
    theme: Theme,
}

impl<'a, T: DiffableStr + ?Sized> Plain<'a, T> {
    pub(crate) fn new(diff: TextDiff<'a, 'a, 'a, T>, theme: Theme) -> Plain<'a, T> {
        Plain { diff, theme }
    }
}

impl<'a, T: DiffableStr + ?Sized> Plain<'a, T> {
    fn format_line(&self, line: Cow<str>, tag: ChangeTag) -> String {
        match tag {
            ChangeTag::Equal => (self.theme.equal_line)(line),
            ChangeTag::Delete => (self.theme.delete_line)(line),
            ChangeTag::Insert => (self.theme.insert_line)(line),
        }
    }
}

impl<'a, T: DiffableStr + ?Sized> Display for Plain<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for change in self.diff.iter_all_changes() {
            for line in change.value().tokenize_lines() {
                write!(
                    f,
                    "{}",
                    self.format_line(line.to_string_lossy(), change.tag())
                )?;
            }

            if change.missing_newline() {
                write!(f, "{}", self.theme.line_end)?;
            }
        }

        Ok(())
    }
}

impl<'a, T: DiffableStr + ?Sized> From<TextDiff<'a, 'a, 'a, T>> for Plain<'a, T> {
    fn from(diff: TextDiff<'a, 'a, 'a, T>) -> Self {
        Self {
            diff,
            theme: colorless_theme(),
        }
    }
}

impl<'a, T: DiffableStr + ?Sized> Output<TextDiff<'a, 'a, 'a, T>> for Plain<'a, T> {}

#[cfg(test)]
mod test {
    use similar::TextDiff;

    use super::color_theme;
    use crate::output::Plain;

    #[test]
    fn single_characters() {
        let old = "a\nb\nc";
        let new = "a\nc\n";
        let actual: Plain<_> = TextDiff::from_lines(old, new).into();

        assert_eq!(format!("{}", actual), " a\n<b\n<c\n>c\n");
    }

    #[test]
    fn one_line() {
        let old = "adc";
        let new = "abc";
        let actual: Plain<_> = TextDiff::from_lines(old, new).into();
        assert_eq!(format!("{}", actual), "<adc\n>abc\n");
    }

    #[test]
    fn line_by_line() {
        let old = "Good Error\nBad Success";
        let new = "Bad Error\nGood Success";
        let actual: Plain<_> = TextDiff::from_lines(old, new).into();
        assert_eq!(
            format!("{}", actual),
            "<Good Error\n<Bad Success\n>Bad Error\n>Good Success\n"
        );
    }

    #[test]
    fn its_customisable() {
        let old = "Good Error\nBad Success";
        let new = "Bad Error\nGood Success";
        let actual: Plain<_> = Plain::new(TextDiff::from_lines(old, new), color_theme());

        assert_eq!(
            format!("{}", actual),
            "\u{1b}[38;5;9m<Good Error\n\u{1b}[39m\u{1b}[38;5;9m<Bad Success\u{1b}[39m\n\u{1b}[38;5;10m>Bad Error\n\u{1b}[39m\u{1b}[38;5;10m>Good Success\u{1b}[39m\n"
        );
    }

    #[test]
    fn word_based_diff() {
        let old = "Good Error\nBad Success";
        let new = "Bad Error\nGood Success";
        let actual: Plain<_> = TextDiff::from_words(old, new).into();
        assert_eq!(
            format!("{}", actual),
            "<Good
>Bad
  
 Error
 
<Bad
>Good
  
 Success
"
        );
    }

    #[test]
    fn letter_based_diff() {
        let old = "adc";
        let new = "abc";
        let actual: Plain<_> = TextDiff::from_chars(old, new).into();
        assert_eq!(
            format!("{}", actual),
            " a
<d
>b
 c
"
        );
    }
}
