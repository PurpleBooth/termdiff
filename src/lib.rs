use std::io::Write;

use crossterm::style::Stylize;
use similar::{Change, ChangeTag, TextDiff};

#[derive(Clone, Copy)]
pub enum Config {
    Plain,
    Color,
}

/// # Errors
///
/// Errors on write failure
pub fn line_by_line(
    w: &mut dyn Write,
    old: &str,
    new: &str,
    config: Config,
) -> std::io::Result<()> {
    let mut deltas = vec![];

    for change in TextDiff::from_lines(old, new).iter_all_changes() {
        let old_line = change.old_index().and_then(|index| new.lines().nth(index));
        let new_line = change.new_index().and_then(|index| old.lines().nth(index));
        match change.tag() {
            ChangeTag::Equal => deltas.push(change_equal(&change)),
            ChangeTag::Delete => deltas.push(change_delete(
                &change,
                config,
                old_line.map(|old_line| (change.value(), old_line)),
            )),
            ChangeTag::Insert => deltas.push(change_insert(
                &change,
                config,
                new_line.map(|new_line| (new_line, change.value())),
            )),
        }
    }

    deltas
        .last_mut()
        .filter(|x| x.ends_with('\n') && !old.ends_with('\n') && !new.ends_with('\n'))
        .map(std::string::String::pop);

    write!(w, "{}", deltas.into_iter().collect::<String>())
}

fn change_equal(change: &Change<&str>) -> String {
    let text = format!(" {}", change.value());
    let newline = if change.missing_newline() { "\n" } else { "" };

    format!("{}{}", text, newline)
}

fn change_delete(change: &Change<&str>, config: Config, diff: Option<(&str, &str)>) -> String {
    let anchor = "<";
    let text = change.value().to_string();
    let newline = if change.missing_newline() { "\n" } else { "" };

    match (config, diff) {
        (Config::Plain, _) => {
            format!("{}{}{}", anchor, text, newline)
        }

        (Config::Color, None) => {
            format!("{}{}{}", anchor.red(), text.red(), newline)
        }
        (Config::Color, Some((old, new))) => {
            let highlighted = TextDiff::from_chars(old, new)
                .iter_all_changes()
                .filter(|change| !matches!(change.tag(), ChangeTag::Insert))
                .map(|change| match change.tag() {
                    ChangeTag::Equal => change.value().red().to_string(),
                    ChangeTag::Insert | ChangeTag::Delete => {
                        change.value().red().underlined().to_string()
                    }
                })
                .collect::<String>();

            format!("{}{}{}", anchor.red(), highlighted.red(), newline)
        }
    }
}

