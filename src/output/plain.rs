use std::fmt::{Display, Formatter};

use similar::{ChangeTag, DiffableStr, TextDiff};

use super::protocol::Output;

pub struct Plain<'a, T: DiffableStr + ?Sized> {
    diff: TextDiff<'a, 'a, 'a, T>,
    equal_sign: String,
    delete_sign: String,
    insert_sign: String,
}

impl<'a, T: DiffableStr + ?Sized> Plain<'a, T> {
    fn prefix(&self, tag: ChangeTag) -> String {
        match tag {
            ChangeTag::Equal => self.equal_sign.clone(),
            ChangeTag::Delete => self.delete_sign.clone(),
            ChangeTag::Insert => self.insert_sign.clone(),
        }
    }
}

impl<T: DiffableStr + ?Sized> Display for Plain<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for change in self.diff.iter_all_changes() {
            for line in change.value().tokenize_lines() {
                write!(f, "{}{}", self.prefix(change.tag()), line.to_string_lossy())?;
            }

            if change.missing_newline() {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl<'a, T: DiffableStr + ?Sized> From<TextDiff<'a, 'a, 'a, T>> for Plain<'a, T> {
    fn from(diff: TextDiff<'a, 'a, 'a, T>) -> Self {
        Self {
            diff,
            equal_sign: " ".to_string(),
            insert_sign: ">".to_string(),
            delete_sign: "<".to_string(),
        }
    }
}

impl<'a, T: DiffableStr + ?Sized> Output<TextDiff<'a, 'a, 'a, T>> for Plain<'a, T> {}

#[cfg(test)]
mod test {
    use similar::TextDiff;

    use crate::output::plain::Plain;

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