fn change_insert(change: &Change<&str>, config: Config, diff: Option<(&str, &str)>) -> String {
    let anchor = ">";
    let text = change.value();
    let newline = if change.missing_newline() { "\n" } else { "" };

    match (config, diff) {
        (Config::Plain, _) => {
            format!("{}{}{}", anchor, text, newline)
        }

        (Config::Color, None) => {
            format!("{}{}{}", anchor, text.green(), newline)
        }
        (Config::Color, Some((old, new))) => {
            let highlighted = TextDiff::from_chars(old, new)
                .iter_all_changes()
                .filter(|change| !matches!(change.tag(), ChangeTag::Delete))
                .map(|change| match change.tag() {
                    ChangeTag::Equal => change.value().green().to_string(),
                    ChangeTag::Delete | ChangeTag::Insert => {
                        change.value().green().underlined().to_string()
                    }
                })
                .collect::<String>();

            format!("{}{}{}", anchor.green(), highlighted.green(), newline)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn single_characters() {
        let old = "a\nb\nc";
        let new = "a\nc\n";
        let mut buffer: Vec<u8> = Vec::new();
        super::line_by_line(&mut buffer, old, new, super::Config::Plain).unwrap();
        let actual: String = String::from_utf8(buffer).expect("Not valid UTF-8");

        assert_eq!(actual, " a\n<b\n<c\n>c\n");
    }

    #[test]
    fn color_single_characters() {
        let old = "a\nb\nc";
        let new = "a\nc\n";
        let mut buffer: Vec<u8> = Vec::new();
        super::line_by_line(&mut buffer, old, new, super::Config::Color).unwrap();
        let actual: String = String::from_utf8(buffer).expect("Not valid UTF-8");
        assert_eq!(
            actual,
            " a
\u{1b}[38;5;9m<\u{1b}[39m\u{1b}[38;5;9m\u{1b}[38;5;9m\u{1b}[4mb\u{1b}[0m\u{1b}[38;5;9m\u{1b}[4m
\u{1b}[0m\u{1b}[39m\u{1b}[38;5;9m<\u{1b}[39m\u{1b}[38;5;9mc\u{1b}[39m
\u{1b}[38;5;10m>\u{1b}[39m\u{1b}[38;5;10m\u{1b}[38;5;10m\u{1b}[4mc\u{1b}[0m\u{1b}[38;5;10m\u{1b}[4m
\u{1b}[0m\u{1b}[39m"
        );
    }

    #[test]
    fn oneline() {
        let old = "adc";
        let new = "abc";
        let mut buffer: Vec<u8> = Vec::new();
        super::line_by_line(&mut buffer, old, new, super::Config::Plain).unwrap();
        let actual: String = String::from_utf8(buffer).expect("Not valid UTF-8");

        assert_eq!(actual, "<adc\n>abc");
    }

    #[test]
    fn oneline_color() {
        let old = "abc";
        let new = "adc";
        let mut buffer: Vec<u8> = Vec::new();
        super::line_by_line(&mut buffer, old, new, super::Config::Color).unwrap();
        let actual: String = String::from_utf8(buffer).expect("Not valid UTF-8");

        assert_eq!(
            actual,
            "\u{1b}[38;5;9m<\u{1b}[39m\u{1b}[38;5;9m\u{1b}[38;5;9ma\u{1b}[39m\u{1b}[38;5;9m\u{1b}[4mb\u{1b}[0m\u{1b}[38;5;9mc\u{1b}[39m\u{1b}[39m
\u{1b}[38;5;10m>\u{1b}[39m\u{1b}[38;5;10m\u{1b}[38;5;10ma\u{1b}[39m\u{1b}[38;5;10m\u{1b}[4md\u{1b}[0m\u{1b}[38;5;10mc\u{1b}[39m\u{1b}[39m"
        );
    }

    #[test]
    fn line_by_line() {
        let old = "Good Error\nBad Success";
        let new = "Bad Error\nGood Success";
        let mut buffer: Vec<u8> = Vec::new();
        super::line_by_line(&mut buffer, old, new, super::Config::Plain).unwrap();
        let actual: String = String::from_utf8(buffer).expect("Not valid UTF-8");

        assert_eq!(
            actual,
            "<Good Error\n<Bad Success\n>Bad Error\n>Good Success"
        );
    }

    #[test]
    fn color_line_by_line() {
        let old = "Good Error\nBad Success";
        let new = "Bad Error\nGood Success";
        let mut buffer: Vec<u8> = Vec::new();
        super::line_by_line(&mut buffer, old, new, super::Config::Color).unwrap();
        let actual: String = String::from_utf8(buffer).expect("Not valid UTF-8");

        assert_eq!(
            actual,
            "\u{1b}[38;5;9m<\u{1b}[39m\u{1b}[38;5;9m\u{1b}[38;5;9m\u{1b}[4mG\u{1b}[0m\u{1b}[38;5;9m\u{1b}[4mo\u{1b}[0m\u{1b}[38;5;9m\u{1b}[4mo\u{1b}[0m\u{1b}[38;5;9md\u{1b}[39m\u{1b}[38;5;9m \u{1b}[39m\u{1b}[38;5;9mE\u{1b}[39m\u{1b}[38;5;9mr\u{1b}[39m\u{1b}[38;5;9mr\u{1b}[39m\u{1b}[38;5;9mo\u{1b}[39m\u{1b}[38;5;9mr\u{1b}[39m\u{1b}[38;5;9m\u{1b}[4m
\u{1b}[0m\u{1b}[39m\u{1b}[38;5;9m<\u{1b}[39m\u{1b}[38;5;9m\u{1b}[38;5;9m\u{1b}[4mB\u{1b}[0m\u{1b}[38;5;9m\u{1b}[4ma\u{1b}[0m\u{1b}[38;5;9md\u{1b}[39m\u{1b}[38;5;9m \u{1b}[39m\u{1b}[38;5;9mS\u{1b}[39m\u{1b}[38;5;9mu\u{1b}[39m\u{1b}[38;5;9mc\u{1b}[39m\u{1b}[38;5;9mc\u{1b}[39m\u{1b}[38;5;9me\u{1b}[39m\u{1b}[38;5;9ms\u{1b}[39m\u{1b}[38;5;9ms\u{1b}[39m\u{1b}[39m
\u{1b}[38;5;10m>\u{1b}[39m\u{1b}[38;5;10m\u{1b}[38;5;10m\u{1b}[4mB\u{1b}[0m\u{1b}[38;5;10m\u{1b}[4ma\u{1b}[0m\u{1b}[38;5;10md\u{1b}[39m\u{1b}[38;5;10m \u{1b}[39m\u{1b}[38;5;10mE\u{1b}[39m\u{1b}[38;5;10mr\u{1b}[39m\u{1b}[38;5;10mr\u{1b}[39m\u{1b}[38;5;10mo\u{1b}[39m\u{1b}[38;5;10mr\u{1b}[39m\u{1b}[38;5;10m\u{1b}[4m
\u{1b}[0m\u{1b}[39m\u{1b}[38;5;10m>\u{1b}[39m\u{1b}[38;5;10m\u{1b}[38;5;10m\u{1b}[4mG\u{1b}[0m\u{1b}[38;5;10m\u{1b}[4mo\u{1b}[0m\u{1b}[38;5;10m\u{1b}[4mo\u{1b}[0m\u{1b}[38;5;10md\u{1b}[39m\u{1b}[38;5;10m \u{1b}[39m\u{1b}[38;5;10mS\u{1b}[39m\u{1b}[38;5;10mu\u{1b}[39m\u{1b}[38;5;10mc\u{1b}[39m\u{1b}[38;5;10mc\u{1b}[39m\u{1b}[38;5;10me\u{1b}[39m\u{1b}[38;5;10ms\u{1b}[39m\u{1b}[38;5;10ms\u{1b}[39m\u{1b}[39m"
        );
    }

    #[test]
    fn richer_color_error() {
        let old = "concensus\nblue\nequiptment\ngreen\nindependant\npurple\nthe";
        let new = "consensus\nblue\nequipment\ngreen\nindependent\npurple\nthese";
        let mut buffer: Vec<u8> = Vec::new();
        super::line_by_line(&mut buffer, old, new, super::Config::Color).unwrap();
        let actual: String = String::from_utf8(buffer).expect("Not valid UTF-8");

        assert_eq!(
            actual,
            "\u{1b}[38;5;9m<\u{1b}[39m\u{1b}[38;5;9m\u{1b}[38;5;9mc\u{1b}[39m\u{1b}[38;5;9mo\u{1b}[39m\u{1b}[38;5;9mn\u{1b}[39m\u{1b}[38;5;9m\u{1b}[4mc\u{1b}[0m\u{1b}[38;5;9me\u{1b}[39m\u{1b}[38;5;9mn\u{1b}[39m\u{1b}[38;5;9ms\u{1b}[39m\u{1b}[38;5;9mu\u{1b}[39m\u{1b}[38;5;9ms\u{1b}[39m\u{1b}[38;5;9m\u{1b}[4m
\u{1b}[0m\u{1b}[39m\u{1b}[38;5;10m>\u{1b}[39m\u{1b}[38;5;10m\u{1b}[38;5;10mc\u{1b}[39m\u{1b}[38;5;10mo\u{1b}[39m\u{1b}[38;5;10mn\u{1b}[39m\u{1b}[38;5;10m\u{1b}[4ms\u{1b}[0m\u{1b}[38;5;10me\u{1b}[39m\u{1b}[38;5;10mn\u{1b}[39m\u{1b}[38;5;10ms\u{1b}[39m\u{1b}[38;5;10mu\u{1b}[39m\u{1b}[38;5;10ms\u{1b}[39m\u{1b}[38;5;10m\u{1b}[4m
\u{1b}[0m\u{1b}[39m blue
\u{1b}[38;5;9m<\u{1b}[39m\u{1b}[38;5;9m\u{1b}[38;5;9me\u{1b}[39m\u{1b}[38;5;9mq\u{1b}[39m\u{1b}[38;5;9mu\u{1b}[39m\u{1b}[38;5;9mi\u{1b}[39m\u{1b}[38;5;9mp\u{1b}[39m\u{1b}[38;5;9m\u{1b}[4mt\u{1b}[0m\u{1b}[38;5;9mm\u{1b}[39m\u{1b}[38;5;9me\u{1b}[39m\u{1b}[38;5;9mn\u{1b}[39m\u{1b}[38;5;9mt\u{1b}[39m\u{1b}[38;5;9m\u{1b}[4m
\u{1b}[0m\u{1b}[39m\u{1b}[38;5;10m>\u{1b}[39m\u{1b}[38;5;10m\u{1b}[38;5;10me\u{1b}[39m\u{1b}[38;5;10mq\u{1b}[39m\u{1b}[38;5;10mu\u{1b}[39m\u{1b}[38;5;10mi\u{1b}[39m\u{1b}[38;5;10mp\u{1b}[39m\u{1b}[38;5;10mm\u{1b}[39m\u{1b}[38;5;10me\u{1b}[39m\u{1b}[38;5;10mn\u{1b}[39m\u{1b}[38;5;10mt\u{1b}[39m\u{1b}[38;5;10m\u{1b}[4m
\u{1b}[0m\u{1b}[39m green
\u{1b}[38;5;9m<\u{1b}[39m\u{1b}[38;5;9m\u{1b}[38;5;9mi\u{1b}[39m\u{1b}[38;5;9mn\u{1b}[39m\u{1b}[38;5;9md\u{1b}[39m\u{1b}[38;5;9me\u{1b}[39m\u{1b}[38;5;9mp\u{1b}[39m\u{1b}[38;5;9me\u{1b}[39m\u{1b}[38;5;9mn\u{1b}[39m\u{1b}[38;5;9md\u{1b}[39m\u{1b}[38;5;9m\u{1b}[4ma\u{1b}[0m\u{1b}[38;5;9mn\u{1b}[39m\u{1b}[38;5;9mt\u{1b}[39m\u{1b}[38;5;9m\u{1b}[4m
\u{1b}[0m\u{1b}[39m\u{1b}[38;5;10m>\u{1b}[39m\u{1b}[38;5;10m\u{1b}[38;5;10mi\u{1b}[39m\u{1b}[38;5;10mn\u{1b}[39m\u{1b}[38;5;10md\u{1b}[39m\u{1b}[38;5;10me\u{1b}[39m\u{1b}[38;5;10mp\u{1b}[39m\u{1b}[38;5;10me\u{1b}[39m\u{1b}[38;5;10mn\u{1b}[39m\u{1b}[38;5;10md\u{1b}[39m\u{1b}[38;5;10m\u{1b}[4me\u{1b}[0m\u{1b}[38;5;10mn\u{1b}[39m\u{1b}[38;5;10mt\u{1b}[39m\u{1b}[38;5;10m\u{1b}[4m
\u{1b}[0m\u{1b}[39m purple
\u{1b}[38;5;9m<\u{1b}[39m\u{1b}[38;5;9m\u{1b}[38;5;9mt\u{1b}[39m\u{1b}[38;5;9mh\u{1b}[39m\u{1b}[38;5;9me\u{1b}[39m\u{1b}[39m
\u{1b}[38;5;10m>\u{1b}[39m\u{1b}[38;5;10m\u{1b}[38;5;10mt\u{1b}[39m\u{1b}[38;5;10mh\u{1b}[39m\u{1b}[38;5;10me\u{1b}[39m\u{1b}[38;5;10m\u{1b}[4ms\u{1b}[0m\u{1b}[38;5;10m\u{1b}[4me\u{1b}[0m\u{1b}[39m"
        );
    }
}
